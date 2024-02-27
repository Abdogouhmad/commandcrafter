# CommandCrafter

CommandCrafter is a Rust crate designed to simplify interaction with the console and facilitate the creation of automated programs. It provides a convenient interface for executing commands and capturing their output. Additionally, it offers functionality to write this output to a file, enabling easy logging and further processing

## Features

- Execute commands from within Rust programs.
- Capture command output.
- Write command output to a file for logging and analysis.
- Display the output command in the terminal.
- Simplify automation tasks by leveraging Rust's capabilities.
- Store the output in a file within the log folder on the desktop.
- The processing now is colorized for easy reading.
- The progress of program running displayed in a colorized way.

## Installation

Add this crate to your `Cargo.toml` file:

```bash
cargo add commandcrafter
```

## Example

for normal use check this example:

```rust
use commandcrafter::execute::Execute;

fn main() {
    let output = Execute::run("du", &["-h", "--max-depth=1", "."]);
    Execute::print_into_console(output);
}
```

for more Usage check [docs](https://docs.rs/commandcrafter/0.3.2/commandcrafter/)

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, feel free to open an issue or submit a pull request on GitHub.
