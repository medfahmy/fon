use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum TokenName {
    Semi, Colon, Dot,
    Osq, Csq, Obr, Cbr, Opr, Cpr, Otg, Ctg,
    Eq, Add, Sub, Mul, Div,
    Assign, AddAssign, SubAssign, MulAssign, DivAssign,
    Lt, Lte, Gt, Gte,
    Bang, And, Or,
    Ident, Let, Return, If, Else, For,
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
        write!(
            f,
            "{}(`{}`, [{}:{}])",
            self.name,
            self.value.to_string(),
            self.row,
            self.col
        )
    }
}
