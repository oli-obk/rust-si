#![cfg_attr(feature="nightly", feature(plugin))]
#![cfg_attr(feature="nightly", plugin(text_io))]
#[cfg_attr(not(feature="nightly"), macro_use(read))]
#[cfg(not(feature="nightly"))]
extern crate text_io;

fn main() {
    let val: i32 = read!("the answer™: {}");
    assert_eq!(val, 42);
}
