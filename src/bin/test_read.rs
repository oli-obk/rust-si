#![cfg_attr(nightly, feature(plugin))]
#![cfg_attr(nightly, plugin(text_io))]
#[cfg_attr(not(nightly), macro_use(read))]
extern crate text_io;

fn main() {
    let val: i32 = read!("the answer™: {}");
    assert_eq!(val, 42);
}
