// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: immutability
/// `let` bindings are immutable by default — reassigning one without
/// `mut` is a compile error, not a warning:
///
/// ```compile_fail
/// let x = 5;
/// x = 6; // error[E0384]: cannot assign twice to immutable variable `x`
/// ```
///
/// Captured on this book's toolchain, the real compiler output is:
///
/// ```text
/// error[E0384]: cannot assign twice to immutable variable `x`
///  --> immut.rs:3:5
///   |
/// 2 |     let x = 5;
///   |         - first assignment to `x`
/// 3 |     x = 6;
///   |     ^^^^^ cannot assign twice to immutable variable
///   |
/// help: consider making this binding mutable
///   |
/// 2 |     let mut x = 5;
///   |         +++
/// ```
pub fn immutability_examples_are_doctests_only() {}
// ANCHOR_END: immutability

// ANCHOR: shadowing
pub fn shadowing_demo() -> usize {
    // `let` without `mut` still allows re-*declaring* the name: this is
    // shadowing, not mutation. Each `let` introduces a brand-new binding
    // (which can even change type), it does not overwrite the old one in
    // place — the old `spaces` (a `&str`) still exists until this scope
    // ends, it's just no longer nameable.
    let spaces = "   ";
    let spaces = spaces.len(); // now a usize — a genuinely different type
    spaces
}
// ANCHOR_END: shadowing

// ANCHOR: narrowing
/// Unlike C, assigning a wider integer type to a narrower variable is a
/// compile error, not a silent truncation — see the `compile_fail` doctest
/// below. The checked, safe way to go from a wide type to a narrow one is
/// `TryFrom`/`try_into`, which reports failure instead of truncating:
///
/// ```
/// use std::convert::TryFrom;
///
/// let too_big: u32 = 90_000;
/// assert!(u16::try_from(too_big).is_err()); // doesn't fit in 16 bits
///
/// let fits: u32 = 8080;
/// assert_eq!(u16::try_from(fits), Ok(8080));
/// ```
///
/// If you genuinely want C's truncating behavior, it still exists — but
/// you have to ask for it explicitly with `as`:
///
/// ```
/// let wide: u32 = 90_000;
/// let truncated = wide as u16; // explicit opt-in to truncation
/// assert_eq!(truncated, (90_000u32 % 65_536) as u16);
/// ```
///
/// And here is the rejected version — the same assignment C performs
/// silently, with no cast at all:
///
/// ```compile_fail
/// let wide: u32 = 1000;
/// let narrow: u8 = wide; // error[E0308]: mismatched types
/// ```
///
/// Captured on this book's pinned toolchain, the real compiler output for
/// that last snippet is:
///
/// ```text
/// error[E0308]: mismatched types
///  --> narrow.rs:3:21
///   |
/// 3 |     let small: u8 = big;
///   |                --   ^^^ expected `u8`, found `u32`
///   |                |
///   |                expected due to this
///   |
/// help: you can convert a `u32` to a `u8` and panic if the converted value doesn't fit
///   |
/// 3 |     let small: u8 = big.try_into().unwrap();
///   |                        ++++++++++++++++++++
/// ```
pub fn narrowing_examples_are_doctests_only() {}
// ANCHOR_END: narrowing

// ANCHOR: arrays
pub fn array_demo() -> usize {
    // A fixed-size array's length is part of its *type* ([i32; 4], not
    // just `int*`), so `.len()` is always available with zero runtime
    // cost — no separate "did I remember to pass the length" parameter,
    // and no decay to a bare pointer that's lost track of its own size.
    let readings: [i32; 4] = [10, 20, 30, 40];
    readings.len()
}
// ANCHOR_END: arrays

// ANCHOR: if_expression
pub fn if_expression_demo(threshold_exceeded: bool) -> &'static str {
    // `if` is an expression, not just a statement: both branches must
    // produce the same type, and the whole `if` evaluates to that value.
    // There is no C equivalent to this beyond the very limited `?:`
    // ternary — this works for arbitrarily large branch bodies.
    if threshold_exceeded {
        "over threshold"
    } else {
        "nominal"
    }
}
// ANCHOR_END: if_expression

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadowing_demo_returns_length() {
        assert_eq!(shadowing_demo(), 3);
    }

    #[test]
    fn array_demo_returns_len() {
        assert_eq!(array_demo(), 4);
    }

    #[test]
    fn if_expression_demo_picks_a_branch() {
        assert_eq!(if_expression_demo(true), "over threshold");
        assert_eq!(if_expression_demo(false), "nominal");
    }
}
