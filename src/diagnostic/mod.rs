use crate::diagnostic::error::TolError;

pub mod error;

pub enum TolDiagnostic {
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
}
