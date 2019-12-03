use std::result;

use super::super::lexer::{LexType, TokType};
use super::{AstNode, Expr};

/// `ParseError` is an enum which represents errors encounted during parsing an expression
#[derive(Debug, Clone)]
pub enum ParseError {
    /// When it expected a certain kind of token, but got another as part of something
    Expected(Vec<TokType>, LexType, &'static str),
    /// When it expected a certain expression, but got another
    ExpectedExpr(&'static str, Expr),
    /// When it didn't expect this keyword
    UnexpectedKeyword(LexType),
    /// When there is an abrupt end to the parsing
    AbruptEnd,
}

pub type Result = std::result::Result<Expr, ParseError>;

#[derive(Debug)]
pub struct Parser {
    /// The tokens being input
    tokens: Vec<LexType>,
    /// The current position within the tokens
    pos: usize,
}

impl Parser {
    /// Create a new parser, using `tokens` as input
    pub fn new(tokens: Vec<LexType>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parse all expressions in the token array
    pub fn parse_all(&mut self) -> Result {
        let mut exprs = Vec::new();
        while self.pos < self.tokens.len() {
            let result = self.parse()?;
            exprs.push(result);
        }

        Ok(Expr::new(AstNode::Block(exprs)))
    }

    fn get_token(&self, pos: usize) -> result::Result<LexType, ParseError> {
        if pos < self.tokens.len() {
            Ok(self.tokens.get(pos).expect("failed getting token").clone())
        } else {
            Err(ParseError::AbruptEnd) // EOF warning with pos?
        }
    }

    pub fn parse(&mut self) -> Result {
        if self.pos > self.tokens.len() {
            return Err(ParseError::AbruptEnd);
        }
        let token = self.get_token(self.pos)?;
        self.pos += 1;
        let expr: Expr = match token.token {
            TokType::Semicolon if self.pos < self.tokens.len() => self.parse()?,
            TokType::IDENTIFIER(s) => match self.get_token(self.pos)?.token {
                TokType::LParen => {
                    self.pos += 1;
                    return self.parse_function(s.clone());
                }
                _ => {
                    return Err(ParseError::UnexpectedKeyword(self.get_token(self.pos)?));
                }
            },
            _ => Expr::new(AstNode::AndOp),
        };
        if self.pos >= self.tokens.len() {
            Ok(expr)
        } else {
            Err(ParseError::AbruptEnd)
        }
    }

    fn parse_function(&mut self, s: String) -> Result {
        let mut arguments = Vec::new();
        loop {
            match self.get_token(self.pos)?.token {
                TokType::RParen => {
                    self.pos += 2;
                    if self.get_token(self.pos - 1)?.token != TokType::Semicolon {
                        return Err(ParseError::Expected(
                            vec![TokType::Semicolon],
                            self.get_token(self.pos - 1)?,
                            "function call",
                        ));
                    }
                    trace!("add function call {}({:?})", s, arguments);
                    return Ok(Expr::new(AstNode::FunctionCall(
                        Box::new(Expr::new(AstNode::SConstant(s.clone()))),
                        arguments,
                    )));
                }
                TokType::IConstant(v) => {
                    arguments.push(Expr::new(AstNode::IConstant(v)));
                    self.pos += 1;
                }
                TokType::FConstant(v) => {
                    arguments.push(Expr::new(AstNode::FConstant(v)));
                    self.pos += 1;
                }
                TokType::IDENTIFIER(s) => {
                    trace!("found override");
                    self.pos += 3;
                    if self.get_token(self.pos - 2)?.token != TokType::Assign {
                        return Err(ParseError::Expected(
                            vec![TokType::Assign],
                            self.get_token(self.pos - 2)?,
                            "override",
                        ));
                    }
                    match self.get_token(self.pos - 1)?.token {
                        TokType::IConstant(v) => {
                            arguments.push(Expr::new(AstNode::Override(
                                Box::new(Expr::new(AstNode::SConstant(s))),
                                Box::new(Expr::new(AstNode::IConstant(v))),
                            )));
                        }
                        TokType::FConstant(v) => {
                            arguments.push(Expr::new(AstNode::Override(
                                Box::new(Expr::new(AstNode::SConstant(s))),
                                Box::new(Expr::new(AstNode::FConstant(v))),
                            )));
                        }
                        _ => {
                            return Err(ParseError::Expected(
                                vec![
                                    TokType::IConstant(0),
                                    TokType::FConstant(0.0),
                                    TokType::IDENTIFIER("".to_string()),
                                ],
                                self.get_token(self.pos - 1)?,
                                "override",
                            ));
                        }
                    }
                }
                TokType::Comma => {
                    self.pos += 1;
                }
                _ => {
                    return Err(ParseError::Expected(
                        vec![
                            TokType::IConstant(0),
                            TokType::FConstant(0.0),
                            TokType::IDENTIFIER("".to_string()),
                            TokType::Comma,
                            TokType::RParen,
                        ],
                        self.get_token(self.pos)?,
                        "function call",
                    ));
                }
            }
        }
    }
}
