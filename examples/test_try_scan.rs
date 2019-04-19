#[macro_use]
extern crate text_io;

pub fn main() {
    use std::error::Error;

    fn parse_box_error() -> Result<(i32, i32, i32), Box<Error>> {
        // Parsing from stdio
        let (a, b, c);
        try_scan!("{}, {}\n{}", a, b, c);
        Ok((a, b, c))
    }

    let val = parse_box_error();
    // Cannot match on `Ok` due to `Box<Error>` not implementing `PartialEq`, so we unwrap.
    assert_eq!(val.unwrap(), (99, 42, 66));

    fn parse_textio_error() -> Result<(i32, i32, i32), text_io::Error> {
        // Parsing a string literal
        let (a, b, c);
        try_scan!("99, 42\n66".bytes() => "{}, {}\n{}", a, b, c);
        Ok((a, b, c))
    }

    let val = parse_textio_error();
    assert_eq!(val, Ok((99, 42, 66)));

    let parse_closure = |input: &str| -> Result<(i32, i32, i32), text_io::Error> {
        // Parsing the closure input
        let (a, b, c);
        try_scan!(input.bytes() => "{}, {}\n{}", a, b, c);
        Ok((a, b, c))
    };

    let val = parse_closure("99, abc\n66");
    // Failed to parse 'abc' into variable 'b' (defined in the closure)
    assert_eq!(val, Err(text_io::Error::Parse("abc".into(), "b")));

    let val = parse_closure("99, 42\n66");
    assert_eq!(val, Ok((99, 42, 66)));
}
