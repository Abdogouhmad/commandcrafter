use commandcrafter::execute::Execute;

fn main() {
    let l_flag = ["-la"];
    let p_flag = ["-c", "2", "8.8.8.8"];
    let l = Execute::run("ls", &l_flag);
    let p = Execute::run("ping", &p_flag);
    let o = vec![l, p];
    let _ = Execute::write_combined_to_file(&o);
}
