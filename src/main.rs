#![allow(unused_imports)]
use commandcrafter::{execute::Execute, filestore::Filestore};

fn main() {
    match Execute::exe("lk", &["--all"]) {
        Ok(_) => println!("Command executed successfully."),
        Err(e) => eprintln!("Error executing command: {}", e),
    }

    let _ = Execute::exe("pacman", &["-Qu", "--color=always"]);
    let _ = Execute::exe("yay", &["-Qu", "--color=always"]);
}
