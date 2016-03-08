[![Build Status](https://travis-ci.org/oli-obk/rust-si.svg?branch=master)](https://travis-ci.org/oli-obk/rust-si)
[![Latest Version](https://img.shields.io/crates/v/text_io.svg)](https://crates.io/crates/text_io)
[![Clippy Linting Result](http://clippy.bashy.io/github/oli-obk/rust-si/master/badge.svg)](http://clippy.bashy.io/github/oli-obk/rust-si/master/log)

Don't forget to add the plugin to your crate:

```rust
#[macro_use] extern crate text_io;
```

#Examples

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
```
