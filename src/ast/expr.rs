use std::fmt;

use crate::{ast::stmt::Stmt, prelude::Span, token::Token};

#[derive(Debug)]
pub struct Expr {
    kind: ExprKind,
    span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn new_block(span: Span, statements: Vec<Stmt>) -> Self {
        Self {
            kind: ExprKind::Block { statements },
            span,
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn kind(&self) -> &ExprKind {
        &self.kind
    }
}

#[derive(Debug)]
pub enum ExprKind {
    Integer(Token),
    Float(Token),
    Identifier(Token),
    Binary {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        op: Token,
    },
    Block {
        statements: Vec<Stmt>,
    },
}

// ============= USED FOR TESTS ONLY =============
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl fmt::Display for ExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprKind::Integer(tok) | ExprKind::Float(tok) | ExprKind::Identifier(tok) => {
                write!(f, "{}", tok.lexeme())
            }
            ExprKind::Binary { lhs, rhs, op } => {
                write!(f, "({} {} {})", lhs, op.lexeme(), rhs)
            }
            _ => unimplemented!(),
        }
    }
}
