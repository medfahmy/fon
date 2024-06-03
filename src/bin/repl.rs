use fon::lexer::Lexer;
use fon::token::Token;

fn main() {
    let input = "1;";
    let mut lexer = Lexer::new(input);

    while let Some(next) = lexer.next() {
        println!("{}", next);
    }
}
