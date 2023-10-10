use std::{error::Error, fmt::Display};
#[derive(Debug)]
pub enum Errors {
    PortValueIsNotValid,
    CannotGetLocalAddr,
    CannotAcceptConnection,
    CannotWriteToStream,
    UnsupportedHttpMethod,
    MissingMethod,
    CannotReadLineWhileParsingRequest,
    MissingPath,
    MissingHeaderName,
    MissingHeaderValue,
    InvalidQueryFormat,
    CopyError,
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Errors::PortValueIsNotValid => "PortValueIsNotValid",
                Errors::CannotGetLocalAddr => "CannotGetLocalAddr",
                Errors::CannotAcceptConnection => "CannotAcceptConnection",
                Errors::CannotWriteToStream => "CannotWriteToStream",
                Errors::UnsupportedHttpMethod => "UnsupportedHttpMethod",
                Errors::MissingMethod => "MissingMethod",
                Errors::MissingPath => "MissingPath",
                Errors::MissingHeaderName => "MissingHeaderName",
                Errors::MissingHeaderValue => "MissingHeaderValue",
                Errors::CannotReadLineWhileParsingRequest => "CannotReadLineWhileParsingRequest",
                Errors::CopyError => "CopyError",
                Errors::InvalidQueryFormat => "InvalidQueryFormat",
            }
        )
    }
}

impl Error for Errors {}
