
#[derive(Debug, PartialEq, Clone)]
pub struct LexType {
    pub token: TokType,
    pub line: usize,
    pub collum: usize,
}

impl LexType {
    pub fn new(token: TokType, line: usize, collum: usize) -> Self {
        Self { token, line, collum}
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
    pub fn lex(input: &str) -> Result<Vec<TokType>, String> {
        let mut result = Vec::new();

        let mut line: usize = 1;
        let mut collum: usize = 1;

        let mut it = input.chars().peekable();

        while let Some(&c) = it.peek() {
            match c {
                '"' => {    // FIXME: EOF
                    collum += 1;;
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
                    result.push(TokType::StringLiteral(s, "fixme".to_string()));
                }
                '\'' => {   // FIXME: EOF
                    it.next();
                    collum += 1;;
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
                        collum += 1;;
                    }
                    trace!("StringLiteral: {} at {}:{}", s, line, collum);
                    result.push(TokType::StringLiteral(s, "fixme".to_string()));
                }
                '0'..='9' => {
                    it.next();
                    collum += 1;;
                    let mut number = c
                        .to_string()
                        .parse::<i64>()
                        .expect("The caller should have passed a digit.");
                    
                    while let Some(Ok(digit)) = it.peek().map(|c| c.to_string().parse::<i64>()) {
                        number = number * 10 + digit;
                        it.next();
                        collum += 1;;
                    }
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '.' => {
                                it.next();
                                collum += 1;;
                                let mut number = number as f64;
                                let mut i = 10;
                                while let Some(Ok(digit)) = it.peek().map(|c| c.to_string().parse::<i64>()) {
                                    //println!("divider is {}, so number is {}, additor is {}", i, number, (digit as f64 / f64::from(i)));
                                    number += digit as f64 / f64::from(i);
                                    //println!("divider is {}, so number is {}, additor is {}", i, number, (digit as f64 / f64::from(i)));
                                    i *= 10;
                                    it.next();
                                    collum += 1;;
                                }
                                warn!("FConstants are still experimental: got floating constant {}", number);
                                result.push(TokType::FConstant(number));
                            }
                            _ => {
                                trace!("IConstant {}", number);
                                result.push(TokType::IConstant(number));
                            }
                        }
                        _ => {
                            trace!("IConstant {}", number);
                            result.push(TokType::IConstant(number));
                        }
                    }
                }
                'a'..='z' | 'A'..='Z' | '$' | '_' => {
                    it.next();
                    collum += 1;;
                    let mut s = String::new();
                    s.push(c);
                    while let Some(&tmp) = it.peek() {
                        match tmp {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                                s.push(tmp);
                                it.next();
                                collum += 1;;
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    trace!("got identifier {}", s);
                    match s.as_ref() {
                        "module" => result.push(TokType::Module),
                        "if" => result.push(TokType::IF),
                        "else" => result.push(TokType::ELSE),
                        _ => result.push(TokType::IDENTIFIER(s)),
                    }
                }
                '(' => {
                    it.next();
                    collum += 1;;
                    result.push(TokType::LParen);
                }
                ')' => {
                    it.next();
                    collum += 1;;
                    result.push(TokType::RParen);
                }
                '{' => {
                    it.next();
                    collum += 1;;
                    result.push(TokType::LBrace);
                }
                '}' => {
                    it.next();
                    collum += 1;;();
                    result.push(TokType::RBrace);
                }
                '[' => {
                    it.next();
                    collum += 1;;
                    result.push(TokType::LBracket);
                }
                ']' => {
                    it.next();
                    collum += 1;;
                    result.push(TokType::RBracket);
                }
                ';' => {
                    it.next();
                    collum += 1;;
                    result.push(TokType::Semicolon);
                }
                '=' => {
                    it.next();
                    collum += 1;;
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(TokType::EqOp);
                            }
                            _ => { result.push(TokType::Assign); },
                        },
                        _ => { result.push(TokType::Assign); },
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
                                result.push(TokType::LeOp);
                            }
                            '<' => {
                                it.next();
                                collum += 1;;
                                result.push(TokType::LeftOp);
                            }
                            _ => {
                                result.push(TokType::Lt);
                            }
                        },
                        _ => {
                            result.push(TokType::Lt);
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
                                result.push(TokType::GeOp);
                            },
                            '>' => {
                                it.next();
                                collum += 1;;
                                result.push(TokType::RightOp);
                            },
                            _ => {
                                result.push(TokType::Gt);
                            }
                        }
                        _ => {
                            result.push(TokType::Gt);
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
                                result.push(TokType::DecOp);
                            }
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(TokType::SubAssign);
                            }
                            _ => {
                                result.push(TokType::Minus);
                            }
                        }
                        _ => {
                            result.push(TokType::Minus);
                        }
                    }
                }
                '~' => {
                    it.next();
                    collum += 1;;
                    result.push(TokType::Tilde);
                }
                '!' => {
                    it.next();
                    collum += 1;;
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(TokType::NeOp);
                            }
                            _ => {
                                result.push(TokType::Exclamation);
                            }
                        }
                        _ => {
                            result.push(TokType::Exclamation);
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
                                result.push(TokType::IncOp);
                            }
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(TokType::AddAssign);
                            }
                            _ => {
                                result.push(TokType::Plus);
                            }
                        }
                        _ => {
                            result.push(TokType::Plus);
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
                                result.push(TokType::MulAssign);
                            }
                            _ => {
                                result.push(TokType::Multi);
                            }
                        }
                        _ => {
                            result.push(TokType::Multi);
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
                                result.push(TokType::ModAssign);
                            }
                            _ => {
                                result.push(TokType::Mod);
                            }
                        }
                        _ => {
                            result.push(TokType::Mod);
                        }
                    }
                }
                '/' => {
                    it.next();
                    collum += 1;;
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(TokType::DivAssign);
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
                                    collum += 1;;
                                }
                            }
                            '*' => {
                                trace!("got comment");
                                it.next();
                                collum += 1;;
                                while let Some(&c) = it.peek() {    // FIXME: not ending?
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
                                            collum += 1;;
                                        }
                                    }
                                    if c == '\n' {
                                        collum = 1;
                                        line += 1;
                                    }
                                    it.next();
                                    collum += 1;;
                                }
                            }
                            _ => {
                                result.push(TokType::Splash);
                            }
                        }
                        _ => {
                            result.push(TokType::Splash);
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
                                result.push(TokType::AndOp);
                            }
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(TokType::AddAssign);
                            }
                            _ => {
                                result.push(TokType::SingleAnd);
                            }
                        }
                        _ => {
                            result.push(TokType::SingleAnd);
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
                                result.push(TokType::OrOp);
                            }
                            '=' => {
                                it.next();
                                collum += 1;;
                                result.push(TokType::OrAssign);
                            }
                            _ => {
                                result.push(TokType::InclusiveOr);
                            }
                        }
                        _ => {
                            result.push(TokType::InclusiveOr);
                        }
                    }
                },
                '?' => {
                    it.next();
                    collum += 1;;
                    result.push(TokType::QuestionMark);
                }
                ':' => {
                    it.next();
                    collum += 1;;
                    result.push(TokType::Colon);
                }
                ',' => {
                    it.next();
                    collum += 1;;
                    result.push(TokType::Colon);
                }
                '#' => {
                    it.next();
                    collum += 1;;
                    result.push(TokType::Highlight);
                }
                '.' => {
                    it.next();
                    collum += 1;;
                    result.push(TokType::Dot);
                }
                ' ' | '\t' | '\r' => {
                    //skip
                    it.next();
                    collum += 1;;
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
        Ok(result)
    }
}
