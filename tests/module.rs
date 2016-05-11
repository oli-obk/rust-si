#[macro_use] extern crate text_io;

mod bla {
    pub fn parse(text: String) -> (i32, i32) {
        let (a, b): (i32, i32);
        scan!(text.bytes() => "{}:{}", a, b);
        return (a, b);
    }
}

#[test]
fn calling_scan_in_submodule() {
    assert_eq!(bla::parse("5:6".to_owned()), (5, 6));
}
