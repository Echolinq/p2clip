# Rust Clipboard Utility

This is a simple command-line utility written in Rust that reads input from the standard input (stdin), removes any newline characters, and copies the resulting string to the system clipboard.

## Dependencies

- `std::io` and `std::io::Read` for reading from stdin.
- `arboard::Clipboard` for interacting with the system clipboard.
- `regex::Regex` for handling regular expressions.

## How to Use

1. Compile the program using Rust's `cargo build` command.
2. Run the compiled binary and provide your input. The input will be read until EOF (End of File) is encountered.
3. The input is then processed to remove any newline characters and copied to the system clipboard.

## Error Handling

The program includes basic error handling for issues that might occur while reading from stdin or interacting with the clipboard.

## Future Improvements

- Add support for command-line arguments to provide more flexibility in how the program is used.
- Improve error handling and provide more informative error messages.