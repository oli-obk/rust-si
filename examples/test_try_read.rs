#[macro_use]
extern crate text_io;

pub fn main() {
    use std::error::Error;

    fn parse_box_error() -> Result<i32, Box<dyn Error>> {
        // Parsing from stdio
        Ok(try_read!("the answer™: {}")?)
    }

    let val = parse_box_error();
    // Cannot match on `Ok` due to `Box<Error>` not implementing `PartialEq`, so we unwrap.
    assert_eq!(val.unwrap(), 42);

    fn parse_textio_error() -> Result<i32, text_io::Error> {
        // Parsing a string literal
        try_read!("the answer™: {}", "the answer™: 42".bytes())
    }

    let val = parse_textio_error();
    assert_eq!(val, Ok(42));

    let parse_closure = |input: &str| -> Result<i32, text_io::Error> {
        // Parsing a string literal
        try_read!("the answer™: {}", input.bytes())
    };

    let val = parse_closure("the answer™: abc");
    // Failed to parse 'abc' into variable '__try_read_var__' (defined in the try_read macro)
    assert_eq!(
        val,
        Err(text_io::Error::Parse("abc".into(), "__try_read_var__"))
    );

    let val = parse_closure("the answer™: 42");
    assert_eq!(val, Ok(42));
}
