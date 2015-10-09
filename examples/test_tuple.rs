#![cfg_attr(feature="nightly", feature(plugin))]
#![cfg_attr(feature="nightly", plugin(text_io))]
#[cfg_attr(not(feature="nightly"), macro_use(read))]
#[cfg(not(feature="nightly"))]
extern crate text_io;

#[cfg(feature="nightly")]
pub fn main() {
    let tup: (i32, i8, String) = read!("{}, {}, {}");
    assert_eq!(tup, (42, 99, "Ä".to_string()));
}

#[cfg(not(feature="nightly"))]
pub fn main() {
    unimplemented!()
}
