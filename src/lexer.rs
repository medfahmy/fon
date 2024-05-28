use crate::token::{Token, TokenName};

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn curr(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn read(&mut self) {
        if self.curr().is_some() {
            self.pos += 1;
        }
    }

    fn read_symbol() -> Option<Token<'a>> {
        todo!()
    }

    fn read_num(&mut self) -> Option<Token<'a>> {
        todo!()
    }

    fn read_str(&mut self) -> Option<Token<'a>> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use TokenName::*;

    // let x: Int = 5;
    // let y: Float = 5.5;
    // let c: Char = '[';
    // let s: Str = "hello";
    // let add: Fn(

    fn lex(input: &str) -> Vec<Token> {
        let lexer = Lexer::new(input);
        lexer.collect()
    }

    fn basics() {
        let tokens = lex("{}[]()<>;.:=+-*/");
        assert_eq!(
            tokens.into_iter().map(|token| token.name).collect::<Vec<_>>(),
            &[Osq, Csq, Obr, Cbr, Opr, Cpr, Otg, Ctg, Semi, Dot, Colon, Assign, Plus, Minus, Asterisk, Slash], 
        );
    }
}
