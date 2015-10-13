//! This crate allows one-liners to read from a terminal
//! A minimal example to get an i32 from the command line is
//!
//! ```rust
//! #[macro_use] extern crate text_io;
//! fn main() {
//!     let i: i32 = read!();
//! }
//! ```
//!
//! The `read!()` macro will always read until the next ascii whitespace character
//! (`\n`, `\r`, `\t` or space).
//!
//! Any type that implements the `FromStr` trait can be read with the `read!` macro.
//!
//! # Advanced
//! Text parsing can be done similar to `println!` by adding a format string
//! to the macro:
//!
//! ```rust
//! let i: i32 = read!("The answer: {}!");
//! ```
//!
//! This will read `"The answer: "`, then an integer, then an exclamation mark. Any deviation from
//! the format string will result in a panic.
//!
//! Note: only a single value can be read per `read!` invocation. If you want more complex input
//! you need to either use multiple `read!` invocations or use the nightly compiler
//!
//! # Expert (nightly)
//! If you compile with nightly, you can enable the cargo feature `nightly` in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! text_io = { version = "*", features = ["nightly"] }
//! ```
//!
//! Now you can add an arbitrary number of `{}` to your format string.

#![cfg_attr(feature="nightly", feature(plugin_registrar, rustc_private, plugin, slice_patterns))]
#![cfg_attr(feature="nightly", plugin(quasi_macros))]

#[cfg(feature="nightly")]
extern crate syntax;
#[cfg(feature="nightly")]
extern crate quasi;
#[cfg(feature="nightly")]
extern crate rustc;
#[cfg(feature="nightly")]
use std::fmt::Write;

#[cfg(feature="nightly")]
use syntax::codemap::{Span, BytePos, Spanned};
#[cfg(feature="nightly")]
use syntax::parse::token;
#[cfg(feature="nightly")]
use syntax::ast::{TokenTree, TtToken, TtDelimited, TtSequence, Ident};
#[cfg(feature="nightly")]
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
#[cfg(feature="nightly")]
use syntax::ext::build::AstBuilder;  // trait for expr_usize
#[cfg(feature="nightly")]
use rustc::plugin::Registry;

#[cfg(feature="nightly")]
fn expand_read(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
        -> Box<MacResult + 'static> {

    let (text, sp) = match args {
        [TtToken(sp, token::Literal(token::Lit::Str_(s), _))] => (s, sp),
        [TtToken(sp, _)] => {
            cx.span_err(sp, "expected a string literal");
            return DummyResult::any(sp);
        }
        [TtDelimited(sp, _)] => {
            cx.span_err(sp, "expected a string literal got delimited");
            return DummyResult::any(sp);
        }
        [TtSequence(sp, _)] => {
            cx.span_err(sp, "expected a string literal got sequence");
            return DummyResult::any(sp);
        }
        [] => {
            (token::intern("{}"), sp)
        }
        [..] => {
            cx.span_err(sp, "expected a single string literal");
            return DummyResult::any(sp);
        }
    };
    let text = text.as_str();

    let mut tup = vec![];
    let mut stmts = vec![];
    stmts.push(quote_stmt!(cx,
        use std::io::Read;
    ).unwrap());
    stmts.push(quote_stmt!(cx,
        use std::str::{FromStr, from_utf8};
    ).unwrap());
    stmts.push(quote_stmt!(cx,
        let stdin = std::io::stdin();
    ).unwrap());
    stmts.push(quote_stmt!(cx,
        let mut stdin = stdin.lock().bytes();
    ).unwrap());
    let mut text = text.bytes();
    let mut sp = sp;
    let mut n = 0;
    while let Some(c) = text.next() {
        sp.lo = sp.lo + BytePos(1);
        match c {
            b'{' => match text.next().unwrap() {
                b'{' => {
                    sp.lo = sp.lo + BytePos(2);
                },
                b'}' => {
                    let next = text.next().map(|c| [c]);
                    let next: &[u8] = match &next {
                        &Some(ref c) => c,
                        &None => b"\t\r\n ",
                    };
                    sp.lo = sp.lo + BytePos(1);
                    let next = syntax::ast::LitBinary(std::rc::Rc::new(next.iter().cloned().collect()));
                    let next = Spanned {
                        node: next,
                        span: sp,
                    };
                    // compiler plugin + library = not good idea -> can't use read_until here...
                    stmts.push(quote_stmt!(cx,
                        let txt = stdin.by_ref()
                                       .map(|c| c.unwrap())
                                       .take_while(|c| !$next.contains(c))
                                       .collect::<Vec<u8>>();
                    ).unwrap());
                    let mut name = "tup".to_string();
                    name.write_fmt(format_args!("{}", n)).unwrap();
                    let name = Ident::new(token::intern(&name));
                    n += 1;
                    stmts.push(quote_stmt!(cx,
                        let $name = FromStr::from_str(from_utf8(&txt).unwrap()).unwrap();
                    ).unwrap());
                    tup.push(quote_expr!(cx, $name));
                    continue;
                },
                _ => {
                    sp.hi = sp.lo + BytePos(1);
                    cx.span_err(sp, "found bad curly brace");
                    return DummyResult::any(sp);
                }
            },
            _ => {}
        }
        stmts.push(quote_stmt!(cx,
            assert_eq!($c, stdin.next().unwrap().unwrap());
        ).unwrap());
    }

    let expr = if tup.len() == 1 {
        tup.pop().unwrap()
    } else {
        cx.expr_tuple(sp, tup)
    };
    MacEager::expr(cx.expr_block(cx.block(sp, stmts, Some(expr))))
}

#[cfg(feature="nightly")]
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("read", expand_read);
}


///////////////////////////////////////////////////////////////////////////////////////////////
//BBB  EEEE TTTTT   A     /////////////////////////////////////////////////////////////////////
//B  B E      T    A A    /////////////////////////////////////////////////////////////////////
//BBB  EEE    T   A   A   /////////////////////////////////////////////////////////////////////
//B  B E      T  AAAAAAA  /////////////////////////////////////////////////////////////////////
//BBB  EEEE   T A       A /////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(not(feature="nightly"))]
/// All text input is handled through this macro
#[macro_export]
macro_rules! read(
    () => { read!("{}") };
    ($text:expr) => { read!($text, std::io::stdin().bytes().map(|c| c.unwrap())) };
    ($text:expr, $input:expr) => {{
        use std::io::Read;
        use std::str::{FromStr, from_utf8};
        // typesafe macros :)
        let text: &'static str = $text;
        let stdin: &mut Iterator<Item = u8> = &mut ($input);

        let mut text = text.bytes();
        let value;
        loop { match text.next() {
            Some(b'{') => match text.next() {
                Some(b'{') => assert_eq!(Some(b'{'), stdin.next()),
                Some(b'}') => {
                    let s: Vec<u8> = match text.next() {
                        Some(c) => stdin.take_while(|&ch| ch != c).collect(),
                        None => stdin.take_while(|ch| !b"\t\r\n ".contains(ch)).collect(),
                    };
                    let s = std::str::from_utf8(&s).unwrap();
                    value = FromStr::from_str(s).unwrap();
                    for c in text {
                        assert_eq!(Some(c), stdin.next());
                    }
                    break;
                }
                Some(_) => panic!("found bad curly brace"),
                None => panic!("found single open curly brace at the end of the format string"),
            },
            Some(c) => assert_eq!(Some(c), stdin.next()),
            None => panic!("Bad read! format string: did not contain {{}}"),
        } }
        value
    }};
);
