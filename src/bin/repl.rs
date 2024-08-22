use std::io::{stdin, stdout, Write, Result};
use tau::lexer::Lexer;

fn main() -> Result<()> {
    let stdin = stdin();
    let mut stdout = stdout();
    let mut buf = String::new();

    loop {
        print!(">> ");
        stdout.flush()?;
        buf.clear();
        stdin.read_line(&mut buf)?;

        let lexer = Lexer::new(&buf);

        let tokens: Vec<_> = lexer.collect();
        for token in tokens {
            println!("{}", token);
        }

        println!();
    }
}
