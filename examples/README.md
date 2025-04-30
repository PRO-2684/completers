# Examples

To run an example, you should:

- Compile it with:
    ```bash
    cargo build --example <example_name>
    jiu e <example_name> # Alternative
    ```
- Setup the completion:
    ```bash
    source ./examples/<example_name>.sh
    ```
- Run the example:
    ```bash
    ./target/debug/examples/<example_name> <TAB>
    ```
