#[macro_use] extern crate text_io;

pub fn main() {
    let (a, b, c): (i32, i32, i32);
    scan!("{}, {}\n{}", a, b, c);
    assert_eq!(a, 99);
    assert_eq!(b, 42);
    assert_eq!(c, 66);
}
