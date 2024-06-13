use crate::token::Token;

pub enum InfixOp {
    Add, Sub, Mul, Div, Rem,
    And, Or,
    Eq, Lt, Lte, Gt, Gte,
}

pub enum PrefixOp {
    Not, Neg,
}

pub enum ExprKind {
    Ident,
    Prefix(PrefixOp, Box<ExprKind>),
    Infix(InfixOp, Box<ExprKind>, Box<ExprKind>),
    Call(Box<ExprKind>),
}

pub struct Expr<'a> {
    id: usize,
    kind: ExprKind,
    literal: &'a str,
}

pub enum StmtKind {
    Let(ExprKind),
    Return(ExprKind),
    Assign,
    Expr(ExprKind),
    If(ExprKind, Box<StmtKind>, Box<StmtKind>),
    Block(Vec<StmtKind>),
}

pub struct Stmt<'a> {
    id: usize,
    kind: StmtKind,
    literal: &'a str,
}
