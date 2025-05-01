# completers

[![GitHub License](https://img.shields.io/github/license/PRO-2684/completers?logo=opensourceinitiative)](https://github.com/PRO-2684/completers/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/completers/release.yml?logo=githubactions)](https://github.com/PRO-2684/completers/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/completers?logo=githubactions)](https://github.com/PRO-2684/completers/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/completers/total?logo=github)](https://github.com/PRO-2684/completers/releases)
[![Crates.io Version](https://img.shields.io/crates/v/completers?logo=rust)](https://crates.io/crates/completers)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/completers?logo=rust)](https://crates.io/crates/completers)
[![docs.rs](https://img.shields.io/docsrs/completers?logo=rust)](https://docs.rs/completers)

A tiny Rust-native shell completion solution.

## ⚙️ Automatic Releases Setup

1. [Create a new GitHub repository](https://github.com/new) with the name `completers` and push this generated project to it.
2. Enable Actions for the repository, and grant "Read and write permissions" to the workflow [here](https://github.com/PRO-2684/completers/settings/actions).
3. [Generate an API token on crates.io](https://crates.io/settings/tokens/new), with the following setup:

    - `Name`: `completers`
    - `Expiration`: `No expiration`
    - `Scopes`: `publish-new`, `publish-update`
    - `Crates`: `completers`

4. [Add a repository secret](https://github.com/PRO-2684/completers/settings/secrets/actions) named `CARGO_TOKEN` with the generated token as its value.
5. Consider removing this section and updating this README with your own project information.

## 📥 Installation

### Using [`binstall`](https://github.com/cargo-bins/cargo-binstall)

```bash
cargo binstall completers
```

### Downloading from Releases

Navigate to the [Releases page](https://github.com/PRO-2684/completers/releases) and download respective binary for your platform. Make sure to give it execute permissions.

### Compiling from Source

```bash
cargo install completers
```

## 💡 Examples

TODO

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

### Shell Completion

In Bash, you can designate a shell function as the completion provider using the `complete` command:

```bash
complete -F _my_completion_function my_binary
```

Or you can designate a command using the `-C` option:

```bash
complete -C _my_completion_command my_binary
```

Then, when the user types:

```bash
my_binary <TAB>
```

Bash will call the completion provider with [relevant variables set](https://www.gnu.org/software/bash/manual/html_node/Programmable-Completion.html#:~:text=When%20the%20command%20or%20function%20is%20invoked%2C%20the%20COMP_LINE%2C%20COMP_POINT%2C%20COMP_KEY%2C%20and%20COMP_TYPE%20variables%20are%20assigned%20values%20as%20described%20above%20(see%20Bash%20Variables).%20If%20a%20shell%20function%20is%20being%20invoked%2C%20the%20COMP_WORDS%20and%20COMP_CWORD%20variables%20are%20also%20set), including:

- `COMP_WORDS`: An array of all the words in the command line, including the command itself. (If the completion provider is a shell function)
- `COMP_CWORD`: The index of the word in `COMP_WORDS` that is currently being completed. (If the completion provider is a shell function)
- `COMP_LINE`: The current command line.
- `COMP_POINT`: The index of the current cursor position relative to the beginning of the current command.
- `COMP_TYPE`: Set to an integer value corresponding to the type of completion attempted that caused a completion function to be called.
- `COMP_KEY`: The key (or final key of a key sequence) used to invoke the current completion function.

If the completion provider is a shell function, it should set the `COMPREPLY` variable to an array of completion candidates. If the completion provider is a command, it should print the completion candidates to `stdout` line by line. Bash will then display the candidates to the user, or directly complete the command line if there is only one candidate.

### Integration with Rust

Consider the following code snippet:

```bash
_my_completion_function() {
    local IFS=$'\n'
    COMPREPLY=($(COMPLETE=1 my_binary "$COMP_CWORD" "$COMP_LINE" "$COMP_POINT" "$COMP_TYPE" "$COMP_KEY" "${COMP_WORDS[@]}"))
}
complete -F _my_completion_function my_binary
```

When the user types `my_binary <TAB>`, Bash will call `_my_completion_function` with the relevant variables set. The function will then spread these values as command line arguments to `my_binary` with `COMPLETE` environment variable set. `my_binary`, on seeing the `COMPLETE` environment variable, will parse the arguments, determine completion candidates, and print them line by line to `stdout`. After receiving the output, `_my_completion_function` will then split the output line by line and set the completion candidates using `COMPREPLY` variable.

Note that the candidates are determined by `my_binary` itself, not by a separate shell function. This allows for so-called Rust-native completion.

## 🎉 Credits

- [`clap`](https://github.com/clap-rs/clap), whose code and API is used as a reference. When `clap`'s [Rust-Native Completion Engine](https://github.com/clap-rs/clap/issues/3166) is stablized, this crate will be deprecated in favor of it.

## ✅ TODO

- [ ] Escape special characters in generated shell code & completion candidates
