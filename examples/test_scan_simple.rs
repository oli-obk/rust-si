#[macro_use] extern crate text_io;

pub fn main() {
    let a: i32;
    scan!("{}", a);
    assert_eq!(a, 42);
}
