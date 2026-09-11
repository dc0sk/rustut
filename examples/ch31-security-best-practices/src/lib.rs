// SPDX-License-Identifier: MIT OR Apache-2.0
//! Guided example for Chapter 31 — integer-overflow tools, and
//! `#![forbid(unsafe_code)]` as a compiler-enforced audit shortcut.

// ANCHOR: forbid_unsafe
#![forbid(unsafe_code)]
// This attribute makes *any* `unsafe` block anywhere in this crate a hard
// compile error — not a lint someone has to notice in review, a build
// failure. A library that never has a legitimate reason to touch raw
// pointers, FFI (Ch. 17, Ch. 26), or any of `unsafe`'s other four
// operations should set this: it turns "did an unsafe block sneak in" from
// a code-review question into a mechanical one. It must appear before any
// other item in the crate root, like any other inner (`#![...]`) attribute.
// ANCHOR_END: forbid_unsafe

// ANCHOR: overflow_tools
/// Adds a byte count to a running total, refusing rather than silently
/// producing a wrong answer if the total would overflow `u32`.
///
/// C's signed-integer overflow is undefined behavior unconditionally, not
/// just "wraps in release" — there is no portable, defined way to even
/// detect it happened after the fact. Rust gives you the choice, explicitly:
pub fn add_checked(total: u32, more: u32) -> Option<u32> {
    total.checked_add(more)
}

/// Same operation, but clamped at `u32::MAX` instead of failing — useful
/// when "as much as physically fits" is an acceptable answer (e.g. a
/// progress counter that should never wrap back to a small number).
pub fn add_saturating(total: u32, more: u32) -> u32 {
    total.saturating_add(more)
}

/// Same operation, but with defined, silent wraparound — the *only* time
/// you should reach for this is when wraparound is the actual intended
/// behavior (a hash mix, a ring-buffer index), not as a default.
pub fn add_wrapping(total: u32, more: u32) -> u32 {
    total.wrapping_add(more)
}
// ANCHOR_END: overflow_tools

/// This crate forbids `unsafe` code entirely (see the attribute at the top
/// of this file), so even a deliberately-written `unsafe` block touching
/// only a local variable — never mind FFI or a pointer from outside — is
/// rejected: the forbid lint doesn't evaluate whether the specific
/// operation would have been sound, it categorically refuses the keyword:
///
/// ```rust,compile_fail
/// #![forbid(unsafe_code)]
/// fn main() {
///     let x = 5;
///     let p = std::ptr::addr_of!(x);
///     unsafe {
///         let _ = *p;
///     }
/// }
/// ```
///
/// The real compiler output, captured by compiling the equivalent code as
/// a standalone `src/main.rs` (rustc 1.98, stable — doctest line numbers
/// in the actual error would differ slightly due to the wrapping rustdoc
/// generates, so this is the same code compiled directly for a clean
/// transcript):
///
/// ```text
/// error: usage of an `unsafe` block
///  --> src/main.rs:5:5
///   |
/// 5 | /     unsafe {
/// 6 | |         let _ = *p;
/// 7 | |     }
///   | |_____^
///   |
/// note: the lint level is defined here
///  --> src/main.rs:1:11
///   |
/// 1 | #![forbid(unsafe_code)]
///   |           ^^^^^^^^^^^
/// ```
pub fn forbid_unsafe_is_documented_here() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_add_succeeds_below_the_boundary() {
        assert_eq!(add_checked(u32::MAX - 1, 1), Some(u32::MAX));
    }

    #[test]
    fn checked_add_fails_exactly_at_the_boundary() {
        assert_eq!(add_checked(u32::MAX, 1), None);
    }

    #[test]
    fn saturating_add_clamps_instead_of_wrapping() {
        assert_eq!(add_saturating(u32::MAX - 1, 5), u32::MAX);
    }

    #[test]
    fn wrapping_add_wraps_around_to_zero_at_the_boundary() {
        assert_eq!(add_wrapping(u32::MAX, 1), 0);
    }
}
