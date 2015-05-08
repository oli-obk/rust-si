[![Build Status](https://travis-ci.org/oli-obk/rust-si.svg?branch=master)](https://travis-ci.org/oli-obk/rust-si)
[![Latest Version](https://img.shields.io/crates/v/text_io.svg)](https://crates.io/crates/text_io)

Don't forget to add the plugin to your crate:

```rust
#![feature(plugin)]
#![plugin(text_io)]
```

#Examples

```rust
// read until a whitespace and try to get a number
let i: i32 = read!();

// read until a whitespace (but not including it)
let word: String = read!();

// read first value, then comma, then whitespace, then second value
let tup: (i32, String) = read!("{}, {}");
```
