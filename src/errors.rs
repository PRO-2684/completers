//! Possible errors.

use std::io;

/// Possible errors that can occur.
#[derive(Debug)]
pub enum CompletersError {
    /// Error while generating shell code.
    ShellCode(ShellCodeError),
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

impl std::fmt::Display for CompletersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ShellCode(err) => {
                write!(f, "Shell code error: {err}")
            }
            Self::MissingField => write!(f, "Missing required field"),
            Self::InvalidValue { field, value, what } => {
                write!(f, "Invalid value for field {field}: {value} ({what})")
            }
            Self::UnrecognizedEnvVar(value) => {
                write!(f, "Unrecognized environment variable value: {value}")
            }
            Self::Custom(message) => write!(f, "Custom error: {message}"),
        }
    }
}

impl std::error::Error for CompletersError {}

impl From<ShellCodeError> for CompletersError {
    fn from(err: ShellCodeError) -> Self {
        Self::ShellCode(err)
    }
}

/// Possible errors that can occur when generating shell code.
#[derive(Debug)]
pub enum ShellCodeError {
    /// IO error.
    IO(io::Error),
    /// Encoding error.
    Encoding(String),
}

impl std::fmt::Display for ShellCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(err) => {
                write!(f, "IO error: {err}")
            }
            Self::Encoding(message) => {
                write!(f, "Encoding error: {message}")
            }
        }
    }
}

impl std::error::Error for ShellCodeError {}

impl From<io::Error> for ShellCodeError {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}
