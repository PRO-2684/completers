//! # `completers` library crate
//!
//! If you are reading this, you are reading the documentation for the `completers` library crate. For the cli, kindly refer to the README file.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use std::{num::ParseIntError, process::exit};

/// Possible errors that can occur when parsing a completion request.
pub enum CompletionError {
    /// The completion request is missing a required field.
    MissingField,
    /// The completion request contains an invalid value for some field.
    InvalidValue,
    /// Unrecognized environment variable.
    UnrecognizedEnvVar,
}

impl From<ParseIntError> for CompletionError {
    fn from(_: ParseIntError) -> Self {
        CompletionError::InvalidValue
    }
}

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
            "1" => Ok(Some(Self::from_args(std::env::args().collect())?)),
            _ => Err(CompletionError::UnrecognizedEnvVar),
        }
    }

    /// Constructs a [`Completion`] object from the arguments.
    fn from_args(args: Vec<String>) -> Result<Self, CompletionError> {
        if args.len() < 5 {
            return Err(CompletionError::MissingField);
        }
        let (positional, words) = (args[0..5].to_vec(), args[5..].to_vec());
        let positional: [String; 5] = positional
            .try_into()
            .map_err(|_| CompletionError::MissingField)?; // Shouldn't happen, but just in case
        let [word_index, line, cursor_index, completion_type, key] = positional;

        let word_index = word_index.parse::<usize>()?;
        let cursor_index = cursor_index.parse::<usize>()?;
        let completion_type = completion_type
            .parse::<u8>()?
            .try_into()
            .map_err(|_| CompletionError::InvalidValue)?;
        let key = key.parse::<u8>()?.into();

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
        I: Iterator<Item = String>,
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
