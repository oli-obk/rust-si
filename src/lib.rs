#![feature(plugin_registrar, rustc_private, plugin, slice_patterns)]
#![plugin(quasi_macros)]

extern crate syntax;
extern crate quasi;
extern crate rustc;

use std::fmt::Write;

use syntax::codemap::{Span, BytePos};
use syntax::parse::token;
use syntax::ast::{TokenTree, TtToken, TtDelimited, TtSequence, Ident};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;  // trait for expr_usize
use rustc::plugin::Registry;

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
                    let next = text.next().unwrap_or(b' ');
                    sp.lo = sp.lo + BytePos(1);
                    stmts.push(quote_stmt!(cx,
                        let txt = {
                            let next = [$next];
                            let next: &[u8] = match next[0] {
                                b'\n'
                                | b'\r'
                                | b'\t'
                                | b' ' => b"\t\r\n ",
                                _ => &next,
                            };
                            stdin.by_ref()
                                 .map(|c| c.unwrap())
                                 .take_while(|c| !next.contains(c))
                                 .collect::<Vec<u8>>()
                        };
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
            {
                let nex = [$c];
                let n: &[u8] = match nex[0] {
                    b'\n'
                    | b'\r'
                    | b'\t'
                    | b' ' => b"\t\r\n ",
                    _ => &nex,
                };
                assert!(n.contains(&stdin.next().unwrap().unwrap()));
            }
        ).unwrap());
    }

    let expr = if tup.len() == 1 {
        tup.pop().unwrap()
    } else {
        cx.expr_tuple(sp, tup)
    };
    MacEager::expr(cx.expr_block(cx.block(sp, stmts, Some(expr))))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("read", expand_read);
}
