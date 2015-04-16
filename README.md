![Travis](https://travis-ci.org/oli-obk/rust-si.svg)

#Examples

```rust
let s: String = si::read_line();
let i: i32 = si::read(); // read until a whitespace and try to get a number
let word: String = si::read(); // read until a whitespace (but not including it)
```