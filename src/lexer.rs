use crate::token::{Token, TokenName};

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    row: usize,
    col: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        
        self.curr().map(|curr| {
            match curr {
                c if c.is_ascii_digit() => self.read_num(),
                _ => self.read_symbol(),
            }
        })?
    }
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self { 
            input, 
            pos: 0,
            row: 1,
            col: 1,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(curr) = self.curr() {
            if curr.is_whitespace() {
                self.read();
            } else {
                break;
            }
        }
    }

    fn read(&mut self) {
        if let Some(curr) = self.curr() {
            self.pos += 1;

            if curr == '\n' {
                self.row += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
        }
    }

    fn curr(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos + 1)
    }

    fn read_symbol(&mut self) -> Option<Token<'a>> {
        let name = self.curr().map(|curr| {
            match curr {
                '{' => TokenName::Osq,
                '}' => TokenName::Csq,
                '[' => TokenName::Obr,
                ']' => TokenName::Cbr,
                '(' => TokenName::Opr,
                ')' => TokenName::Cpr,
                ':' => TokenName::Colon,
                ';' => TokenName::Semi,
                _ => unreachable!(),
            }
        })?;

        let token = Token {
            name,
            value: &self.input[self.pos..self.pos + 1],
            row: self.row,
            col: self.col,
        };

        self.read();

        Some(token)
    }

    fn read_op(&mut self) -> Option<Token<'a>> {
        todo!()
    }

    fn read_num(&mut self) -> Option<Token<'a>> {
        let pos = self.pos;
        let row = self.row;
        let col = self.col;

        while let Some(curr) = self.curr() {
            if curr.is_ascii_digit() || curr == '.' {
                self.read();
            }
        }

        let value = &self.input[pos..self.pos];
        let name = if value.contains('.') { TokenName::Float } else { TokenName::Int };

        return Some(Token { name, value, row, col });
    }

    fn read_str(&mut self) -> Option<Token<'a>> {
        todo!()
    }
}



// it's not useful to single out eof condition. In the parser, you generally handle eof and any other wrong token the same. So, it’s clear to handle one unified error path for both conditions by having a dedicated eof token.
// it’s better not to pollute the lexer with io::Errors. Working with &str is usually fine. Like, source files are usually smaller than 10 mb, so it’s not a problem to keep them in memory. If there is a requirement to work with arbitrary Read, wrap it into a type that just returns eof on error and communicates the actual error via a side channel.
// similarly, it’s a good idea to not report lexer errors (malformed tokens) as a Result. Instead, one can return an explicit Error token, or emit error via diagnostic/side channel, or store error flags (like unclosed quotes) in the token itself — that way the lexer doesn’t need to stop after the first error.



#[cfg(test)]
mod tests {
    use super::*;
    use TokenName::*;

    /*
    let x: Int = 5;
    let y: Float = 5.5;
    let c: Char = '[';
    let s: Str = "hello";
    let m: Map(Str, Str) : 
    let add: Fn(Int) -> String
    */

    fn lex(input: &str) -> Vec<Token> {
        let lexer = Lexer::new(input);
        lexer.collect()
    }

    #[test]
    fn symbols() {
        let tokens = lex("{}[]();:");
        assert_eq!(
            tokens,
            &[
                Token { name: Osq, value: "{", row: 1, col: 1, },
                Token { name: Csq, value: "}", row: 1, col: 2, },
                Token { name: Obr, value: "[", row: 1, col: 3, },
                Token { name: Cbr, value: "]", row: 1, col: 4, },
                Token { name: Opr, value: "(", row: 1, col: 5, },
                Token { name: Cpr, value: ")", row: 1, col: 6, },
                Token { name: Semi, value: ";", row: 1, col: 7, },
                Token { name: Colon, value: ":", row: 1, col: 8, },
            ], 
        );
    }

    #[test]
    fn number() {
        let tokens = lex("123");
        assert_eq!(tokens, &[Token { name: Int, value: "123", row: 1, col: 1 }]);

        let tokens = lex("0.0");
        assert_eq!(tokens, &[Token { name: Float, value: "0.0", row: 1, col: 1 }]);

        let tokens = lex("123.23");
        assert_eq!(tokens, &[Token { name: Float, value: "123.23", row: 1, col: 1 }]);
    }
}
