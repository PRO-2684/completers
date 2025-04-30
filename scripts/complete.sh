_my_completion_function() {
    local IFS=$'\n'
    COMPREPLY=($(COMPLETE=1 my_command "$COMP_CWORD" "$COMP_LINE" "$COMP_POINT" "$COMP_TYPE" "$COMP_KEY" "${COMP_WORDS[@]}"))
}
complete -F _my_completion_function completers
