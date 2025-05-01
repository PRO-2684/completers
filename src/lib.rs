#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

mod errors;
mod types;

use errors::{CompletersError, ShellCodeError};
use std::{env, path::absolute, process::exit};
use types::CompletionType;

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
    match Completion::init() {
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
    /// Construct a [`Completion`] object from command line arguments and envs. If `COMPLETE` environment variable:
    ///
    /// - Is not set, or set to `0` or empty, return `None`.
    /// - Is set to `1`, return a [`Completion`] object.
    /// - Is set to `bash`, generate shell code and exit successfully.
    ///
    /// ## Errors
    ///
    /// If `COMPLETE` is set to any other value, return [`CompletersError::UnrecognizedEnvVar`]; If [`generate`](Completion::generate) fails, return [`ShellCodeError`].
    pub fn init() -> Result<Option<Self>, CompletersError> {
        // Check if the COMPLETE environment variable is set
        let Ok(complete) = env::var("COMPLETE") else {
            return Ok(None);
        };
        match complete.as_str() {
            "" | "0" => Ok(None),
            "1" => Ok(Some(Self::from_args(env::args().skip(1).collect())?)),
            "bash" => {
                println!("{}", Self::generate_bash()?);

                exit(0);
            }
            _ => Err(CompletersError::UnrecognizedEnvVar(complete)),
        }
    }

    /// Constructs a [`Completion`] object from the arguments, without the first argument (the program name).
    fn from_args(mut args: Vec<String>) -> Result<Self, CompletersError> {
        use CompletersError::InvalidValue;
        if args.len() < 5 {
            return Err(CompletersError::MissingField);
        }
        let words = args.split_off(5);
        let positional: [String; 5] = args.try_into().map_err(|_| CompletersError::MissingField)?; // Shouldn't happen, but just in case
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
        let completion_type = completion_type.try_into().map_err(|()| InvalidValue {
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

    /// Process the completion request and exit successfully.
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

    /// Generate Bash completion code.
    ///
    /// ## Errors
    ///
    /// If the program name cannot be determined or is not a valid identifier in Bash, return [`ShellCodeError::Encoding`]. If IO error occurs, return [`ShellCodeError::IO`].
    pub fn generate_bash() -> Result<String, ShellCodeError> {
        // We want to keep symbolic links, so we don't use `canonicalize`
        let path = env::args().nth(0).map_or_else(env::current_exe, absolute)?;
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| ShellCodeError::Encoding("Failed to decode program name".to_string()))?;
        if !is_safe(name) {
            return Err(ShellCodeError::Encoding(
                "Program name contains unsafe characters".to_string(),
            ));
        }

        let path = path
            .to_str()
            .ok_or_else(|| ShellCodeError::Encoding("Failed to decode program path".to_string()))?;
        if !is_safe(path) {
            return Err(ShellCodeError::Encoding(
                "Program path contains unsafe characters".to_string(),
            ));
        }

        // Generate the completion code
        Ok(format!(
            r#"_completer_{name}() {{
    local IFS=$'\n'
    COMPREPLY=($(COMPLETE=1 {path} "$COMP_CWORD" "$COMP_LINE" "$COMP_POINT" "$COMP_TYPE" "$COMP_KEY" "${{COMP_WORDS[@]}}"))
}}
complete -F _completer_{name} {name}"#,
        ))
    }
}

/// Checks if the string is safe in Bash.
fn is_safe(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_alphanumeric() || "_-./\\".contains(c))
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
