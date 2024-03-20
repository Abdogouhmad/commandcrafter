#![allow(unused_imports)]
use commandcrafter::{execute::Execute, filestore::Filestore};

fn main() {
    let out = Execute::run("ls", &["-l"]);
    let _ = Filestore::write_into_desktop(&out, "/ls.log");
}
