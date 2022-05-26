use std::process::{Command, Stdio};

fn command(cmd: &str, args: &str) -> String {
    let output = Command::new(cmd)
        .args(args.split_whitespace())
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");
    let stdout = String::from_utf8(output.stdout).expect("failed to convert to string");
    stdout
}

#[test]
fn test_parse() {
    let result = command("cargo", "run 1+1");
    assert_eq!("number: 1\nadd: +\nnumber: 1\n", result);
}
