use crate::{ast::expr::Expr, prelude::Span, token::Token, toltype::TolType};

pub struct Stmt {
    kind: StmtKind,
    span: Span,
}

impl Stmt {
    pub fn new_name_declaration(span: Span, name: Token, ty: Option<TolType>, rhs: Expr) -> Self {
        Self {
            kind: StmtKind::NameDeclaration { name, ty, rhs },
            span,
        }
    }
}

pub enum StmtKind {
    NameDeclaration {
        name: Token,
        ty: Option<TolType>,
        rhs: Expr,
    },
}
