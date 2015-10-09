fn test_str(exe: &str, s: &str) {
    use std::process::{Command, Stdio};
    use std::io::Write;
    use std::str::from_utf8;
    let mut cmd = Command::new(exe)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    println!("started cmd");
    write!(cmd.stdin.as_mut().unwrap(), "{}", s).unwrap();
    println!("wrote to stdin");
    let res = cmd.wait_with_output().unwrap();
    println!("stdout: {}", from_utf8(&res.stdout).unwrap());
    println!("stderr: {}", from_utf8(&res.stderr).unwrap());
    println!("status: {:?}", res.status);
    assert!(res.status.success());
}

#[test]
#[cfg(feature="nightly")]
fn run() {
    test_str("target/debug/test_tuple", "42, 99, Ä");
}

#[test]
#[should_panic]
fn no_ws_replacement() {
    test_str("target/debug/test_tuple", "42,\t99,\nÄ");
}

#[test]
#[should_panic]
fn run_fail() {
    test_str("target/debug/test_tuple", "42,99, Ä");
}

#[test]
fn run_read() {
    test_str("target/debug/test_read", "the answer™: 42");
}

#[test]
fn run_read_simple() {
    test_str("target/debug/test_read_simple", "99\n");
}
