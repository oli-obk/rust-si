use std::io::{BufRead, Read};
use std::str::{FromStr, from_utf8};
use std::fmt::Debug;


/// the `read_line` function returns all characters up to but not including the next `\n`
/// ```rust
/// let s: String = si::read_line();
/// ```
pub fn read_line() -> String {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines();
    lines.next().unwrap().unwrap()
}

/// the `read` function reads until the next whitespace and tries to convert the read characters
/// into the requested type `T` through the `FromStr` trait.
/// ```rust
/// let i: i32 = si::read(); // read until a whitespace and try to get a number
/// let word: String = si::read(); // read until a whitespace (but not including it)
/// ```
pub fn read<T: FromStr>() -> T
    where <T as FromStr>::Err: Debug
{
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let v: Vec<u8> = stdin.bytes()
        .map(|c| c.unwrap())
        .take_while(|c| b" \n\r\t".contains(&c))
        .collect();
    FromStr::from_str(from_utf8(&v).unwrap()).unwrap()
}
