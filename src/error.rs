use std::io;

use ureq::Error;

/// Various errors that can come from the function in this crate
#[derive(Debug)]
pub enum MojangError {
    /// The request is invalid
    InvalidRequest(String),

    /// IO Error while reading a stream
    ReadError(io::Error),

    /// Ureq Request Error
    RequestError(Box<Error>),

    /// IO Error
    IoError(io::Error),

    /// Error parsing Json
    ParseError(serde_json::Error),

    /// Error parsing UUID
    InvalidUuid(uuid::Error),
}

impl From<ureq::Error> for MojangError {
    fn from(value: ureq::Error) -> Self {
        MojangError::RequestError(Box::new(value))
    }
}

impl From<io::Error> for MojangError {
    fn from(value: io::Error) -> Self {
        MojangError::IoError(value)
    }
}

impl From<serde_json::Error> for MojangError {
    fn from(value: serde_json::Error) -> Self {
        MojangError::ParseError(value)
    }
}

impl From<uuid::Error> for MojangError {
    fn from(value: uuid::Error) -> Self {
        MojangError::InvalidUuid(value)
    }
}
