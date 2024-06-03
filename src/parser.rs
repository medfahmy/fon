use crate::lexer::Lexer;
use crate::token::{Token, TokenKind::*};

struct Parser<'a> {
    lexer: Lexer<'a>,
    curr: Option<Token<'a>>,
    peek: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let curr = lexer.next();
        let peek = lexer.next();

        Self { lexer, curr, peek }
    }

    fn read_token(&mut self) {
        self.curr = self.peek.take();
        self.peek = self.lexer.next();
    }
}
