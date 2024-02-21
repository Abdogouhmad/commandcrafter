// use commandcrafter::execute::Execute;
// use commandcrafter::filestore::Filestore;
use commandcrafter::color::Fmt;
fn main() {
    let f = Fmt::coprint(&Fmt::Green, "hello");
    println!("{}", f);
}
