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
const FILE_NAME: &str = "ExecuteLog.lg";
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
    /// let output = Execute::run("du", &["-h", "--max-depth=1", "."]);
    /// println!("{}", String::from_utf8_lossy(&output));
    /// ```
    pub fn run(command: &str, arguments: &[&str]) -> Vec<u8> {
        // execute the command and return the output
        let output = Command::new(command)
            .args(arguments)
            .stdout(Stdio::piped())
            .spawn();

        let child = match output {
            Ok(child) => child,
            Err(err) => {
                panic!("Failed to execute command '{}': {}", command, err);
            }
        };

        let result = child.wait_with_output();
        match result {
            Ok(output) => output.stdout,
            Err(err) => {
                panic!("Failed to wait for command '{}': {}", command, err);
            }
        }
    }

    /// format method is used to display the output in the console
    /// it will be used only when we want to display the output in the console
    /// # Example
    /// ```rust
    /// use commandcrafter::execute::Execute;
    /// let out = Execute::run("ls", &["-l"]);
    /// let out1 = Execute::run("ping", &["-c", "5", "8.8.8.8"]);
    /// let cmb = &[out, out1].concat();
    /// Execute::print_into_console(cmb);
    /// ```
    pub fn print_into_console(output: &[u8]) {
        let formatting = format!("{}", String::from_utf8_lossy(output));
        println!("{}", formatting);
    }
    /// # Example
    ///```rust
    /// use commandcrafter::execute::Execute;
    /// let out = Execute::run("ls", &["-l"]);
    /// let out1 = Execute::run("ping", &["-c", "5", "8.8.8.8"]);
    /// let cmb = &[out, out1].concat();
    /// let _ = Execute::write_to_file(cmb);
    /// ```
    pub fn write_to_file(content: &[u8]) -> std::io::Result<()> {
        let mut file = File::create(FILE_NAME)?;
        file.write_all(content)?;
        Ok(())
    }

    /// this module comes to check the existence of the file: `ExecuteLog.log`
    /// as the existence of the file will show us that the command is executed successfully
    pub fn check_operation(op: &std::io::Result<()>) -> bool {
        match op {
            Ok(_) => {
                println!("File created successfully in {}.", FILE_NAME);
                true
            }
            Err(_) => {
                panic!("Failed to create file {}.", FILE_NAME);
            }
        }
    }

}
