use std::iter::Peekable;

use crate::types::*;

const REGEX_PATTERN: &str =
    r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#;

lazy_static! {
    static ref TOKEN_REGEX: regex::Regex = regex::Regex::new(REGEX_PATTERN).unwrap();
}

type Tokens<'a> = Peekable<regex::CaptureMatches<'a, 'a>>;

pub fn read_str(input_str: &str) -> Result<AST, regex::Error> {
    /*
       call tokenize and then create a new Reader object instance with the tokens. Then it
       will call read_form with the Reader instance.
    */
    let mut tokens = tokenize(input_str);
    let mal_type = read_from(&mut tokens);
    println!("{}", mal_type);
    Ok(Vec::new())
}

pub fn tokenize(input_str: &str) -> Tokens {
    TOKEN_REGEX.captures_iter(input_str).peekable()
}

pub fn read_from(mut tokens: &mut Tokens) -> MalType {
    /*
       peek at the first token in the Reader object and switch on the first character of that
       token. If the character is a left paren then read_list is called with the Reader object.
       Otherwise, read_atom is called with the Reader Object. The return value from read_form is a
       mal data type. If your target language is statically typed then you will need some way for
       read_form to return a variant or subclass type. For example, if your language is object
       oriented, then you can define a top level MalType (in types.qx) that all your mal data
       types inherit from. The MalList type (which also inherits from MalType) will contain a
       list/array of other MalTypes. If your language is dynamically typed then you can likely
       just return a plain list/array of other mal types.
    */
    if let Some(t) = tokens.peek() {
        match &t[0] {
            "(" => read_list(&mut tokens),
            _ => read_atom(&mut tokens),
        }
    } else {
        MalType::Nil
    }
}

pub fn read_list(mut tokens: &mut Tokens) -> MalType {
    /*
        repeatedly call read_form with the Reader object until it encounters a ')' token
       (if it reach EOF before reading a ')' then that is an error). It accumulates the
       results into a List type. If your language does not have a sequential data type that
       can hold mal type values you may need to implement one (in types.qx). Note that
       read_list repeatedly calls read_form rather than read_atom. This mutually recursive
       definition between read_list and read_form is what allows lists to contain lists.
    */
    let mut mal_list = Vec::new();
    tokens.next(); // skip the (
    while let Some(cap) = tokens.peek() {
        let t = &cap[1];
        if t == ")" {
            break;
        }

        mal_list.push(read_from(tokens));
    }
    MalType::List(mal_list)
}

pub fn read_atom(tokens: &mut Tokens) -> MalType {
    /*
        look at the contents of the token and return the appropriate scalar (simple/single)
        data type value. Initially, you can just implement numbers (integers) and symbols.
        This will allow you to proceed through the next couple of steps before you will need
        to implement the other fundamental mal types: nil, true, false, and string. The
        remaining scalar mal type, keyword does not need to be implemented until step A
        (but can be implemented at any point between this step and that). BTW, symbols types
        are just an object that contains a single string name value (some languages have
        symbol types already).
    */
    if let Some(cap) = tokens.next() {
        let t = &cap[1];
        if let Ok(int) = t.parse() {
            MalType::Integer(int)
        } else if let Ok(float) = t.parse() {
            MalType::Float(float)
        } else if let Ok(sym) = parse_symbol(t) {
            sym
        } else if t.to_lowercase() == "true" {
            MalType::True
        } else if t.to_lowercase() == "false" {
            MalType::False
        } else {
            MalType::String(t.to_string())
        }
    } else {
        MalType::Nil
    }
}

fn parse_symbol(token_str: &str) -> Result<MalType, SymbolParseError> {
    match token_str {
        "*" => Ok(MalType::Symbol(Symbol::Asterisk)),
        "-" => Ok(MalType::Symbol(Symbol::Minus)),
        "+" => Ok(MalType::Symbol(Symbol::Plus)),
        "/" => Ok(MalType::Symbol(Symbol::Divide)),
        "=" => Ok(MalType::Symbol(Symbol::Equals)),
        "!" => Ok(MalType::Symbol(Symbol::Exclamation)),
        "&" => Ok(MalType::Symbol(Symbol::Ampersand)),
        "%" => Ok(MalType::Symbol(Symbol::Modulus)),
        ":" => Ok(MalType::Symbol(Symbol::Colon)),
        ";" => Ok(MalType::Symbol(Symbol::SemiColon)),
        r#"""# => Ok(MalType::Symbol(Symbol::DoubleQuote)),
        "'" => Ok(MalType::Symbol(Symbol::SingleQuote)),
        "?" => Ok(MalType::Symbol(Symbol::QuestionMark)),
        "<" => Ok(MalType::Symbol(Symbol::LessThan)),
        ">" => Ok(MalType::Symbol(Symbol::GreaterThan)),
        "," => Ok(MalType::Symbol(Symbol::Comma)),
        "." => Ok(MalType::Symbol(Symbol::Period)),
        _ => Err(SymbolParseError::NotASymbol),
    }
}
