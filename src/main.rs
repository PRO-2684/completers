#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use completers::{Completion, Response, handle_completion};

fn main() {
    handle_completion(handler);
    let args: Vec<String> = std::env::args().skip(1).collect();
    for arg in args {
        println!("You've ordered: {arg}");
    }
}

/// Handles the completion request.
fn handler(completion: Completion) -> Response<Vec<String>> {
    // Demo words for completion. Should contain some words with common prefixes for demo purposes.
    const WORDLIST: [&str; 7] = [
        "apple",
        "apricot",
        "banana",
        "blueberry",
        "grape",
        "orange",
        "watermelon",
    ];
    let Some(query) = completion.words.get(completion.word_index) else {
        return Response::Candidates(vec![]);
    };
    let query = query.to_lowercase();
    // Filter the words based on the query
    let candidates = WORDLIST
        .iter()
        .filter(|word| word.starts_with(&query))
        .map(|word| word.to_string())
        .collect();
    Response::Candidates(candidates)
}
