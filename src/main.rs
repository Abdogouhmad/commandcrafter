use commandcrafter::execute::Execute;
use commandcrafter::filestore::Filestore;

fn main() {
    let out = Execute::run("ls", &["-l"]);

    Filestore::write_into_desktop(&out).unwrap();
}
