#![feature(plugin)]
#![plugin(text_io)]

fn main() {
    let val: i32 = read!("the answer™: {}");
    assert_eq!(val, 42);
}
