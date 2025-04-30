//! Errors related to completion.

use std::io;

/// Possible errors that can occur when parsing a completion request.
#[derive(Debug)]
pub enum CompletionError {
    /// IO error while resolving the program path.
    IO(io::Error),
    /// Encoding error.
    Encoding(String),
    /// The completion request is missing a required field.
    MissingField,
    /// The completion request contains an invalid value for some field.
    InvalidValue {
        /// The field that contains the invalid value.
        field: String,
        /// The invalid value.
        value: String,
        /// The error message.
        what: String,
    },
    /// Unrecognized environment variable value.
    UnrecognizedEnvVar(String),
    /// Custom error message.
    Custom(String),
}

impl std::fmt::Display for CompletionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompletionError::IO(err) => {
                write!(f, "IO error: {err}")
            }
            CompletionError::Encoding(message) => {
                write!(f, "Encoding error: {message}")
            }
            CompletionError::MissingField => write!(f, "Missing required field"),
            CompletionError::InvalidValue { field, value, what } => {
                write!(f, "Invalid value for field {field}: {value} ({what})")
            }
            CompletionError::UnrecognizedEnvVar(value) => write!(f, "Unrecognized environment variable value: {value}"),
            CompletionError::Custom(message) => write!(f, "Custom error: {message}"),
        }
    }
}

impl std::error::Error for CompletionError {}

impl From<io::Error> for CompletionError {
    fn from(err: io::Error) -> Self {
        CompletionError::IO(err)
    }
}
