use std::fmt;

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

    pub fn new_expression(span: Span, expr: Expr) -> Self {
        Self {
            kind: StmtKind::Expression { expr },
            span,
        }
    }

    pub fn pretty(&self) -> String {
        self.to_string()
    }
}

pub enum StmtKind {
    NameDeclaration {
        name: Token,
        ty: Option<TolType>,
        rhs: Expr,
    },
    Expression {
        expr: Expr,
    },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl fmt::Display for StmtKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StmtKind::NameDeclaration { name, ty, rhs } => {
                write!(f, "{}", name.lexeme())?;

                if let Some(ty) = ty {
                    write!(f, ": {}", display_type(ty))?;
                }

                write!(f, " = {}", rhs)
            }
            StmtKind::Expression { expr } => {
                write!(f, "Expression Statement ({})", expr.pretty())
            }
        }
    }
}

fn display_type(ty: &TolType) -> &'static str {
    match ty {
        TolType::Numero => "numero",
        TolType::Lutang => "lutang",
        TolType::Wala => "wala",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::expr::{Expr, ExprKind},
        prelude::Span,
        token::{Token, TokenKind},
    };

    fn tok(lexeme: &str, kind: TokenKind, span: Span) -> Token {
        Token::new(lexeme.to_string(), kind, span)
    }

    #[test]
    fn pretty_prints_name_declaration_with_type() {
        let stmt = Stmt::new_name_declaration(
            0..7,
            tok("x", TokenKind::Identifier, 0..1),
            Some(TolType::Numero),
            Expr::new(
                ExprKind::Integer(tok("1", TokenKind::IntLiteral, 4..5)),
                4..5,
            ),
        );

        assert_eq!(stmt.pretty(), "x: numero = 1");
    }

    #[test]
    fn pretty_prints_name_declaration_without_type() {
        let stmt = Stmt::new_name_declaration(
            0..5,
            tok("x", TokenKind::Identifier, 0..1),
            None,
            Expr::new(
                ExprKind::Identifier(tok("y", TokenKind::Identifier, 4..5)),
                4..5,
            ),
        );

        assert_eq!(stmt.pretty(), "x = y");
    }
}
