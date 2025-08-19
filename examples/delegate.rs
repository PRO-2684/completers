#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use completers::{CompletersError, Completion};
use std::process::exit;

fn main() -> Result<(), CompletersError> {
    match Completion::init() {
        Ok(Some(completion)) => {
            delegate_to_cargo(completion)?;
        }
        Ok(None) => {
            // No completion request, do nothing
        }
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    };
    let args: Vec<String> = std::env::args().skip(1).collect();
    for arg in args {
        println!("{arg}");
    }
    Ok(())
}

/// Delegates completion to `cargo build --example`, exit if successful.
fn delegate_to_cargo(mut comp: Completion) -> Result<(), CompletersError> {
    let old_words_count = comp.words.len();
    comp.words.remove(0); // Discard program name
    let mut new_words = vec![
        "cargo".to_string(),
        "build".to_string(),
        "--example".to_string(),
    ];
    new_words.append(&mut comp.words);
    comp.words = new_words;
    comp.word_index += comp.words.len();
    comp.word_index -= old_words_count;

    comp.line = comp.words.join(" ");
    comp.cursor_index = comp
        .words
        .iter()
        .take(comp.word_index)
        .map(|word| word.len())
        .sum::<usize>()
        + comp.word_index
        + comp.words[comp.word_index].len();
    // TODO: Resolve cursor index correctly, instead of assuming it at the end of current word
    // FIXME: Proper escaping when composing `comp.line` (Although rarely does anyone use it)

    comp.delegate();
    Ok(())
}
