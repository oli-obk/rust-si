#[macro_use] extern crate text_io;

#[test]
fn simple() {
    let val: i32 = read!("{}", "42".bytes());
    assert_eq!(val, 42);
}

#[test]
fn simple_spaces() {
    let val: String = read!("{}  64", "42  64".bytes());
    assert_eq!(val, "42");
}

#[test]
fn file() {
    use std::io::Read;
    let mut file = std::fs::File::open("tests/answer.txt").unwrap().bytes().map(|ch| ch.unwrap());
    let val: i32 = read!("The answer is {}!!!11einself\n", file);
    assert_eq!(val, 42);
    let s: String = read!("There is {} spoon", file);
    assert_eq!(s, "no");
}
