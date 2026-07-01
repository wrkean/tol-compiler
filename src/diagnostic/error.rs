use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Diagnostic, Error, Debug)]
pub enum TolError {
    #[error("May nakita akong hindi ko kilalang karakter: `{character}`")]
    #[diagnostic(help("Baka hindi ito parte ng aking sintaks, subukan mo itong tanggalin"))]
    UnrecognizedCharacter {
        character: char,

        #[label("Hindi ko kilala ang karakter na ito")]
        span: SourceSpan,
    },

    #[error("May nakita akong hindi inaasahang token: `{token}`")]
    #[diagnostic(help("Subukan mong palitan ng `{expected}` ang `{token}`"))]
    UnexpectedToken {
        token: String,
        expected: String,

        #[label("Umasa ako na `{expected}` ang makikita ko, ngunit ito ang nakita ko")]
        span: SourceSpan,
    },

    #[error("May nakita akong invalid na pagsimula ng isang expresyon")]
    InvalidStartOfAnExpression {
        #[label("Hindi ito pwedeng magsimula ng isang expresyon")]
        span: SourceSpan,
    },

    #[error("May nakita akong hindi ko kilalang tipo: `{invalid_type}`")]
    InvalidType {
        invalid_type: String,

        #[label("Ito ay hindi isang valid na tipo sa tol")]
        type_span: SourceSpan,
    },

    #[error("May nakita akong pangalan na idineklara mo ulit sa kaparehong sakop: `{name}`")]
    #[diagnostic(help("Maaaring isa lamang na kaparehong pangalan ang nasa isang sakop"))]
    NameRedeclaration {
        name: String,

        #[label("Naideklara mo na ang `{name}` dito...")]
        declared_span: SourceSpan,

        #[label("...at nakita kong naideklara mo ulit dito")]
        redeclared_span: SourceSpan,
    },

    #[error("May nakita akong pangalan na ginamit ngunit hindi pa ito naideklara: `{name}`")]
    UseOfUndeclaredName {
        name: String,

        #[label("Hindi mo pa ito naideklara")]
        span: SourceSpan,
    },
}
