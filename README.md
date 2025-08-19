# completers

[![GitHub License](https://img.shields.io/github/license/PRO-2684/completers?logo=opensourceinitiative)](https://github.com/PRO-2684/completers/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/completers/release.yml?logo=githubactions)](https://github.com/PRO-2684/completers/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/completers?logo=githubactions)](https://github.com/PRO-2684/completers/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/completers/total?logo=github)](https://github.com/PRO-2684/completers/releases)
[![Crates.io Version](https://img.shields.io/crates/v/completers?logo=rust)](https://crates.io/crates/completers)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/completers?logo=rust)](https://crates.io/crates/completers)
[![docs.rs](https://img.shields.io/docsrs/completers?logo=rust)](https://docs.rs/completers)

> [!WARNING]
> This crate is still a prototype, and is subject to BREAKING changes without notice.

A tiny Rust-native shell completion solution.

## 💡 Examples

See [`examples`](./examples/README.md) for a few examples of how to use this crate.

## 📖 Usage

### Rust Part

#### Candidates

First, define a completion handler function that takes a [`Completion`] struct as an argument and returns a vector of completion candidates:

```rust
use completers::Completion;

fn handler(_completion: Completion) -> Vec<String> {
    vec![]
}
```

Then, call [`handle_completion`] BEFORE any other command that writes to stdout in your main function:

```rust
use completers::{Completion, handle_completion};

fn main() {
    handle_completion(handler);
    // Other logic
}
#
# fn handler(_completion: Completion) -> Vec<String> {
#     vec![]
# }
```

#### Delegate

To delegate completion, we should first match against [`Completion::init()`]:

```rust
use completers::{CompletersError, Completion};
use std::process::exit;

fn main() -> Result<(), CompletersError> {
    match Completion::init() {
        Ok(Some(completion)) => {
            delegate_completion(completion)?;
        }
        Ok(None) => {
            // No completion request, do nothing
        }
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    };
    // Do your job
    Ok(())
}
#
# fn delegate_completion(mut comp: Completion) -> Result<(), CompletersError> {
#     Ok(())
# }
```

Then, construct or mutate the [`Completion`] object in the delegate function. We'll delegate to `cargo build --example` for example:

```rust
# use completers::{CompletersError, Completion};
# use std::process::exit;
#
/// Delegates completion to `cargo build --example`, exit if successful.
fn delegate_completion(mut comp: Completion) -> Result<(), CompletersError> {
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

    comp.delegate();
    Ok(())
}
```

### Shell Part

> [!NOTE]
> By using `completers`, we assume that you've got [`bash-completion`](https://github.com/scop/bash-completion) installed. Some features such as completion delegate won't work without it.

Generate and evaluate the shell code via:

```bash
source <(COMPLETE=bash my_binary)
```

You should be able to complete your commands now. To enable completion across all your terminal sessions, you can add the above code to your completions directory, like:

```bash
mkdir -p ~/.local/share/bash-completion/completions # Create the directory if it doesn't exist
echo 'source <(COMPLETE=bash my_binary)' > ~/.local/share/bash-completion/completions/my_binary
```

You can also use `/usr/share/bash-completion/completions/` as the directory, if you want the completion to be available system-wise.

### The `completers` Binary

Currently, the `completers` binary does nothing.

## ⚙️ Mechanism

See [`MECHANISM.md`](doc/MECHANISM.md) for a detailed explanation of how this works, in case you're curious.

## 🎉 Credits

- [`clap`](https://github.com/clap-rs/clap), whose code and API is used as a reference. When `clap`'s [Rust-Native Completion Engine](https://github.com/clap-rs/clap/issues/3166) is stablized, this crate will be deprecated in favor of it.
- [`complete-alias`](https://github.com/cykerway/complete-alias), whose shell code helped a lot.

## ✅ TODO

- [ ] Escape special characters in generated shell code & completion candidates
- [ ] Completion delegation
    - Need to consider how to design the API
    - Prototypes available in [`prototype`](./prototype)
- [ ] Extensibility (API?)
