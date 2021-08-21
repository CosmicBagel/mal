use std::io::{self, stdin, Write};

fn main() -> io::Result<()> {
    let mut buf = String::new();
    loop {
        let prompt = "user> ";
        io::stdout().write_all(prompt.as_bytes())?;
        io::stdout().flush()?;

        stdin().read_line(&mut buf)?;
        //READ - EVAL- PRINT
        io::stdout().write_all(buf.as_bytes())?;
        io::stdout().flush()?;
        buf.clear();
    }
}
