use crate::token::Token;

pub enum InfixOp {
    Add, Sub, Mul, Div, Rem,
    And, Or,
    Eq, Lt, Lte, Gt, Gte,
}

pub enum PrefixOp {
    Not, Neg,
}

pub enum Expr<'a> {
    Ident(&'a str),
    Prefix(PrefixOp, Box<Expr<'a>>),
    Infix(InfixOp, Box<Expr<'a>>, Box<Expr<'a>>),
    Call(&'a str, Box<Expr<'a>>),
}

pub enum Stmt<'a> {
    Let(&'a str, Expr<'a>),
    Return(Expr<'a>),
    Expr(Expr<'a>),
}
