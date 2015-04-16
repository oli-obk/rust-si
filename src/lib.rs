use std::io::{BufRead, Read};
use std::str::{FromStr, from_utf8};
use std::fmt::Debug;

pub fn read_line() -> String {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines();
    lines.next().unwrap().unwrap()
}

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
