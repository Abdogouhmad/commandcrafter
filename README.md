# CommandCrafter

CommandCrafter is a Rust crate designed to simplify interaction with the console and facilitate the creation of automated programs. It provides a convenient interface for executing commands and capturing their output. Additionally, it offers functionality to write this output to a file, enabling easy logging and further processing

## Features

- Execute commands from within Rust programs.
- Capture command output.
- Write command output to a file for logging and analysis.
- Display the output command in the terminal.
- Simplify automation tasks by leveraging Rust's capabilities.
- storing the output in a file within log folder in desktop.
- the processing now is colorized for easy reading.
## Installation

Add this crate to your `Cargo.toml` file:

```bash
cargo add commandcrafter
```

## Usage

```rust
use commandcrafter::execute::Execute;

fn main() {
    let output = Execute::run("du", &["-h", "--max-depth=1", "."]);
    print_into_console(&output);
}
```

for more Usage check [docs](https://docs.rs/commandcrafter/0.2.2/commandcrafter/)

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, feel free to open an issue or submit a pull request on GitHub.
