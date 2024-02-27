#![allow(unused_imports)]
use commandcrafter::{execute::Execute, filestore::Filestore};

fn main() {
    let out = Execute::run("ls", &["-l"]);
    let out1 = Execute::run("ping", &["-c", "2", "8.8.8.8"]);
    let cmb = &[out, out1];
    Filestore::write_combined_to_desktop_log(cmb).unwrap();
}
