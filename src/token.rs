use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum TokenKind {
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

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub value: &'a str,
    pub row: usize,
    pub col: usize,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}(`{}`, [{}:{}])",
            self.kind,
            self.value.to_string(),
            self.row,
            self.col
        )
    }
}
