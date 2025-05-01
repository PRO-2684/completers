_delegated() {
    > log.txt
    for arg in "${COMP_WORDS[@]}"; do
        echo $arg >> log.txt
    done
    _command_offset 1;
}
complete -F _delegated foo
# Source this file, and then try:
# foo <any-command>
# Example:
# foo git br<TAB>
# Completion will be delegated to the command you specify
