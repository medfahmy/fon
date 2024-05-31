use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum TokenName {
    Osq, Csq, Obr, Cbr, Opr, Cpr, Otg, Ctg,
    Eq, Add, Sub, Mul, Div,
    Semi, Colon, Dot,
    Assign, AddAssign, SubAssign, MulAssign, DivAssign,
    Bang, And, Or,
    Let, Return, Ident,
    Int, Float, Char, Str, List, Map,
    Data, TypeSignature,
    Fn,
    Error,
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
        write!(f, "{}(`{}`, [{}:{}])", self.name, self.value.to_string(), self.row, self.col)
    }
}
