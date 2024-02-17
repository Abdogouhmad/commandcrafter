# CommandCrafter

CommandCrafter is a Rust crate designed to simplify interaction with the console and facilitate the creation of automated programs. It provides a convenient interface for executing commands and capturing their output. Additionally, it offers functionality to write this output to a file, enabling easy logging and further processing

## Features

- Execute commands from within Rust programs.
- Capture command output.
- Write command output to a file for logging and analysis.
- Display the output command in the terminal.
- Simplify automation tasks by leveraging Rust's capabilities.

## Installation

Add this crate to your `Cargo.toml` file:

```bash
cargo add commandcrafter
```

## Usage

```rust
use commandcrafter::Execute;

fn main() {
    // Execute a command and capture its output
    let output = Execute::new("du", &["-h", "--max-depth=1", "."]);
    
    // Write output to a file
    Execute::write_to_file(&output);
}
```

for more Usage check [docs](https://docs.rs/commandcrafter/)

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, feel free to open an issue or submit a pull request on GitHub.
