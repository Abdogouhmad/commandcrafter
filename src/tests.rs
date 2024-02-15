#![cfg(test)]

use super::execute::Execute;

#[test]
/// this test is intends to execute a command
fn execute_command() {
    let output: Vec<u8> = Execute::new("pwd", &["--version"]);
    assert_eq!(output, Execute::new("pwd", &["--version"]));
}
#[test]
fn execute_command_error() {
    let output: Vec<u8> = Execute::new("ls", &["-ll"]);
    assert_ne!(output, Execute::new("pwd", &["--version"]));
}

#[test]
/// this test is intends to execute a command and store it inside a file `ExecuteLog.log`
pub fn store_inside_file() {
    let output = Execute::new("pwd", &["--version"]);
    let res = Execute::write_to_file(&output);
    assert_eq!(res.is_ok(), true);
}

#[test]
#[should_panic]
/// this test is intends to execute a wrong command and without storing it inside a file `ExecuteLog.log`
fn wrong_cmd() {
    let output = Execute::new("pw", &["--ver"]);
    assert_eq!(output, output);
}

#[test]
#[should_panic]
/// this test is intends to execute a wrong command and without storing it inside a file `ExecuteLog.log`
fn store_inside_file_error_wrong_cmd() {
    // use wrong commands
    let output = Execute::new("tre", &["--ver"]);
    let res = Execute::write_to_file(&output);
    assert_eq!(res.is_err(), true);
}

#[test]
/// this test check the operation of existence of the file log
fn check_op() {
    let out = Execute::new("ls", &["-l"]);
    let res = Execute::write_to_file(&out);
    let bl = Execute::check_operation(&res);
    assert_eq!(bl, true)
}

#[test]
#[should_panic]
/// check if the operation of existence of the file log is not exists and should panic accordingly to that.
fn check_op_with_error() {
    let out = Execute::new("tre", &["--ver"]);
    let res = Execute::write_to_file(&out);
    let bl = Execute::check_operation(&res);
    assert_eq!(bl, false);
}
