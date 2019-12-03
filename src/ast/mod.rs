use std::fmt::{self, Display, Formatter};

use super::lexer::{LexType, TokType};

pub mod parser;

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    /// The expression definition
    pub def: AstNode,
}

impl Expr {
    /// Create a new expression with a starting and ending position
    pub fn new(def: AstNode) -> Self {
        Self { def }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.def) //FIXME: implement display
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
    Block(Vec<Expr>),
    FunctionCall(Box<Expr>, Vec<Expr>),
    Condition(Box<Condition>),

    // Constants
    IConstant(i64),
    FConstant(f64),
    SConstant(String),

    Override(Box<Expr>, Box<Expr>),

    // OPs
    LeOp,  // <=
    GeOp,  // >=
    EqOp,  // ==
    NeOp,  // !=
    AndOp, // &&
    OrOp,  // ||

    True,
    False,
}

impl AstNode {
    /*pub fn parse(lex: &[LexType]) -> Result<Self, String> {
        let mut result = Vec::new();

        let mut it = lex.iter().peekable();

        while let Some(lex) = it.peek() {
            match &lex.token {
                TokType::IDENTIFIER(s) => {
                    trace!("found function call: {}", s);
                    it.next();
                    if let Some(l) = it.peek() {
                        if l.token == TokType::LParen {
                            it.next();
                            while let Some(&c) = it.peek() {
                                if c.token == TokType::RParen {
                                    it.next();
                                    if let Some(c) = it.next() {
                                        if c.token != TokType::Semicolon {
                                            return Err(format!("expected semicolon at {}:{}", c.line, c.collum));
                                        }
                                    }
                                    break;
                                }

                            }
                        }
                    }
                }
                _ => {
                    return Err(format!("not expecterd {:?} at {}:{}", lex.token, lex.line, lex.collum));
                }
            }
        }

        Ok(AstNode::CodeBlock(result))
    }*/
}

#[derive(Debug, PartialEq, Clone)]
pub struct Condition {
    pub lhs: AstNode,
    pub rhs: AstNode,
    pub op: AstNode,
}

impl Condition {
    pub fn new_lhs(lhs: AstNode) -> Self {
        Self {
            lhs,
            rhs: AstNode::True,
            op: AstNode::EqOp,
        }
    }
}
