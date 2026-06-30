use crate::{prelude::Span, token::Token, toltype::TolType};

pub mod expr;
pub mod pretty_printer;
pub mod stmt;

#[derive(Debug)]
pub struct Param {
    name: Token,
    ty: TolType,
    span: Span,
}

impl Param {
    pub fn new(name: Token, ty: TolType, span: Span) -> Self {
        Self { name, ty, span }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn ty(&self) -> &TolType {
        &self.ty
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}
