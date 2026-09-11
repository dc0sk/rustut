// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

/// Safely narrow an `i64` (say, a value read from a config file) into a
/// `u16` (say, a TCP port). Unlike C's implicit truncation on assignment,
/// return `Err(value)` — the original, untouched value — when it doesn't
/// fit, instead of silently keeping only the low 16 bits.
pub fn to_port(value: i64) -> Result<u16, i64> {
    todo!("use u16::try_from(value), mapping a conversion error back to Err(value)")
}

/// Add two `u8`s without ever panicking or silently wrapping: return
/// `None` on overflow instead of relying on debug-only overflow panics or
/// release-mode wraparound (Chapter 31 covers why relying on either is a
/// security footgun).
pub fn checked_sum(a: u8, b: u8) -> Option<u8> {
    todo!("use a.checked_add(b)")
}
