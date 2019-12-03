#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Clone)]
pub struct LexType {
    pub token: TokType,
    pub line: usize,
    pub collum: usize,
}

impl LexType {
    pub fn new(token: TokType, line: usize, collum: usize) -> Self {
        Self {
            token,
            line,
            collum,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum TokType {
    LBrace,       // {
    RBrace,       // }
    LParen,       // (
    RParen,       // )
    LBracket,     // [
    RBracket,     // ]
    Semicolon,    // ;
    Assign,       // =
    Lt,           // <
    Gt,           // >
    Minus,        // -
    Tilde,        // ~
    Exclamation,  // !
    Plus,         // +
    Multi,        // *
    Splash,       // /
    Colon,        // :
    QuestionMark, // ?
    Comma,        // ,
    Dot,          // .
    SingleAnd,    // &
    InclusiveOr,  // |
    ExclusiveOr,  // ^
    Mod,          // %
    Highlight,    // #
    IDENTIFIER(String),
    IConstant(i64),
    FConstant(f64),
    StringLiteral(String, String),
    Module,

    // FuncName,    // __func__
    // SIZEOF,      // sizeof
    // PtrOp,       // ->
    IncOp,       // ++
    DecOp,       // --
    LeftOp,      // <<
    RightOp,     // >>
    LeOp,        // <=
    GeOp,        // >=
    EqOp,        // ==
    NeOp,        // !=
    AndOp,       // &&
    OrOp,        // ||
    MulAssign,   // *=
    DivAssign,   // /=
    ModAssign,   // %=
    AddAssign,   // +=
    SubAssign,   // -=
    LeftAssign,  // <<=
    RightAssign, // >>=
    AndAssign,   // &=
    XorAssign,   // ^=
    OrAssign,    // |=
    EOF,
    /* // TODO: this should be done when we found this is a typedef name,
    //       typedef LL int, then LL is typedef_name
    TypedefName,
    ELLIPSIS,                    // ...
    EnumerationConstant(String), // TODO: add check
    TYPEDEF,
    EXTERN,
    STATIC,
    AUTO,
    REGISTER,
    INLINE,
    CONST,
    RESTRICT,
    VOLATILE,
    BOOL,
    CHAR,
    SHORT,
    INT,
    LONG,
    SIGNED,
    UNSIGNED,
    FLOAT,
    DOUBLE,
    VOID,
    COMPLEX,
    IMAGINARY,
    STRUCT,
    UNION,
    ENUM,
    CASE,
    DEFAULT, */
    IF,
    ELSE,
    /* SWITCH,
    WHILE,
    DO,
    FOR,
    GOTO,
    CONTINUE,
    BREAK,
    RETURN,
    ALIGNAS,
    ALIGNOF,
    ATOMIC,
    GENERIC,
    NORETURN,
    StaticAssert,
    ThreadLocal, */
}

impl TokType {
    #[allow(clippy::cognitive_complexity)]
    pub fn lex(input: &str) -> Result<Vec<LexType>, String> {
        let mut result = Vec::new();

        let mut line: usize = 1;
        let mut collum: usize = 1;

        let mut it = input.chars().peekable();

        while let Some(&c) = it.peek() {
            match c {
                '"' => {
                    // FIXME: EOF
                    collum += 1;
                    let start = collum;
                    let start_line = line;
                    it.next();
                    let mut s = "".to_string();
                    while let Some(&c) = it.peek() {
                        if c == '"' {
                            collum += 1;;
                            it.next();
                            break;
                        }
                        if c == '\n' {
                            collum = 1;
                            line += 1;
                        }
                        s.push(c);
                        collum += 1;;
                        it.next();
                    }
                    trace!("StringLiteral: {}", s);
                    result.push(LexType::new(
                        TokType::StringLiteral(s, "fixme".to_string()),
                        start_line,
                        start,
                    ));
                }
                '\'' => {
                    // FIXME: EOF
                    it.next();
                    collum += 1;
                    let start = collum;
                    let start_line = line;
                    let mut s = String::new();
                    while let Some(&c) = it.peek() {
                        if c == '\'' {
                            it.next();
                            collum += 1;;
                            break;
                        }
                        if c == '\n' {
                            collum = 1;
                            line += 1;
                        }
                        s.push(c);
                        it.next();
                        collum += 1;
                    }
                    trace!("StringLiteral: {} at {}:{}", s, line, collum);
                    result.push(LexType::new(
                        TokType::StringLiteral(s, "fixme".to_string()),
                        start_line,
                        start,
                    ));
                }
                '0'..='9' => {
                    it.next();
                    collum += 1;
                    let start = collum;
                    let mut number = c
                        .to_string()
                        .parse::<i64>()
                        .expect("The caller should have passed a digit.");

                    while let Some(Ok(digit)) = it.peek().map(|c| c.to_string().parse::<i64>()) {
                        number = number * 10 + digit;
                        it.next();
                        collum += 1;
                    }
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '.' => {
                                it.next();
                                collum += 1;;
                                let mut number = number as f64;
                                let mut i = 10;
                                while let Some(Ok(digit)) =
                                    it.peek().map(|c| c.to_string().parse::<i64>())
                                {
                                    //println!("divider is {}, so number is {}, additor is {}", i, number, (digit as f64 / f64::from(i)));
                                    number += digit as f64 / f64::from(i);
                                    //println!("divider is {}, so number is {}, additor is {}", i, number, (digit as f64 / f64::from(i)));
                                    i *= 10;
                                    it.next();
                                    collum += 1;
                                }
                                warn!(
                                    "FConstants are still experimental: got floating constant {}",
                                    number
                                );
                                result.push(LexType::new(TokType::FConstant(number), line, start));
                            }
                            _ => {
                                trace!("IConstant {}", number);
                                result.push(LexType::new(TokType::IConstant(number), line, start));
                            }
                        },
                        _ => {
                            trace!("IConstant {}", number);
                            result.push(LexType::new(TokType::IConstant(number), line, start));
                        }
                    }
                }
                'a'..='z' | 'A'..='Z' | '$' | '_' => {
                    it.next();
                    let start = collum;
                    collum += 1;
                    let mut s = String::new();
                    s.push(c);
                    while let Some(&tmp) = it.peek() {
                        match tmp {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                                s.push(tmp);
                                it.next();
                                collum += 1;
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    trace!("got identifier {}", s);
                    match s.as_ref() {
                        "module" => result.push(LexType::new(TokType::Module, line, start)),
                        "if" => result.push(LexType::new(TokType::IF, line, start)),
                        "else" => result.push(LexType::new(TokType::ELSE, line, start)),
                        _ => result.push(LexType::new(TokType::IDENTIFIER(s), line, start)),
                    }
                }
                '(' => {
                    it.next();
                    collum += 1;
                    result.push(LexType::new(TokType::LParen, line, collum));
                }
                ')' => {
                    it.next();
                    collum += 1;;
                    result.push(LexType::new(TokType::RParen, line, collum));
                }
                '{' => {
                    it.next();
                    collum += 1;;
                    result.push(LexType::new(TokType::LBrace, line, collum));
                }
                '}' => {
                    it.next();
                    collum += 1;
                    result.push(LexType::new(TokType::RBrace, line, collum));
                }
                '[' => {
                    it.next();
                    collum += 1;;
                    result.push(LexType::new(TokType::LBracket, line, collum));
                }
                ']' => {
                    it.next();
                    collum += 1;;
                    result.push(LexType::new(TokType::RBracket, line, collum));
                }
                ';' => {
                    it.next();
                    collum += 1;;
                    result.push(LexType::new(TokType::Semicolon, line, collum));
                }
                '=' => {
                    it.next();
                    collum += 1;;
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::EqOp, line, collum));
                            }
                            _ => {
                                result.push(LexType::new(TokType::Assign, line, collum));
                            }
                        },
                        _ => {
                            result.push(LexType::new(TokType::Assign, line, collum));
                        }
                    }
                }
                '<' => {
                    it.next();
                    collum += 1;;
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::LeOp, line, collum));
                            }
                            '<' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::LeftOp, line, collum));
                            }
                            _ => {
                                result.push(LexType::new(TokType::Lt, line, collum));
                            }
                        },
                        _ => {
                            result.push(LexType::new(TokType::Lt, line, collum));
                        }
                    }
                }
                '>' => {
                    it.next();
                    collum += 1;;
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::GeOp, line, collum));
                            }
                            '>' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::RightOp, line, collum));
                            }
                            _ => {
                                result.push(LexType::new(TokType::Gt, line, collum));
                            }
                        },
                        _ => {
                            result.push(LexType::new(TokType::Gt, line, collum));
                        }
                    }
                }
                '-' => {
                    it.next();
                    collum += 1;;
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '-' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::DecOp, line, collum));
                            }
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::SubAssign, line, collum));
                            }
                            _ => {
                                result.push(LexType::new(TokType::Minus, line, collum));
                            }
                        },
                        _ => {
                            result.push(LexType::new(TokType::Minus, line, collum));
                        }
                    }
                }
                '~' => {
                    it.next();
                    collum += 1;;
                    result.push(LexType::new(TokType::Tilde, line, collum));
                }
                '!' => {
                    it.next();
                    collum += 1;;
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::NeOp, line, collum));
                            }
                            _ => {
                                result.push(LexType::new(TokType::Exclamation, line, collum));
                            }
                        },
                        _ => {
                            result.push(LexType::new(TokType::Exclamation, line, collum));
                        }
                    }
                }
                '+' => {
                    it.next();
                    collum += 1;;
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '+' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::IncOp, line, collum));
                            }
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::AddAssign, line, collum));
                            }
                            _ => {
                                result.push(LexType::new(TokType::Plus, line, collum));
                            }
                        },
                        _ => {
                            result.push(LexType::new(TokType::Plus, line, collum));
                        }
                    }
                }
                '*' => {
                    it.next();
                    collum += 1;;
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::MulAssign, line, collum));
                            }
                            _ => {
                                result.push(LexType::new(TokType::Multi, line, collum));
                            }
                        },
                        _ => {
                            result.push(LexType::new(TokType::Multi, line, collum));
                        }
                    }
                }
                '%' => {
                    it.next();
                    collum += 1;;
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::ModAssign, line, collum));
                            }
                            _ => {
                                result.push(LexType::new(TokType::Mod, line, collum));
                            }
                        },
                        _ => {
                            result.push(LexType::new(TokType::Mod, line, collum));
                        }
                    }
                }
                '/' => {
                    it.next();
                    collum += 1;
                    let start = collum;
                    let _start_line = line;
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::DivAssign, line, start));
                            }
                            '/' => {
                                trace!("got comment");
                                it.next();
                                collum += 1;;
                                while let Some(&c) = it.peek() {
                                    if c == '\n' {
                                        it.next();
                                        collum = 1;
                                        line += 1;
                                        break;
                                    }
                                    it.next();
                                    collum += 1;
                                }
                            }
                            '*' => {
                                trace!("got comment");
                                it.next();
                                collum += 1;;
                                while let Some(&c) = it.peek() {
                                    // FIXME: not ending?
                                    if c == '*' {
                                        it.next();
                                        collum += 1;;
                                        if let Some(&c) = it.peek() {
                                            if c == '/' {
                                                it.next();
                                                collum += 1;;
                                                break;
                                            }
                                            it.next();
                                            collum += 1;
                                        }
                                    }
                                    if c == '\n' {
                                        collum = 1;
                                        line += 1;
                                    }
                                    it.next();
                                    collum += 1;
                                }
                            }
                            _ => {
                                result.push(LexType::new(TokType::Splash, line, collum));
                            }
                        },
                        _ => {
                            result.push(LexType::new(TokType::Splash, line, collum));
                        }
                    }
                }
                '&' => {
                    it.next();
                    collum += 1;;
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '&' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::AndOp, line, collum));
                            }
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::AndAssign, line, collum));
                            }
                            _ => {
                                result.push(LexType::new(TokType::SingleAnd, line, collum));
                            }
                        },
                        _ => {
                            result.push(LexType::new(TokType::SingleAnd, line, collum));
                        }
                    }
                }
                '|' => {
                    it.next();
                    collum += 1;;
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '|' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::OrOp, line, collum));
                            }
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(LexType::new(TokType::OrAssign, line, collum));
                            }
                            _ => {
                                result.push(LexType::new(TokType::InclusiveOr, line, collum));
                            }
                        },
                        _ => {
                            result.push(LexType::new(TokType::InclusiveOr, line, collum));
                        }
                    }
                }
                '?' => {
                    it.next();
                    collum += 1;;
                    result.push(LexType::new(TokType::QuestionMark, line, collum));
                }
                ':' => {
                    it.next();
                    collum += 1;;
                    result.push(LexType::new(TokType::Colon, line, collum));
                }
                ',' => {
                    it.next();
                    collum += 1;;
                    result.push(LexType::new(TokType::Comma, line, collum));
                }
                '#' => {
                    it.next();
                    collum += 1;;
                    result.push(LexType::new(TokType::Highlight, line, collum));
                }
                '.' => {
                    it.next();
                    collum += 1;;
                    result.push(LexType::new(TokType::Dot, line, collum));
                }
                ' ' | '\t' | '\r' => {
                    //skip
                    it.next();
                    collum += 1;
                }
                '\n' => {
                    it.next();
                    collum = 1;
                    line += 1;
                }
                _ => {
                    //error!("unexpected Character {}", c);
                    //it.next();
                    return Err(format!("unexpected Character {} at {}:{}", c, line, collum));
                }
            }
        }
        result.push(LexType::new(TokType::EOF, line, collum));
        Ok(result)
    }
}
