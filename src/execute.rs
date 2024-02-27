#![allow(dead_code)]
//! # Execute module
//! The rationale behind creating this crate, is to make life easier
//! for Rust developers when it comes to interacting with your console. Additionally,
//! using our crate will help you with the process of creating automated programs
//! instead of using other languages.

use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};

pub struct Execute;
/// this const is intended to hold the name of the file where we will store our output
const FILE_NAME: &str = "ExecuteLog.log";
/// Implement the **Execute struct**
impl Execute {
    /// # Execute::run;
    /// this method is used to execute the command and return the output
    /// ## Arguments
    /// * `command` - the name of the command
    /// * `arguments` - the arguments of the command to be executed
    /// ## Returns
    /// the output of the command as a vector of bytes
    /// ###  Example
    /// ```rust
    /// use commandcrafter::execute::Execute;
    /// let out = Execute::run("ls", &["-l"]);
    /// if out.is_ok() {
    ///     Execute::print_into_console(out);
    /// } else {
    ///     eprintln!("Error {}", out.unwrap_err());
    ///     std::process::exit(1)
    /// }
    /// ```
    pub fn run(command: &str, arguments: &[&str]) -> Result<Vec<u8>, String> {
        // execute the command and return the output
        let output = Command::new(command)
            .args(arguments)
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| format!("failed to execute cmd '{}' : {}", command, e))?;

        let result = output
            .wait_with_output()
            .map_err(|err| format!("Failed to wait for command '{}': {}", command, err))?;

        if result.status.success() {
            Ok(result.stdout)
        } else {
            Err(format!(
                "Command failed with exit code {}: {}",
                result.status.code().unwrap_or_default(),
                String::from_utf8_lossy(&result.stderr)
            ))
        }
    }
    /// Displays the output of a shell command in the console.
    ///
    /// # Arguments
    ///
    /// * `output` - The output of the command as a `Result<Vec<u8>, String>`.
    ///              If the result is `Ok`, it contains the output data as a vector of unsigned 8-bit integers (bytes).
    ///              If the result is `Err`, it contains the error message as a string.
    ///
    /// # Returns
    ///
    /// This function does not return anything, it only prints the output into the console.
    ///
    /// # Example
    ///
    /// ```
    /// use commandcrafter::execute::Execute;
    ///
    /// // Run a shell command
    /// let out = Execute::run("ls", &["-l"]);
    ///
    /// // Print the output
    /// Execute::print_into_console(out);
    /// ```
    pub fn print_into_console(output: Result<Vec<u8>, String>) {
        match output {
            Ok(bytes) => {
                let formatting = format!("{}", String::from_utf8_lossy(&bytes));
                println!("{}", formatting);
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    /// Print the outputs of multiple shell commands into the console.
    ///
    /// This function takes a vector of `Result<Vec<u8>, String>` representing the outputs of shell commands.
    /// For each output, it prints the content to the console if it's successful, otherwise, it prints the error message.
    ///
    /// # Arguments
    ///
    /// * `outputs` - A vector of `Result<Vec<u8>, String>` representing the outputs of shell commands.
    ///
    /// # Examples
    ///
    /// ```
    /// use commandcrafter::execute::Execute;
    ///
    /// let out1 = Execute::run("ls", &["-l"]);
    /// let out2 = Execute::run("ping", &["-c", "5", "8.8.8.8"]);
    /// let outputs = vec![out1, out2];
    /// Execute::print_into_console_multiple(outputs);
    /// ```
    pub fn print_into_console_multiple(outputs: Vec<Result<Vec<u8>, String>>) {
        for output in outputs {
            Execute::print_into_console(output)
        }
    }
    /// Write the output of a shell command to a file.
    ///
    /// # Arguments
    ///
    /// * `content` - The output of the command (could be a combination of commands) as a `Result<Vec<u8>, String>`.
    ///               If the result is `Ok`, it contains the output data as a vector of unsigned 8-bit integers (bytes).
    ///               If the result is `Err`, it contains the error message as a string.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<()>` - The result of writing the command output into the file named `ExecuteLog.lg`.
    ///
    /// # Example
    ///```rust
    /// use commandcrafter::execute::Execute;
    /// let out = Execute::run("ls", &["-l"]);
    /// let _ = Execute::write_to_file(&out);
    /// ```
    pub fn write_to_file(content: &Result<Vec<u8>, String>) -> std::io::Result<()> {
        match content {
            Ok(output_cmd) => {
                let mut file = File::create(FILE_NAME)?;
                file.write_all(output_cmd)?;
            }
            Err(e) => println!("Error {}", e),
        }
        Ok(())
    }

    /// Write the combined output of shell commands to a file.
    ///
    /// This function takes a vector of `Result<Vec<u8>, String>` representing the outputs of shell commands.
    /// It iterates over each output and writes it to the file named `ExecuteLog.lg` using the `write_to_file` function.
    ///
    /// # Arguments
    ///
    /// * `outputs` - A vector of `Result<Vec<u8>, String>` representing the outputs of shell commands.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<()>` - The result of writing the combined command outputs into the file named `ExecuteLog.lg`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use commandcrafter::execute::Execute;
    ///
    /// // Run two shell commands
    /// let out1 = Execute::run("ls", &["-l"]);
    /// let out2 = Execute::run("ping", &["-c", "2", "8.8.8.8"]);
    ///
    /// // Combine the outputs into a single vector
    /// let combined_outputs = &[out1, out2];
    ///
    /// // Write the combined outputs to a file
    /// let _ = Execute::write_combined_to_file(combined_outputs).unwrap();
    /// ```

    pub fn write_combined_to_file(outputs: &[Result<Vec<u8>, String>]) -> std::io::Result<()> {
        // Open the file in append mode or create it if it doesn't exist
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(FILE_NAME)?;

        // Write each output to the file
        for (i, output) in outputs.iter().enumerate() {
            match output {
                Ok(output_cmd) => {
                    file.write_all(output_cmd)?;
                }
                Err(e) => {
                    // Handle error
                    println!("Error: {}", e);
                }
            }

            // Insert a newline character after each command output unless it's the last command
            if i < outputs.len() - 1 {
                file.write_all(b"\n")?;
            }
        }

        Ok(())
    }
    /// `check_operation` Check the operation of the file
    /// # Arguments
    /// * `op` - the result of file creation
    /// # Returns
    /// * `true` if the file is created successfully
    /// * `panic` if the file is not created successfully
    /// ##  Example
    /// ```rust
    /// use commandcrafter::execute::Execute;
    /// let out = Execute::run("ls", &["-l"]);
    /// let out1 = Execute::run("ping", &["-c", "5", "8.8.8.8"]);
    /// let cmb = &[out, out1];
    /// let res = Execute::write_combined_to_file(cmb);
    /// Execute::check_operation(&res);
    /// ```
    pub fn check_operation(op: &std::io::Result<()>) -> bool {
        match op {
            Ok(_) => {
                // println!("File created successfully in {}.", FILE_NAME);
                println!("File created successfully in {}.", FILE_NAME);
                true
            }
            Err(_) => {
                panic!("Failed to create file {}.", FILE_NAME);
            }
        }
    }
}
