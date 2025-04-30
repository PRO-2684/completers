//! # `completers` library crate
//!
//! If you are reading this, you are reading the documentation for the `completers` library crate. For the cli, kindly refer to the README file.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use std::process::exit;

/// Helper function for handling completion requests.
///
/// ## Panics
///
/// This function panics if the completion request is invalid or if the environment variable `COMPLETE`'s value is not recognized.
pub fn handle_completion<F, I>(handler: F)
where
    F: FnOnce(Completion) -> I,
    I: IntoIterator<Item = String>,
{
    // Completion::new().unwrap().map(handler).map(Completion::complete);
    match Completion::new() {
        Ok(Some(completion)) => {
            let candidates = handler(completion);
            Completion::complete(candidates);
        }
        Ok(None) => {
            // No completion request, do nothing
        }
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    }
}

/// Possible errors that can occur when parsing a completion request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionError {
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
}

impl std::fmt::Display for CompletionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompletionError::MissingField => write!(f, "Missing required field"),
            CompletionError::InvalidValue { field, value, what } => {
                write!(f, "Invalid value for field {field}: {value} ({what})")
            }
            CompletionError::UnrecognizedEnvVar(value) => write!(f, "Unrecognized environment variable value: {value}"),
        }
    }
}

impl std::error::Error for CompletionError {}

/// A completion request from the shell. [ref](https://www.gnu.org/software/bash/manual/html_node/Programmable-Completion.html).
pub struct Completion {
    /// An array of all the words in the command line, including the command itself. Corresponds to `COMP_WORDS`.
    pub words: Vec<String>,
    /// The index of the word in [`words`](Completion::words) that is currently being completed. Corresponds to `COMP_CWORD`.
    pub word_index: usize,
    /// The current command line. Corresponds to `COMP_LINE`.
    pub line: String,
    /// The index of the current cursor position relative to the beginning of the current command. Corresponds to `COMP_POINT`.
    pub cursor_index: usize,
    /// The type of completion attempted. Corresponds to `COMP_TYPE`.
    pub completion_type: CompletionType,
    /// The key (or final key of a key sequence) used to invoke the current completion function. Corresponds to `COMP_KEY`.
    pub key: char,
}

impl Completion {
    /// Construct a new [`Completion`] object from command line arguments. If `COMPLETE` environment variable:
    ///
    /// - Is not set, or set to `0` or empty, return `None`.
    /// - Is set to `1`, return a [`Completion`] object.
    pub fn new() -> Result<Option<Self>, CompletionError> {
        // Check if the COMPLETE environment variable is set
        let Ok(complete) = std::env::var("COMPLETE") else {
            return Ok(None);
        };
        match complete.as_str() {
            "" | "0" => Ok(None),
            "1" => Ok(Some(Self::from_args(std::env::args().skip(1).collect())?)),
            _ => Err(CompletionError::UnrecognizedEnvVar(complete)),
        }
    }

    /// Constructs a [`Completion`] object from the arguments, without the first argument (the program name).
    fn from_args(args: Vec<String>) -> Result<Self, CompletionError> {
        use CompletionError::InvalidValue;
        if args.len() < 5 {
            return Err(CompletionError::MissingField);
        }
        let (positional, words) = (args[0..5].to_vec(), args[5..].to_vec());
        let positional: [String; 5] = positional
            .try_into()
            .map_err(|_| CompletionError::MissingField)?; // Shouldn't happen, but just in case
        let [word_index, line, cursor_index, completion_type, key] = positional;

        let word_index = word_index.parse::<usize>().map_err(|e| InvalidValue {
            field: "word_index".to_string(),
            value: word_index,
            what: e.to_string(),
        })?;
        let cursor_index = cursor_index.parse::<usize>().map_err(|e| InvalidValue {
            field: "cursor_index".to_string(),
            value: cursor_index,
            what: e.to_string(),
        })?;
        let completion_type = completion_type.parse::<u8>().map_err(|e| InvalidValue {
            field: "completion_type".to_string(),
            value: completion_type,
            what: e.to_string(),
        })?;
        let completion_type = completion_type.try_into().map_err(|_| InvalidValue {
            field: "completion_type".to_string(),
            value: completion_type.to_string(),
            what: "Cannot interpret completion type".to_string(),
        })?;
        let key = key
            .parse::<u8>()
            .map_err(|e| InvalidValue {
                field: "key".to_string(),
                value: key,
                what: e.to_string(),
            })?
            .into();

        Ok(Self {
            words,
            word_index,
            line,
            cursor_index,
            completion_type,
            key,
        })
    }

    /// Answer the completion request and exit.
    pub fn complete<I>(candidates: I)
    where
        I: IntoIterator<Item = String>,
    {
        // Print the candidates to stdout, separated by newlines
        for candidate in candidates {
            println!("{candidate}");
        }
        exit(0);
    }
}

/// The type of completion attempted. [ref](https://www.gnu.org/software/bash/manual/html_node/Bash-Variables.html#index-COMP_005fTYPE).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CompletionType {
    /// Normal completion.
    Normal = b'\t',
    /// Listing completions after successive tabs.
    List = b'?',
    /// Listing alternatives on partial word completion.
    ListAlternatives = b'!',
    /// List completions if the word is not unmodified.
    ListUnmodified = b'@',
    /// Menu completion.
    Menu = b'%',
}

impl From<CompletionType> for u8 {
    fn from(completion_type: CompletionType) -> Self {
        completion_type as u8
    }
}

impl From<CompletionType> for char {
    fn from(completion_type: CompletionType) -> Self {
        completion_type as u8 as char
    }
}

impl TryFrom<u8> for CompletionType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            b'\t' => Ok(CompletionType::Normal),
            b'?' => Ok(CompletionType::List),
            b'!' => Ok(CompletionType::ListAlternatives),
            b'@' => Ok(CompletionType::ListUnmodified),
            b'%' => Ok(CompletionType::Menu),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for CompletionType {
    type Error = ();

    fn try_from(value: char) -> Result<Self, ()> {
        match value as u8 {
            b'\t' => Ok(CompletionType::Normal),
            b'?' => Ok(CompletionType::List),
            b'!' => Ok(CompletionType::ListAlternatives),
            b'@' => Ok(CompletionType::ListUnmodified),
            b'%' => Ok(CompletionType::Menu),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_from_args() {
        let args = vec![
            "1".to_string(),            // index
            "my_command s".to_string(), // line
            "12".to_string(),           // cursor index
            "33".to_string(),           // completion type
            "9".to_string(),            // key
            "my_command".to_string(),   // words
            "s".to_string(),            // words
        ];
        let completion = Completion::from_args(args).unwrap();
        assert_eq!(completion.words, vec!["my_command", "s"]);
        assert_eq!(completion.word_index, 1);
        assert_eq!(completion.line, "my_command s");
        assert_eq!(completion.cursor_index, 12);
        assert_eq!(completion.completion_type, CompletionType::ListAlternatives);
        assert_eq!(completion.key, '\t');
    }
}
