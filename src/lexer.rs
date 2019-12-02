
#[derive(Debug, PartialEq, Clone)]
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
    pub fn lex(input: &str) -> Result<Vec<TokType>, String> {
        let mut result = Vec::new();

        let mut it = input.chars().peekable();

        while let Some(&c) = it.peek() {
            match c {
                // TODO: String literal?
                '"' => {
                    it.next();
                    let mut s = "".to_string();
                    while let Some(&c) = it.peek() {
                        if c == '"' {
                            it.next();
                            break;
                        }
                        s.push(c);
                        it.next();
                    }
                    result.push(TokType::StringLiteral(s, "fixme".to_string()));
                }
                '0'...'9' => {  // FIXME: floating point?
                    it.next();
                    let mut number = c
                        .to_string()
                        .parse::<i64>()
                        .expect("The caller should have passed a digit.");
                    
                    while let Some(Ok(digit)) = it.peek().map(|c| c.to_string().parse::<i64>()) {
                        number = number * 10 + digit;
                        it.next();
                    }
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '.' => {
                                it.next();
                                let mut number = number as f64;
                                let mut i = 10;
                                while let Some(Ok(digit)) = it.peek().map(|c| c.to_string().parse::<i64>()) {
                                    println!("divider is {}, so number is {}, additor is {}", i, number, (digit as f64 / i as f64));
                                    number = number + (digit as f64 / i as f64);
                                    println!("divider is {}, so number is {}, additor is {}", i, number, (digit as f64 / i as f64));
                                    i = i * 10;
                                    it.next();
                                }
                                println!("number is {}", number);
                                result.push(TokType::FConstant(number));
                            }
                            _ => {
                                result.push(TokType::IConstant(number));
                            }
                        }
                        _ => {
                            result.push(TokType::IConstant(number));
                        }
                    }
                }
                'a'...'z' | 'A'...'Z' | '_' => {
                    it.next();
                    let mut s = String::new();
                    s.push(c);
                    while let Some(&tmp) = it.peek() {
                        match tmp {
                            'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => {
                                s.push(tmp);
                                it.next();
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    match s.as_ref() {
                        "module" => result.push(TokType::Module),
                        "if" => result.push(TokType::IF),
                        "else" => result.push(TokType::ELSE),
                        _ => result.push(TokType::IDENTIFIER(s)),
                    }
                }
                '(' => {
                    it.next();
                    result.push(TokType::LParen);
                }
                ')' => {
                    it.next();
                    result.push(TokType::RParen);
                }
                '{' => {
                    it.next();
                    result.push(TokType::LBrace);
                }
                '}' => {
                    it.next();
                    result.push(TokType::RBrace);
                }
                '[' => {
                    it.next();
                    result.push(TokType::LBracket);
                }
                ']' => {
                    it.next();
                    result.push(TokType::RBracket);
                }
                ';' => {
                    it.next();
                    result.push(TokType::Semicolon);
                }
                '=' => {
                    it.next();
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
                                result.push(TokType::EqOp);
                            }
                            _ => {result.push(TokType::Assign);},
                        },
                        _ => return Err(format!("can not peek next char")),
                    }
                }
                '<' => {
                    it.next();
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
                                result.push(TokType::LeOp);
                            }
                            '<' => {
                                it.next();
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
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
                                result.push(TokType::GeOp);
                            },
                            '>' => {
                                it.next();
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
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '-' => {
                                it.next();
                                result.push(TokType::DecOp);
                            }
                            '=' => {
                                it.next();
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
                    result.push(TokType::Tilde);
                }
                '!' => {
                    it.next();
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
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
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '+' => {
                                it.next();
                                result.push(TokType::IncOp);
                            }
                            '=' => {
                                it.next();
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
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
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
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
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
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '=' => {
                                it.next();
                                result.push(TokType::DivAssign);
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
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '&' => {
                                it.next();
                                result.push(TokType::AndOp);
                            }
                            '=' => {
                                it.next();
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
                    match it.peek() {
                        Some(tmp) => match tmp {
                            '|' => {
                                it.next();
                                result.push(TokType::OrOp);
                            }
                            '=' => {
                                it.next();
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
                    result.push(TokType::QuestionMark);
                }
                ':' => {
                    it.next();
                    result.push(TokType::Colon);
                }
                ',' => {
                    it.next();
                    result.push(TokType::Colon);
                }
                '#' => {
                    it.next();
                    result.push(TokType::Highlight);
                }
                '.' => {
                    it.next();
                    result.push(TokType::Dot);
                }
                ' ' | '\n' | '\t' | '\r' => {
                    //skip
                    it.next();
                }
                _ => {
                    eprintln!("unexpected Character {}", c);
                    it.next();
                }
            }
        }
        Ok(result)
    }
}
