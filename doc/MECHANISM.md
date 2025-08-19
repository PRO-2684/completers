# Mechanism

## Shell Completion

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

## Integration with Rust

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

## Completion Delegation

Sometimes, we just want to *delegate* the completion request to another command. One of the well-known examples is `sudo`:

```bash
sudo apt upd<TAB>
```

To complete the command, the completion provider of `apt` must be invoked as if we've typed:

```bash
apt upd<TAB>
```

Which provides a single completion candidate `update`. Thus we get:

```bash
sudo apt update
```

Under the hood, it uses a function named `_command_offset` to "delegate" to the provided completion request[^1]. So, just like we've [sent relevant variables to our binary via commandline arguments](#integration-with-rust), we will now print the delegated completion request in the very same order:

```rust
// Construct the delegated completion request
...
// Print arguments
println!("{COMP_CWORD}");
println!("{COMP_LINE}");
println!("{COMP_POINT}");
println!("{COMP_TYPE}");
println!("{COMP_KEY}");
// Print the words to stdout, separated by newlines
for word in COMP_WORDS {
    println!("{word}");
}
```

Then collect and spread them back to their place in our completion function:

```bash
OUTPUT=($(COMPLETE=1 my_binary "$COMP_CWORD" "$COMP_LINE" "$COMP_POINT" "$COMP_TYPE" "$COMP_KEY" "${COMP_WORDS[@]}"))
COMP_CWORD=${OUTPUT[0]}
COMP_LINE=${OUTPUT[1]}
COMP_POINT=${OUTPUT[2]}
COMP_TYPE=${OUTPUT[3]}
COMP_KEY=${OUTPUT[4]}
COMP_WORDS=("${OUTPUT[@]:5}")
```

Finally, call `_command_offset` to delegate the request[^2]:

```bash
_command_offset 0
```

## Putting It All Together

So now we've got two choices regarding our completion result:

1. We provide candidates directly
2. We want to *delegate* to another completion request

How do we differentiate it? Simply adding one more line in the output will do. When we provide candidates directly, we can print the magic word `COMPLETERS_COMPLETE` as the first line of output; otherwise, if we want to delegate, we print `COMPLETERS_DELEGATE`. Then, a simple switch-case in our completion function will do. Refer to [`bash.tmpl`](/src/templates/bash.tmpl) and [`lib.rs`](/src/lib.rs) for the full implementation.

---

[^1]: Newer versions of `bash-completion` [uses `_comp_command_offset` instead](https://github.com/scop/bash-completion/blob/c55ee7f6fb75300786cb522261f68eb80366c41f/completions/sudo#L17), but has provided [compatible implementation of `_command_offset`](https://github.com/scop/bash-completion/blob/c55ee7f6fb75300786cb522261f68eb80366c41f/bash_completion.d/000_bash_completion_compat.bash#L281-L292) on top of it.
[^2]: "The documentation does not say it will work with argument `0`, but looking at its code (version 2.11) it should." ([ref](https://github.com/cykerway/complete-alias/blob/7f2555c2fe7a1f248ed2d4301e46c8eebcbbc4e2/complete_alias#L834-L840))
