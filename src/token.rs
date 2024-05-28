use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum TokenName {
    Let, Ret,
    Id,
    Osq, Csq, Obr, Cbr, Opr, Cpr, Otg, Ctg,
    Assign, Plus, Minus, Asterisk, Slash,
    Semi, Colon, Dot,
    Int, Float, Char, Str, 
    Fn,
}

impl Display for TokenName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub name: TokenName,
    pub value: &'a str,
    pub row: usize,
    pub col: usize,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "`{}` {} [{}:{}]", self.name, self.value.to_string(), self.row, self.col)
    }
}
