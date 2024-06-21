pub struct AST<'a> {
    nodes: Vec<Stmt<'a>>,
}

pub enum StmtKind {
    Let,
    Return,
    If,
    Fn,
    Expr,
}

pub struct Stmt<'a> {
    kind: StmtKind,
    literal: &'a str,
    exprs: Vec<Expr<'a>>,
}

pub enum ExprKind {
    Prefix,
    Infix,
    Ident,
    Call,
}

pub struct Expr<'a> {
    kind: ExprKind, 
    literal: &'a str,
    children: Vec<usize>,
}
