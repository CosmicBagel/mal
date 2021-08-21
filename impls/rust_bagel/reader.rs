use std::{iter::Peekable, vec::IntoIter};

use crate::types::*;

type Tokens<'a> = Peekable<IntoIter<&'a str>>;

pub fn read_str(input_str: &str) -> AST {
    /*
       call tokenize and then create a new Reader object instance with the tokens. Then it
       will call read_form with the Reader instance.
    */
    let mut tokens = tokenize(input_str);
    let i = tokens.peek();
    read_from();
    Vec::new()
}

pub fn tokenize(input_str: &str) -> Tokens {
    /*
        take a single string and return an array/list of all the tokens (strings) in it.
        The following regular expression (PCRE) will match all mal tokens.
    */
    let mut tokens = Vec::<&str>::new();

    tokens.push("hi");

    tokens.into_iter().peekable()
}

pub fn read_from() -> Vec<MalType> {
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
    Vec::new()
}

pub fn read_list() -> Vec<MalType> {
    /*
        repeatedly call read_form with the Reader object until it encounters a ')' token
       (if it reach EOF before reading a ')' then that is an error). It accumulates the
       results into a List type. If your language does not have a sequential data type that
       can hold mal type values you may need to implement one (in types.qx). Note that
       read_list repeatedly calls read_form rather than read_atom. This mutually recursive
       definition between read_list and read_form is what allows lists to contain lists.
    */
    Vec::new()
}

pub fn read_atom() -> MalType {
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
    MalType::Number(0)
}
