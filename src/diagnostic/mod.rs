use miette::{Diagnostic, IntoDiagnostic};
use thiserror::Error;

use crate::diagnostic::error::TolError;

pub mod error;

#[derive(Diagnostic, Error, Debug)]
pub enum TolDiagnostic {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Error(TolError),
}

pub enum DiagnosticType {
    Error,
    Warning,
    Advice,
}

impl TolDiagnostic {
    pub fn new_error(error: TolError) -> Self {
        Self::Error(error)
    }

    pub fn ty(&self) -> DiagnosticType {
        match self {
            Self::Error(_) => DiagnosticType::Error,
        }
    }

    pub fn display_banner(&self) {
        match self.ty() {
            DiagnosticType::Error => eprintln!("{} ERROR {}", "!".repeat(30), "!".repeat(30)),
            DiagnosticType::Warning => {
                eprintln!("{} WARNING {}", "~".repeat(30), "~".repeat(30))
            }
            DiagnosticType::Advice => eprintln!("{} ADVICE {}", "=".repeat(30), "=".repeat(30)),
        }
    }
}
