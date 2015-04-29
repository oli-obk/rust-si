fn test_str(s: &str) {
    use std::process::{Command, Stdio};
    use std::io::Write;
    use std::str::from_utf8;
    let mut cmd = Command::new("test_tuple")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    write!(cmd.stdin.as_mut().unwrap(), "{}", s).unwrap();
    let res = cmd.wait_with_output().unwrap();
    println!("stdout: {}", from_utf8(&res.stdout).unwrap());
    println!("stderr: {}", from_utf8(&res.stderr).unwrap());
    println!("status: {:?}", res.status);
    assert!(res.status.success());
}

#[test]
fn run() {
    test_str("42, 99, Ä");
}

#[test]
fn run2() {
    test_str("42,\t99,\nÄ");
}

#[test]
#[should_panic]
fn run_fail() {
    test_str("42,99, Ä");
}
