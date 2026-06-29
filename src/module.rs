use std::sync::Arc;

use crate::{diagnostic::TolDiagnostic, token::Token};

pub struct Module {
    source_code: Arc<str>,
    diagnostics: Option<Vec<TolDiagnostic>>,
    // ast: Vec<Stmt>,
}

impl Module {
    pub fn new(source_code: impl Into<Arc<str>>) -> Self {
        Self {
            source_code: source_code.into(),
            diagnostics: None,
        }
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
}
