#![allow(unused_imports)]
use commandcrafter::{execute::Execute, filestore::Filestore};

fn main() {
    Execute::exe("duff", &["--all"]).unwrap();
}
