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
}
