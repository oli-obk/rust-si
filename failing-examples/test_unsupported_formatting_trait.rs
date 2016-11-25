#[macro_use] extern crate text_io;

pub fn main() {
    let val: i32 = read!("the answer™: {:o}");
    assert_eq!(val, 42);
}