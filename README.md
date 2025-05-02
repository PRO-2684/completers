# completers

[![GitHub License](https://img.shields.io/github/license/PRO-2684/completers?logo=opensourceinitiative)](https://github.com/PRO-2684/completers/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/completers/release.yml?logo=githubactions)](https://github.com/PRO-2684/completers/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/completers?logo=githubactions)](https://github.com/PRO-2684/completers/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/completers/total?logo=github)](https://github.com/PRO-2684/completers/releases)
[![Crates.io Version](https://img.shields.io/crates/v/completers?logo=rust)](https://crates.io/crates/completers)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/completers?logo=rust)](https://crates.io/crates/completers)
[![docs.rs](https://img.shields.io/docsrs/completers?logo=rust)](https://docs.rs/completers)

A tiny Rust-native shell completion solution.

<!-- ## 📥 Installation

### Using [`binstall`](https://github.com/cargo-bins/cargo-binstall)

```bash
cargo binstall completers
```

### Downloading from Releases

Navigate to the [Releases page](https://github.com/PRO-2684/completers/releases) and download respective binary for your platform. Make sure to give it execute permissions.

### Compiling from Source

```bash
cargo install completers
``` -->

## 💡 Examples

See [`examples`](./examples/README.md) for a few examples of how to use this crate.

## 📖 Usage

### Rust Part

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

### Shell Part

Generate and evaluate the shell code via:

```bash
source <(COMPLETE=bash my_binary)
```

You should be able to complete your commands now. To enable completion across all your terminal sessions, you can add the above code to your completions directory, like:

```bash
mkdir -p ~/.local/share/bash-completion/completions # Create the directory if it doesn't exist
echo 'source <(COMPLETE=bash my_binary)' > ~/.local/share/bash-completion/completions/my_binary
```

### The `completers` Binary

Currently, the `completers` binary does nothing.

## ⚙️ Mechanism

See [`MECHANISM.md`](doc/MECHANISM.md) for a detailed explanation of how this works, in case you're curious.

## 🎉 Credits

- [`clap`](https://github.com/clap-rs/clap), whose code and API is used as a reference. When `clap`'s [Rust-Native Completion Engine](https://github.com/clap-rs/clap/issues/3166) is stablized, this crate will be deprecated in favor of it.
- [`complete-alias`](https://github.com/cykerway/complete-alias), whose shell code helped a lot.

## ✅ TODO

- [ ] Escape special characters in generated shell code & completion candidates
