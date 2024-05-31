use fon::token::Token;
use fon::lexer::Lexer;

fn main() {
    let input = "1;";
    let mut lexer = Lexer::new(input);
    let mut next =  lexer.next();

    while next.is_some() {
        println!("{:?}", next);
        next = lexer.next();
    }
}
