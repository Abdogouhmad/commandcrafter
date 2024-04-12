use commandcrafter::execute::Execute;

fn main() {
    let l_flag = ["-la"];
    let l = Execute::run("ls", &l_flag);
    Execute::print_into_console(l);
}
