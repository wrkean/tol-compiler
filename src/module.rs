use std::sync::Arc;

use crate::{diagnostic::TolDiagnostic, token::Token};

pub struct Module<'src> {
    source_code: Arc<str>,
    tokens: Option<Vec<Token<'src>>>,
    diagnostics: Option<Vec<TolDiagnostic>>,
}

impl<'src> Module<'src> {
    pub fn new(source_code: impl Into<Arc<str>>) -> Self {
        Self {
            source_code: source_code.into(),
            tokens: None,
            diagnostics: None,
        }
    }

    pub fn set_tokens(&mut self, tokens: Vec<Token<'src>>) {
        self.tokens = Some(tokens);
    }

    pub fn set_diagnostics(&mut self, diagnostics: Vec<TolDiagnostic>) {
        self.diagnostics = Some(diagnostics)
    }

    pub fn source_code(&self) -> &str {
        &self.source_code
    }

    pub fn source_code_arc(&self) -> Arc<str> {
        Arc::clone(&self.source_code)
    }

    pub fn display_tokens(&self) {
        if self.tokens.is_none() {
            panic!("This module does not yet own a stream of tokens")
        }

        for token in self.tokens.as_ref().unwrap() {
            println!(
                "{} => {:?} at [{}:{}]",
                token.lexeme(),
                token.kind(),
                token.span().start,
                token.span().end
            )
        }
    }
}
