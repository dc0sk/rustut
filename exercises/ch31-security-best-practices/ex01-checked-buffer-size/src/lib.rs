// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

/// Computes the total byte size of a buffer holding `record_count` records
/// of `record_size` bytes each, plus a fixed `header_bytes` header.
///
/// This is the exact shape of a real, well-known C vulnerability class:
/// `malloc(header_bytes + record_count * record_size)` where the
/// multiplication (or the addition) silently overflows a 32-bit size,
/// wraps to a small number, `malloc` happily returns a small buffer, and
/// the code that fills it in — believing it asked for and got a *large*
/// buffer — writes past the end of the small one it actually got. Return
/// `None` instead of a wrong, wrapped-around answer if any step overflows.
pub fn buffer_size(header_bytes: u32, record_count: u32, record_size: u32) -> Option<u32> {
    todo!(
        "multiply record_count * record_size, add header_bytes, using checked arithmetic throughout"
    )
}
