# Examples

To run an example, you should:

- Compile it with:
    ```bash
    cargo build --example <example_name>
    jiu e <example_name> # Alternative
    ```
- Setup the completion:
    ```bash
    source <(COMPLETE=bash ./target/debug/examples/<example_name>)
    ```
- Run the example:
    ```bash
    ./target/debug/examples/<example_name> <TAB>
    ```

Here's a list of examples you can try out:

- [`wordlist`](./wordlist.rs): Completion for a provided wordlist.
    - Returns candidate words that start with the query.
- [`number`](./number.rs): Completion for numbers.
    - Returns candidate words that start with the query, or the English word if the query is a number.
    - Example 1: `z` -> `zero`, `f` -> `four`/`five` etc.
    - Example 2: `0` -> `zero`, `1` -> `one`, `2` -> `two`, etc.
- [`delegate`](./delegate.rs): Completion delegation.
    - Delegates `./target/debug/examples/delegate` to `cargo build --example`
    - You'll need to [setup `cargo`'s completion](https://rust-lang.github.io/rustup/installation/index.html?highlight=Comple#enable-tab-completion-for-bash-fish-zsh-or-powershell) first, if not already
    - Example 1: Completion at the end
        ```bash
        ./target/debug/examples/delegate del<TAB>
        ```
    - Example 2: Completion in the middle
        ```bash
        ./target/debug/examples/delegate del<TAB> --example wordlist
        ```
