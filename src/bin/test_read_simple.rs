#![cfg_attr(feature="nightly", feature(plugin))]
#![cfg_attr(feature="nightly", plugin(text_io))]
#[cfg(not(feature="nightly"))] #[macro_use] extern crate text_io;

pub fn main() {
    let i: i32 = read!();
    assert_eq!(i, 99);
}
