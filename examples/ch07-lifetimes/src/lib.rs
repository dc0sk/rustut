// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: elided
/// No explicit lifetime needed: exactly one reference goes in, and the
/// return type borrows — the compiler's elision rules fill in "the
/// output lives as long as the input" automatically. This is the
/// everyday case; you will write this shape far more often than an
/// explicit `'a`.
pub fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}
// ANCHOR_END: elided

// ANCHOR: explicit
/// Elision stops working once there's more than one input reference and
/// the output could plausibly borrow from *either* — the compiler
/// refuses to guess, so `'a` must be spelled out to say "the result
/// borrows from whichever of `a` or `b` is returned, and is valid only
/// as long as *both* are."
pub fn shortest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() < b.len() { a } else { b }
}
// ANCHOR_END: explicit

// ANCHOR: struct_with_lifetime
/// A struct holding a reference must name that reference's lifetime as a
/// generic parameter: `Parser` cannot outlive the `&'a str` it borrows
/// from. This is not a runtime field — no extra bytes, no runtime check —
/// it's a compile-time-only fact the compiler uses to reject any attempt
/// to use a `Parser` after the string it points into is gone.
pub struct Parser<'a> {
    remaining: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { remaining: input }
    }

    pub fn remaining(&self) -> &'a str {
        self.remaining
    }
}
// ANCHOR_END: struct_with_lifetime

// ANCHOR: dangling
/// The compiler rejects returning a reference that would outlive the
/// data it points to — the compile-time version of the classic C bug
/// "return a pointer to a stack variable that's about to be popped."
/// Chapter 1 listed this bug as "dangling pointer"; this is where it's
/// actually caught:
///
/// ```rust,compile_fail
/// fn shortest<'a>(a: &'a str, b: &'a str) -> &'a str {
///     if a.len() < b.len() { a } else { b }
/// }
///
/// let a = String::from("hi");
/// let result;
/// {
///     let b = String::from("longer string");
///     result = shortest(&a, &b); // `b` doesn't live long enough
/// }
/// println!("{result}"); // `b` is already gone here
/// ```
///
/// The real compiler output (rustc 1.98, stable):
///
/// ```text
/// error[E0597]: `b` does not live long enough
///   --> dangling2.rs:10:31
///    |
///  9 |         let b = String::from("longer string");
///    |             - binding `b` declared here
/// 10 |         result = shortest(&a, &b);
///    |                               ^^ borrowed value does not live long enough
/// 11 |     }
///    |     - `b` dropped here while still borrowed
/// 12 |     println!("{result}");
///    |                ------ borrow later used here
/// ```
pub fn dangling_reference_only() {}
// ANCHOR_END: dangling
