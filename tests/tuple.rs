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
fn run_read() {
    test_str("target/debug/examples/test_read", "the answer™: 42");
}

#[test]
fn run_read_simple() {
    test_str("target/debug/examples/test_read_simple", "99\n");
}

#[test]
fn run_scan_simple() {
    test_str("target/debug/examples/test_scan_simple", "42");
}

#[test]
fn run_scan() {
    test_str("target/debug/examples/test_scan", "99, 42\n66");
}

#[test]
#[should_panic]
fn run_unsupported_formatting_trait(){
    test_str("target/debug/failing-examples/test_unsupported_fomatting_trait","the answer™: 42");
}
