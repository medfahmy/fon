use crate::ast::{Expr, ExprKind, Stmt, StmtKind};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind::*};

struct Parser<'a> {
    lexer: Lexer<'a>,
    curr: Option<Token<'a>>,
    peek: Option<Token<'a>>,
    curr_id: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let curr = lexer.next();
        let peek = lexer.next();

        Self {
            lexer,
            curr,
            peek,
            curr_id: 0,
        }
    }

    pub fn parse(&self) -> Self {
        todo!()
    }

    fn read_token(&mut self) {
        self.curr = self.peek.take();
        self.peek = self.lexer.next();
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        if let Some(curr) = &self.curr {
            match &curr.kind {
                Let => self.parse_let_stmt(),
                Return => self.parse_ret_stmt(),
                If => self.parse_if_stmt(),
                _ => self.parse_expr_stmt(),
            }
        } else {
            None
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        let mut exprs = Vec::new();

        self.read_token();

        if let Some(curr) = &self.curr {
            if let Ident = curr.kind {
                let ident = Expr {
                    kind: ExprKind::Ident,
                    literal: curr.value,
                    children: vec![],
                };

                exprs.push(ident);
            }
        }

        todo!()
    }

    fn parse_ret_stmt(&mut self) -> Option<Stmt> {
        todo!()
    }

    fn parse_assign_stmt(&mut self) -> Option<Stmt> {
        todo!()
    }

    fn parse_if_stmt(&mut self) -> Option<Stmt> {
        todo!()
    }

    fn parse_expr_stmt(&mut self) -> Option<Stmt> {
        todo!()
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        todo!()
    }

    fn parse_prefix_expr(&mut self) -> Option<Expr> {
        todo!()
    }

    fn parse_inifx_expr(&mut self) -> Option<Expr> {
        todo!()
    }
}
