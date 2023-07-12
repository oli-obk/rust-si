#[macro_use]
extern crate text_io;

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
    let mut file = std::fs::File::open("tests/answer.txt")
        .unwrap()
        .bytes()
        .map(|ch| ch.unwrap());
    let val: i32 = read!("The answer is {}!!!11einself\n", file);
    assert_eq!(val, 42);
    let s: String = read!("There is {} spoon", file);
    assert_eq!(s, "no");
}

#[test]
fn file_scan() {
    use std::io::Read;
    let mut file = std::fs::File::open("tests/answer.txt")
        .unwrap()
        .bytes()
        .map(|ch| ch.unwrap());
    let val: i32;
    let s = &mut String::new();
    scan!(file => "The answer is {}!!!11einself\nThere is {} spoon", val, *s);
    assert_eq!(val, 42);
    assert_eq!(s, "no");
}

#[test]
fn scan() {
    let val: i32;
    scan!("42".bytes() => "{}", val);
    assert_eq!(val, 42);

    let i: i32;
    scan!("<b>12</b>".bytes() => "<b>{}</b>", i);
    assert_eq!(i, 12);

    let mut stdin = "1  2".bytes();
    let (i, j): (i32, i32);
    scan!(stdin => "{}", i);
    scan!(stdin => "{}", j);
    assert_eq!(1, i);
    assert_eq!(2, j);

    // Returns the scanned value multiplied by two, or None if the provided
    // text doesn't match the pattern
    fn scan_and_double(text: &str) -> Option<i32> {
        let value: i32;
        try_scan!(@impl or_none; text.bytes() => "<b>{}</b>", value);
        Some(value * 2)
    }
    assert_eq!(scan_and_double("<b>7</b>"), Some(14));
    assert_eq!(scan_and_double("<c>7</c>"), None);
}
