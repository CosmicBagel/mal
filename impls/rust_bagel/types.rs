use std::fmt;

#[derive(Debug)]
pub enum MalType {
    Integer(i32),
    Float(f32),
    String(String),
    Symbol(Symbol),
    True,
    False,
    Nil,
    List(Vec<MalType>),
}

impl fmt::Display for MalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MalType::Integer(v) => write!(f, "{}", v),
            MalType::Float(v) => write!(f, "{}", v),
            MalType::String(v) => write!(f, "{}", v),
            MalType::Symbol(v) => write!(f, "{}", v),
            MalType::True => write!(f, "True"),
            MalType::False => write!(f, "False"),
            MalType::Nil => write!(f, "Nil"),
            MalType::List(v) => {
                let mut out = String::new();
                for e in v {
                    out.push_str(&e.to_string());
                }
                write!(f, "{}", out)
            }
        }
    }
}

#[derive(Debug)]
pub enum Symbol {
    Asterisk,
    Plus,
    Minus,
    Divide,
    Equals,
    Ampersand,
    Exclamation,
    Modulus,
    Colon,
    SemiColon,
    DoubleQuote,
    SingleQuote,
    QuestionMark,
    LessThan,
    GreaterThan,
    Comma,
    Period,
    Unknown,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Asterisk => write!(f, "*"),
            Symbol::Plus => write!(f, "+"),
            Symbol::Minus => write!(f, "-"),
            Symbol::Divide => write!(f, "/"),
            Symbol::Equals => write!(f, "="),
            Symbol::Ampersand => write!(f, "&"),
            Symbol::Exclamation => write!(f, "!"),
            Symbol::Modulus => write!(f, "%"),
            Symbol::Colon => write!(f, ":"),
            Symbol::SemiColon => write!(f, ";"),
            Symbol::DoubleQuote => write!(f, r#"""#),
            Symbol::SingleQuote => write!(f, "'"),
            Symbol::QuestionMark => write!(f, "?"),
            Symbol::LessThan => write!(f, "<"),
            Symbol::GreaterThan => write!(f, ">"),
            Symbol::Comma => write!(f, ","),
            Symbol::Period => write!(f, "."),
            Symbol::Unknown => write!(f, "U N K N O W N"),
        }
    }
}

#[derive(Debug)]
pub enum SymbolParseError {
    NotASymbol,
}

impl std::error::Error for SymbolParseError {}

impl fmt::Display for SymbolParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SymbolParseError::NotASymbol => write!(f, "Not a symbol"),
        }
    }
}

pub type AST = Vec<MalType>;
pub type BoxedError = Box<dyn std::error::Error>;
