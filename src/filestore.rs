#![allow(dead_code)]
//! Store data
//! This module is used to replace a method I created in
//! Execute that intends to store output of commands inside
//! a file named `ExecuteLog.lg` [check method](https://docs.rs/commandcrafter/0.2.2/commandcrafter/execute/struct.Execute.html#method.write_to_file).
//! Therefore, this module will has some functionalities such
//! - write the output inside a file named `ExecuteLog.lg`
//! - check if the file exists
//! - store the file inside a folder (usually the home/Desktop directory)
//! - combine multiple outputs into one file
//! - the ability to delete the file (optional)
use crate::color::Col;
use std::env;
use std::fs;
use std::io::Write;
/// struct for filestore implementation
pub struct Filestore;

impl Filestore {
    /// Store the output of a shell command into the folder `desktop/log/ExecuteLog`.
    ///
    /// This method creates a folder named `logs` on the user's desktop (if it doesn't already exist)
    /// and stores the output of the command into a file named `ExecuteLog.lg` within that folder.
    ///
    /// # Arguments
    ///
    /// * `content` - The output of the command as a `Result<Vec<u8>, String>`.
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
    /// use commandcrafter::filestore::Filestore;
    ///
    /// let out = Execute::run("ls", &["-l"]);
    /// Filestore::write_into_desktop(&out).unwrap();
    /// ```
    pub fn write_into_desktop(content: &Result<Vec<u8>, String>) -> std::io::Result<()> {
        let log_folder = env::var("HOME").unwrap() + "/Desktop/logs";
        // Create the folder
        match fs::create_dir_all(&log_folder) {
            Ok(_) => {
                println!(
                    "{}",
                    Col::print_col(&Col::Green, "The folder was created successfully")
                );
                let file_log = log_folder + "/ExecuteLog.log";
                let fl = fs::File::create(&file_log);
                match fl {
                    Ok(mut f) => {
                        println!(
                            "{}: {}",
                            Col::print_col(
                                &Col::Green,
                                "The file log was created successfully with in"
                            ),
                            file_log
                        );

                        // match the content of the command output
                        match content {
                            Ok(output_cmd) => {
                                f.write_all(output_cmd)?;
                            }
                            Err(e) => println!("Error {}", e),
                        }
                        // f.write_all(content)?;
                    }
                    Err(e) => {
                        println!(
                            "{}: {}",
                            Col::print_col(&Col::Red, "The file couldn't be created"),
                            e
                        );
                    }
                }
            }
            Err(e) => {
                println!("Couldn't create the folder: {}", e);
            }
        }

        Ok(())
    }
    /// Write the output of a shell command to a file on the desktop.
    ///
    /// This method creates a folder named `logs` on the user's desktop (if it doesn't already exist)
    /// and stores the output of the command into a file named `ExecuteLog.lg` within that folder.
    ///
    /// # Arguments
    ///
    /// * `content` - The output of the command as a `Result<Vec<u8>, String>`.
    ///               If the result is `Ok`, it contains the output data as a vector of unsigned 8-bit integers (bytes).
    ///               If the result is `Err`, it contains the error message as a string.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<()>` - The result of writing the command output into the file named `ExecuteLog.lg`.
    ///
    /// # Example
    /// ```rust
    /// use commandcrafter::{filestore::Filestore, execute::Execute} ;
    ///
    /// // Run two shell commands
    /// let out1 = Execute::run("ls", &["-l"]);
    /// let out2 = Execute::run("ping", &["-c", "2", "8.8.8.8"]);
    ///
    /// // Combine the outputs into a single vector
    /// let combined_outputs = &[out1, out2];
    ///
    /// // Write the combined outputs to the log file on the desktop
    /// let _ = Filestore::write_combined_to_desktop_log(combined_outputs).unwrap();
    /// ```
    pub fn write_combined_to_desktop_log(
        outputs: &[Result<Vec<u8>, String>],
    ) -> std::io::Result<()> {
        let log_folder = env::var("HOME").unwrap() + "/Desktop/logs";

        // Create the folder
        match fs::create_dir_all(&log_folder) {
            Ok(_) => {
                println!(
                    "{}",
                    Col::print_col(&Col::Green, "The folder was created successfully")
                );
                let file_log = log_folder + "/ExecuteLog.log";
                let mut f = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&file_log)?;
                println!(
                    "{}: {}",
                    Col::print_col(&Col::Green, "The file log was created successfully within"),
                    file_log
                );

                // Iterate over the content and match the content of the command output
                for output in outputs {
                    match output {
                        Ok(out_cmd) => {
                            f.write_all(out_cmd)?;
                            // Add a newline character after each output
                            writeln!(f)?;
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Couldn't create the folder: {}", e);
            }
        }

        Ok(())
    }
}
