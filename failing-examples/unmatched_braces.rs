
#[macro_use] extern crate text_io;
pub fn main() {
    let a: i32;
    let b: &mut u8 = &mut 5;
    scan!("{", a, *b);//^^^~error 7:11: 7:16: invalid format string: expected '}' but string was terminated
}
