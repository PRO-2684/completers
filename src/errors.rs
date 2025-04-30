//! Errors related to completion.

/// Possible errors that can occur when parsing a completion request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionError {
    /// Missing program name when generating completion.
    MissingProgramName,
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
            CompletionError::MissingProgramName => write!(f, "Missing program name"),
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
