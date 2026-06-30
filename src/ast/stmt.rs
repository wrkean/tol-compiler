use std::fmt;

use crate::{
    ast::{Param, expr::Expr},
    prelude::{Span, Spanned},
    token::Token,
    toltype::TolType,
};

#[derive(Debug)]
pub struct Stmt {
    kind: StmtKind,
    span: Span,
}

impl Stmt {
    pub fn new_name_declaration(
        span: Span,
        is_mutable: bool,
        name: Token,
        ty: Option<TolType>,
        rhs: Expr,
    ) -> Self {
        Self {
            kind: StmtKind::NameDeclaration {
                is_mutable,
                name,
                ty,
                rhs,
            },
            span,
        }
    }

    pub fn new_expression(span: Span, expr: Expr) -> Self {
        Self {
            kind: StmtKind::Expression { expr },
            span,
        }
    }

    pub fn new_par(
        span: Span,
        name: Token,
        params: Spanned<Vec<Param>>,
        ret_ty: Option<TolType>,
        block: Expr,
    ) -> Self {
        Self {
            kind: StmtKind::FunctionDeclaration {
                name,
                params,
                ret_ty,
                block,
            },
            span,
        }
    }

    pub fn kind(&self) -> &StmtKind {
        &self.kind
    }
}

#[derive(Debug)]
pub enum StmtKind {
    NameDeclaration {
        is_mutable: bool,
        name: Token,
        ty: Option<TolType>,
        rhs: Expr,
    },
    FunctionDeclaration {
        name: Token,
        params: Spanned<Vec<Param>>,
        ret_ty: Option<TolType>,
        block: Expr,
    },
    Expression {
        expr: Expr,
    },
}
