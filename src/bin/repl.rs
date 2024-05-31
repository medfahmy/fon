use fon::token::Token;
use fon::lexer::Lexer;

fn main() {
    let input = "1;";
    let mut lexer = Lexer::new(input);

    while let Some(next) = lexer.next() {
        println!("{}", next);
    }
}
