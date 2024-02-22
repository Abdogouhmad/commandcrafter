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
    /// `write_into_desktop` method will store the output command into the folder `desktop/log/ExecuteLog`
    /// # Arguments
    /// * `output` - the output of the command (could be combination of commands) of type array of unsigned 8-bit
    /// # Returns
    /// * `std::io::Result<()>`  the result of the command wrote into file named **ExecuteLog.lg**
    /// # Example
    ///```rust
    /// use commandcrafter::execute::Execute;
    /// use commandcrafter::filestore::Filestore;
    ///
    /// let out = Execute::run("ls", &["-l"]);
    /// let out1 = Execute::run("ping", &["-c", "5", "8.8.8.8"]);
    /// let cmb = &[out, out1].concat();
    /// Filestore::write_into_desktop(&cmb).unwrap();
    /// ```
    pub fn write_into_desktop(content: &[u8]) -> std::io::Result<()> {
        let log_folder = env::var("HOME").unwrap() + "/Desktop/logs";
        // Create the folder
        match fs::create_dir_all(&log_folder) {
            Ok(_) => {
                println!(
                    "{}",
                    Col::print_col(&Col::Green, "The folder was created successfully")
                );
                let file_log = log_folder + "/ExecuteLog.lg";
                let fl = fs::File::create(&file_log);
                match fl {
                    Ok(mut f) => {
                        println!(
                            "{}: {}",
                            Col::print_col(&Col::Green, "The file log was created successfully with in"),
                            file_log
                        );
                        f.write_all(content)?;
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
}
