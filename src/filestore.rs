#![allow(dead_code)]
//! Store data
//! This module is used to replace a method I created in
//! Execute that intends to store output of commands inside
//! a file named `ExecuteLog.lg` [check method](https://docs.rs/commandcrafter/0.2.2/commandcrafter/struct.Execute.html#method.write_to_file).
//! Therefore, this module will has some functionalities such
//! - write the output inside a file named `ExecuteLog.lg`
//! - check if the file exists
//! - store the file inside a folder (usually the home/Desktop directory)
//! - combine multiple outputs into one file
//! - the ability to delete the file (optional)
