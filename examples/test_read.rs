#![cfg_attr(feature="nightly", feature(plugin))]
#![cfg_attr(feature="nightly", plugin(text_io))]
#[cfg(not(feature="nightly"))] #[macro_use] extern crate text_io;

pub fn main() {
    let val: i32 = read!("the answer™: {}");
    assert_eq!(val, 42);
}
