_wordlist_completion() {
    local IFS=$'\n'
    COMPREPLY=($(COMPLETE=1 ./target/debug/examples/wordlist "$COMP_CWORD" "$COMP_LINE" "$COMP_POINT" "$COMP_TYPE" "$COMP_KEY" "${COMP_WORDS[@]}"))
}
complete -F _wordlist_completion ./target/debug/examples/wordlist
