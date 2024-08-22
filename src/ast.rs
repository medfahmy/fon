use crate::token::Token;

pub struct AST<'a> {
    pub nodes: Vec<Stmt<'a>>,
}

pub enum StmtKind {
    Let,
    Return,
    If,
    Fn,
    Expr,
}

pub struct Stmt<'a> {
    pub kind: StmtKind,
    pub literal: &'a str,
    pub exprs: Vec<Expr<'a>>,
}

pub enum ExprKind {
    Prefix,
    Infix,
    Ident,
    Call,
}

pub struct Expr<'a> {
    pub kind: ExprKind,
    pub literal: &'a str,
    pub children: Vec<Expr<'a>>,
}
