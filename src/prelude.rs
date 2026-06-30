use std::ops::Range;

use crate::diagnostic::TolDiagnostic;

pub type Span = Range<usize>;
pub type TolResult<T> = Result<T, TolDiagnostic>;

#[derive(Debug)]
pub struct Spanned<T> {
    item: T,
    span: Span,
}

impl<T> Spanned<T> {
    pub fn new(item: T, span: Span) -> Self {
        Self { item, span }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn item(&self) -> &T {
        &self.item
    }
}
