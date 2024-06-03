use crate::token::{
    Token,
    TokenName::{self, *},
};

#[derive(Debug)]
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
        self.curr().map(|curr| match curr {
            '=' | '+' | '-' | '*' | '/' | '<' | '>' | '&' | '|' => self.read_op(),
            '"' => self.read_str(),
            c if c.is_ascii_digit() => self.read_num(),
            c if c.is_alphabetic() || c == '_' => self.read_ident(),
            _ => self.read_symbol(),
        })?
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
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

    fn read_ident(&mut self) -> Option<Token<'a>> {
        let pos = self.pos;
        let row = self.row;
        let col = self.col;

        while let Some(curr) = self.curr() {
            if curr.is_alphabetic() || curr == '_' {
                self.read();
            } else {
                break;
            }
        }

        let value = &self.input[pos..self.pos];
        let name = match value {
            "let" => Let,
            "return" => Return,
            "if" => If,
            "else" => Else,
            "for" => For,
            _ => Ident,
        };
        Some(Token {
            name,
            value,
            row,
            col,
        })
    }

    fn read_symbol(&mut self) -> Option<Token<'a>> {
        let name = if let Some(curr) = self.curr() {
            match curr {
                '{' => Osq,
                '}' => Csq,
                '[' => Obr,
                ']' => Cbr,
                '(' => Opr,
                ')' => Cpr,
                ':' => Colon,
                ';' => Semi,
                '!' => Bang,
                _ => return None,
            }
        } else {
            return None;
        };

        let token = Token {
            name,
            value: &self.input[self.pos..self.pos + 1],
            row: self.row,
            col: self.col,
        };
        self.read();
        Some(token)
    }

    fn read_num(&mut self) -> Option<Token<'a>> {
        let pos = self.pos;
        let row = self.row;
        let col = self.col;

        while let Some(curr) = self.curr() {
            if curr.is_ascii_digit() || curr == '.' {
                self.read();
            } else {
                break;
            }
        }

        let value = &self.input[pos..self.pos];
        let name = if value.contains('.') { Float } else { Int };
        Some(Token {
            name,
            value,
            row,
            col,
        })
    }

    fn read_str(&mut self) -> Option<Token<'a>> {
        self.read();
        let pos = self.pos;
        let row = self.row;
        let col = self.col;

        while let Some(curr) = self.curr() {
            if curr == '"' {
                break;
            } else {
                self.read();
            }
        }

        let token = Token {
            name: Str,
            value: &self.input[pos..self.pos],
            row,
            col,
        };
        self.read();
        Some(token)
    }

    fn read_op(&mut self) -> Option<Token<'a>> {
        let pos = self.pos;
        let row = self.row;
        let col = self.col;
        let name = if let Some(curr) = self.curr() {
            match curr {
                '=' => if let Some('=') = self.peek() { self.read(); Eq } else { Assign },
                '+' => if let Some('=') = self.peek() { self.read(); AddAssign } else { Add },
                '-' => if let Some('=') = self.peek() { self.read(); SubAssign } else { Sub },
                '*' => if let Some('=') = self.peek() { self.read(); MulAssign } else { Mul },
                '/' => if let Some('=') = self.peek() { self.read(); DivAssign } else { Div },
                '<' => if let Some('=') = self.peek() { self.read(); Lte } else { Lt },
                '>' => if let Some('=') = self.peek() { self.read(); Gte } else { Gt },
                '&' => if let Some('&') = self.peek() { self.read(); And } else { Error },
                '|' => if let Some('|') = self.peek() { self.read(); Or } else { Error },
                _ => Error,
            }
        } else {
            return None;
        };

        self.read();
        let token = Token {
            name,
            value: &self.input[pos..self.pos],
            row,
            col,
        };
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    let x: Int = 5;
    let y: Float = 5.5;
    let c: Char = '[';
    let s: Str = "hello";
    let m: Map(Str, Str) = Map("key1": "value1", "key2": "value2");
    let add: Fn(Int, Int) -> Int = Fn(x, y) -> { x + y };
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
                Token {
                    name: Osq,
                    value: "{",
                    row: 1,
                    col: 1,
                },
                Token {
                    name: Csq,
                    value: "}",
                    row: 1,
                    col: 2,
                },
                Token {
                    name: Obr,
                    value: "[",
                    row: 1,
                    col: 3,
                },
                Token {
                    name: Cbr,
                    value: "]",
                    row: 1,
                    col: 4,
                },
                Token {
                    name: Opr,
                    value: "(",
                    row: 1,
                    col: 5,
                },
                Token {
                    name: Cpr,
                    value: ")",
                    row: 1,
                    col: 6,
                },
                Token {
                    name: Semi,
                    value: ";",
                    row: 1,
                    col: 7,
                },
                Token {
                    name: Colon,
                    value: ":",
                    row: 1,
                    col: 8,
                },
            ],
        );
    }

    #[test]
    fn number() {
        assert_eq!(
            lex("123"),
            &[Token {
                name: Int,
                value: "123",
                row: 1,
                col: 1
            }]
        );
        assert_eq!(
            lex("0.0"),
            &[Token {
                name: Float,
                value: "0.0",
                row: 1,
                col: 1
            }]
        );
        assert_eq!(
            lex("123.23"),
            &[Token {
                name: Float,
                value: "123.23",
                row: 1,
                col: 1
            }]
        );
    }

    #[test]
    fn str() {
        assert_eq!(
            lex("\"hello\""),
            &[Token {
                name: Str,
                value: "hello",
                row: 1,
                col: 2
            }]
        );
    }

    #[test]
    fn ident() {
        assert_eq!(
            lex("x"),
            &[Token {
                name: Ident,
                value: "x",
                row: 1,
                col: 1
            }]
        );
        assert_eq!(
            lex("let"),
            &[Token {
                name: Let,
                value: "let",
                row: 1,
                col: 1
            }]
        );
        assert_eq!(
            lex("return"),
            &[Token {
                name: Return,
                value: "return",
                row: 1,
                col: 1
            }]
        );
    }

    #[test]
    fn ops() {
        assert_eq!(
            lex("x + y"),
            &[
                Token {
                    name: Ident,
                    value: "x",
                    row: 1,
                    col: 1
                },
                Token {
                    name: Add,
                    value: "+",
                    row: 1,
                    col: 3
                },
                Token {
                    name: Ident,
                    value: "y",
                    row: 1,
                    col: 5
                },
            ]
        );

        assert_eq!(
            lex("x += y"),
            &[
                Token {
                    name: Ident,
                    value: "x",
                    row: 1,
                    col: 1
                },
                Token {
                    name: AddAssign,
                    value: "+=",
                    row: 1,
                    col: 3
                },
                Token {
                    name: Ident,
                    value: "y",
                    row: 1,
                    col: 6
                },
            ]
        );

        assert_eq!(
            lex("x - y"),
            &[
                Token {
                    name: Ident,
                    value: "x",
                    row: 1,
                    col: 1
                },
                Token {
                    name: Sub,
                    value: "-",
                    row: 1,
                    col: 3
                },
                Token {
                    name: Ident,
                    value: "y",
                    row: 1,
                    col: 5
                },
            ]
        );

        assert_eq!(
            lex("x -= y"),
            &[
                Token {
                    name: Ident,
                    value: "x",
                    row: 1,
                    col: 1
                },
                Token {
                    name: SubAssign,
                    value: "-=",
                    row: 1,
                    col: 3
                },
                Token {
                    name: Ident,
                    value: "y",
                    row: 1,
                    col: 6
                },
            ]
        );

        assert_eq!(
            lex("x * y"),
            &[
                Token {
                    name: Ident,
                    value: "x",
                    row: 1,
                    col: 1
                },
                Token {
                    name: Mul,
                    value: "*",
                    row: 1,
                    col: 3
                },
                Token {
                    name: Ident,
                    value: "y",
                    row: 1,
                    col: 5
                },
            ]
        );

        assert_eq!(
            lex("x *= y"),
            &[
                Token {
                    name: Ident,
                    value: "x",
                    row: 1,
                    col: 1
                },
                Token {
                    name: MulAssign,
                    value: "*=",
                    row: 1,
                    col: 3
                },
                Token {
                    name: Ident,
                    value: "y",
                    row: 1,
                    col: 6
                },
            ]
        );

        assert_eq!(
            lex("x / y"),
            &[
                Token {
                    name: Ident,
                    value: "x",
                    row: 1,
                    col: 1
                },
                Token {
                    name: Div,
                    value: "/",
                    row: 1,
                    col: 3
                },
                Token {
                    name: Ident,
                    value: "y",
                    row: 1,
                    col: 5
                },
            ]
        );

        assert_eq!(
            lex("x /= y"),
            &[
                Token {
                    name: Ident,
                    value: "x",
                    row: 1,
                    col: 1
                },
                Token {
                    name: DivAssign,
                    value: "/=",
                    row: 1,
                    col: 3
                },
                Token {
                    name: Ident,
                    value: "y",
                    row: 1,
                    col: 6
                },
            ]
        );

        assert_eq!(
            lex("x && y"),
            &[
                Token {
                    name: Ident,
                    value: "x",
                    row: 1,
                    col: 1
                },
                Token {
                    name: And,
                    value: "&&",
                    row: 1,
                    col: 3
                },
                Token {
                    name: Ident,
                    value: "y",
                    row: 1,
                    col: 6
                },
            ]
        );

        assert_eq!(
            lex("x || y"),
            &[
                Token {
                    name: Ident,
                    value: "x",
                    row: 1,
                    col: 1
                },
                Token {
                    name: Or,
                    value: "||",
                    row: 1,
                    col: 3
                },
                Token {
                    name: Ident,
                    value: "y",
                    row: 1,
                    col: 6
                },
            ]
        );
    }

    #[test]
    fn let_stmt() {
        assert_eq!(
            lex("let x = 1;"),
            &[
                Token {
                    name: Let,
                    value: "let",
                    row: 1,
                    col: 1
                },
                Token {
                    name: Ident,
                    value: "x",
                    row: 1,
                    col: 5
                },
                Token {
                    name: Assign,
                    value: "=",
                    row: 1,
                    col: 7
                },
                Token {
                    name: Int,
                    value: "1",
                    row: 1,
                    col: 9
                },
                Token {
                    name: Semi,
                    value: ";",
                    row: 1,
                    col: 10
                }
            ]
        );
    }

    #[test]
    fn return_stmt() {
        assert_eq!(
            lex("return x;"),
            &[
                Token {
                    name: Return,
                    value: "return",
                    row: 1,
                    col: 1
                },
                Token {
                    name: Ident,
                    value: "x",
                    row: 1,
                    col: 8
                },
                Token {
                    name: Semi,
                    value: ";",
                    row: 1,
                    col: 9
                }
            ]
        );
    }

    #[test]
    fn if_else() {
        assert_eq!(
            lex("if x < 1 { x } else { 0 };"),
            &[
                //          12345678901234567890123456
                Token {
                    name: If,
                    value: "if",
                    row: 1,
                    col: 1
                },
                Token {
                    name: Ident,
                    value: "x",
                    row: 1,
                    col: 4
                },
                Token {
                    name: Lt,
                    value: "<",
                    row: 1,
                    col: 6
                },
                Token {
                    name: Int,
                    value: "1",
                    row: 1,
                    col: 8
                },
                Token {
                    name: Osq,
                    value: "{",
                    row: 1,
                    col: 10
                },
                Token {
                    name: Ident,
                    value: "x",
                    row: 1,
                    col: 12
                },
                Token {
                    name: Csq,
                    value: "}",
                    row: 1,
                    col: 14
                },
                Token {
                    name: Else,
                    value: "else",
                    row: 1,
                    col: 16
                },
                Token {
                    name: Osq,
                    value: "{",
                    row: 1,
                    col: 21
                },
                Token {
                    name: Int,
                    value: "0",
                    row: 1,
                    col: 23
                },
                Token {
                    name: Csq,
                    value: "}",
                    row: 1,
                    col: 25
                },
                Token {
                    name: Semi,
                    value: ";",
                    row: 1,
                    col: 26
                },
            ]
        );
    }
}
