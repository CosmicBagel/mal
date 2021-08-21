mod printer;
mod reader;
mod types;

use printer::*;
use reader::*;
use std::io::{self, stdin, Write};
use types::AST;

fn main() -> io::Result<()> {
    let mut buf = String::new();
    loop {
        prompt()?;

        let ast = read(&mut buf)?;
        //READ - EVAL- PRINT
        let ast = eval(ast)?;
        print(ast)?;
        buf.clear();
    }
}

fn prompt() -> Result<(), io::Error> {
    let prompt = "user> ";
    io::stdout().write_all(prompt.as_bytes())?;
    io::stdout().flush()?;
    Ok(())
}

fn read(buf: &mut String) -> Result<AST, io::Error> {
    stdin().read_line(buf)?;
    //abstract syntax tree time
    let ast = reader::read_str(buf);
    Ok(ast)
}

fn eval(ast: AST) -> Result<AST, io::Error> {
    Ok(ast)
}

fn print(ast: AST) -> Result<(), io::Error> {
    pr_str(ast);
    // io::stdout().write_all(buf.as_bytes())?;
    // io::stdout().flush()?;
    Ok(())
}
