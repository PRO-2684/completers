#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use completers::{Completion, handle_completion};

fn main() {
    handle_completion(handler);
    let args: Vec<String> = std::env::args().skip(1).collect();
    for arg in args {
        println!("{arg}");
    }
}

/// Handles the completion request.
#[allow(clippy::needless_pass_by_value, reason = "Signature consistency")]
fn handler(completion: Completion) -> Vec<String> {
    // Demo words for completion. Should contain some words with common prefixes for demo purposes.
    const MAPPING: [&str; 10] = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];
    let Some(query) = completion.words.get(completion.word_index) else {
        return vec![];
    };
    let query = query.to_lowercase();
    // Filter the words based on the query
    let mut candidates: Vec<String> = MAPPING
        .iter()
        .filter(|word| word.starts_with(&query))
        .map(|word| (*word).to_string())
        .collect();
    // If the query is numeric, append the indexed value
    if let Ok(num) = query.parse::<usize>() {
        if let Some(&word) = MAPPING.get(num) {
            candidates.push(word.to_string());
        }
    }
    candidates
}
