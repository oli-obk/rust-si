#![feature(plugin)]
#![plugin(text_io)]

fn main() {
    let tup: (i32, i8, String) = read!("{}, {}, {}");
    assert_eq!(tup, (42, 99, "Ä".to_string()));
}
