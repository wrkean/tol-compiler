use crate::{
    diagnostic::{TolDiagnostic, error::TolError},
    prelude::{Span, TolResult},
};

#[derive(Debug, Clone, PartialEq)]
pub enum TolType {
    Numero,
    Lutang,

    Wala,

    // When a data type is not yet determined, it is unknown (di alam)
    DiAlam,
}

impl TolType {
    pub fn from_str(value: &str, span: Span) -> TolResult<Self> {
        let ty = match value {
            "numero" => TolType::Numero,
            "lutang" => TolType::Lutang,
            _ => {
                return Err(TolDiagnostic::new_error(TolError::InvalidType {
                    invalid_type: value.to_string(),
                    type_span: span.into(),
                }));
            }
        };

        Ok(ty)
    }

    pub fn to_tol_str(&self) -> String {
        match self {
            TolType::Numero => "numero".to_string(),
            TolType::Lutang => "lutang".to_string(),
            TolType::Wala => "<invalid:wala>".to_string(),
            TolType::DiAlam => "<invalid:dialam>".to_string(),
        }
    }
}
