#![cfg_attr(nightly, feature(plugin))]
#![cfg_attr(nightly, plugin(text_io))]
#[cfg_attr(not(nightly), macro_use(read))]
extern crate text_io;

#[cfg(nightly)]
fn main() {
    let tup: (i32, i8, String) = read!("{}, {}, {}");
    assert_eq!(tup, (42, 99, "Ä".to_string()));
}

#[cfg(not(nightly))]
fn main() {
    unimplemented!()
}
