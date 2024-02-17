//! # Execute module ?
//! The rational behind creating this crate, is to make life easier
//! for Rust developers when it comes to interacting with your console. Additionally,
//! using our crate will help you with the process of creating automated programs
//! instead of using other languages.
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
pub struct Execute;
/// this const is intend to hold the name of the file where we will store our output
const FILE_NAME: &str = "ExecuteLog.lg";
/// Implement the **Execute struct**
impl Execute {
    /// # Example
    /// ```rust
    /// use commandcrafter::Execute;
    /// // be aware that the command will be executed in the current directory
    /// // also make sure that the command is exists + the arguments are correct
    /// // they should be splitted accordingly
    /// let output = Execute::new("du", &["-h", "--max-depth=1", "."]);
    /// // you can display the output in the console or store it as file
    /// println!("{}", String::from_utf8_lossy(&output));
    /// ```
    #[allow(dead_code)]
    pub fn new(command: &str, arguments: &[&str]) -> Vec<u8> {
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
    /// use commandcrafter::Execute;
    /// let output = Execute::new("du", &["-h", "--max-depth=1", "."]);
    /// Execute::into_console(&output);
    /// ```
    #[allow(dead_code)]
    pub fn into_console(output: &Vec<u8>) {
        let formatting = format!("{}", String::from_utf8_lossy(output));
        println!("{}", formatting);
    }

    /// # Example
    ///```
    /// use commandcrafter::Execute;
    ///
    /// fn main() {
    ///     let output1 = Execute::new("ls", &["-ll"]);
    ///    let output2 = Execute::new("du", &["-h", "--max-depth=1", "."]);
    ///    // combine the outputs into one file
    ///   let combineoutput = &[output1, output2].concat();
    ///   Execute::write_to_file(&combineoutput);
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn write_to_file(content: &Vec<u8>) -> std::io::Result<()> {
        let mut file = File::create(FILE_NAME)?;
        file.write_all(&content)?;
        Ok(())
    }

    /// this module comes to check the existence of the file : `ExecuteLog.log`
    /// as the existence of the file will show us that the command is executed successfully
    #[allow(dead_code)]
    pub fn check_operation(op: &std::io::Result<()>) -> bool {
        match op {
            Ok(_) => {
                println!("File created successfully in {}.", FILE_NAME);
                true
            }
            Err(_) => {
                false;
                panic!("Failed to create file {}.", FILE_NAME);
            }
        }
    }
}
