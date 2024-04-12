use commandcrafter::execute::Execute;

fn main() {
    let l_flag = ["-la"];
    let l = Execute::run("ls", &l_flag);
    let _ = Execute::write_to_file(&l);
}
