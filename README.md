[![Build Status](https://travis-ci.org/oli-obk/rust-si.svg?branch=master)](https://travis-ci.org/oli-obk/rust-si)
[![Latest Version](https://img.shields.io/crates/v/text_io.svg)](https://crates.io/crates/text_io)
[![Clippy Linting Result](http://clippy.bashy.io/github/oli-obk/rust-si/master/badge.svg)](http://clippy.bashy.io/github/oli-obk/rust-si/master/log)

Don't forget to add the plugin to your crate:

```rust
#[macro_use] extern crate text_io;
```

You can use either the `read!` macro to read a single value and return it, or
the `scan!` macro to read one or more values into variables. Both macros can
also read from a file or from memory. The `read!` macro can take any type that
implements `Iterator<Item=u8>` as an optional third argument, and the `scan!`
macro's arguments can be prefixed with `iter => ` where `iter` implements
`Iterator<Item=u8>`.

#Examples

##scan! macro

```rust
// reading from a string source
let i: i32;
scan!("<b>12</b>".bytes() => "<b>{}</b>", i);
assert_eq!(i, 12);

// reading multiple values from stdio
let a: i32;
let b: &mut u8 = &mut 5;
scan!("{}, {}", a, *b);
```

##read! macro

```rust
// read until a whitespace and try to convert what was read into an i32
let i: i32 = read!();

// read until a whitespace (but not including it)
let word: String = read!(); // same as read!("{}")

// read until a newline (but not including it)
let line: String = read!("{}\n");

// expect the input "<b><i>" or panic
// read until the next "<" and return that.
// expect the input "/i></b>"
let stuff: String = read!("<b><i>{}</i></b>");

// reading from files
use std::io::Read;
let mut file = std::fs::File::open("tests/answer.txt").unwrap().bytes().map(|ch| ch.unwrap());
let val: i32 = read!("The answer is {}!!!11einself\n", file);

// reading from strings
let val: i32 = read!("Number: {}", "Number: 99".bytes());
```
