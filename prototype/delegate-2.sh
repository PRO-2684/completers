_delegated_2() {
    # Clear log.txt
    > log.txt

    # Log current environment
    echo "== Initial ==" >> log.txt
    _delegated_2_log

    # Set `COMP_WORDS` to ["cargo", "build". "--example", input-words...]
    COMP_WORDS=(
        "cargo"
        "build"
        "--example"
        "${COMP_WORDS[@]:1}"
    )
    # Offset `COMP_CWORD` by 2 (3 - 1, 3 for ["cargo", "build", "--example"], one for "bar")
    COMP_CWORD=$((COMP_CWORD + 2))

    # Save length of `COMP_LINE` for later use
    local len=${#COMP_LINE}
    # Replace "bar" with "cargo build --example" in `COMP_LINE`
    COMP_LINE="${COMP_LINE//bar/cargo build --example}"
    # Set `COMP_POINT`, by adding the difference between the length of updated `COMP_LINE` and `len`
    COMP_POINT=$((COMP_POINT + ${#COMP_LINE} - len))

    # Log the updated environment
    echo "== Updated ==" >> log.txt
    _delegated_2_log

    # Delegate completion, from https://github.com/cykerway/complete-alias/blob/7f2555c2fe7a1f248ed2d4301e46c8eebcbbc4e2/complete_alias#L834-L840
    _command_offset 0

    # Log the final environment
    echo "== Final ==" >> log.txt
    _delegated_2_log
}

_delegated_2_log() {
    # Log the current environment
    echo "COMP_WORDS:" >> log.txt
    for word in "${COMP_WORDS[@]}"; do
        echo "$word" >> log.txt
    done
    echo "COMP_CWORD: $COMP_CWORD" >> log.txt
    echo "COMP_LINE: $COMP_LINE" >> log.txt
    echo "COMP_POINT: $COMP_POINT" >> log.txt
    # We don't care about the following variables
    # echo "COMP_TYPE: $COMP_TYPE" >> log.txt
    # echo "COMP_KEY: $COMP_KEY" >> log.txt
}

complete -F _delegated_2 bar
# Source this file, and then try:
# bar <example>
# Example:
# bar wor<TAB>
# Completion will be delegated to `cargo build --example`,
# so you'd get `cargo build --example wordlist` as a completion.
