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
#[cfg(nightly)]
fn run() {
    test_str("test_tuple", "42, 99, Ä");
}

#[test]
#[cfg(nightly)]
fn run2() {
    test_str("test_tuple", "42,\t99,\nÄ");
}

#[test]
#[should_panic]
fn run_fail() {
    test_str("test_tuple", "42,99, Ä");
}

#[test]
fn run_read() {
    test_str("test_read", "the answer™: 42");
}
