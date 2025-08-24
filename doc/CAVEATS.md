# Caveats

## Common

Since `completers` uses line break (`\n`) to split candidates, it is not possible to include this character in your candidates or delegation requests.

## Bash

N/A

## Nushell

- Completion only works for `my_binary`, and not `./my_binary` or `/path/to/my_binary`.
- Completion delegation is not supported yet.
- Lazy loading blocked by nushell/nushell#4874.
- Expect argument splitting issues when spaces are present.
