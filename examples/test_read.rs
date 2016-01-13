#[macro_use] extern crate text_io;

pub fn main() {
    let val: i32 = read!("the answer™: {}");
    assert_eq!(val, 42);
}
