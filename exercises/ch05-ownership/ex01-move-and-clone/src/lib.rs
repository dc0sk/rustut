// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

/// Takes ownership of both `data` and `suffix`, appends `suffix` to
/// `data`, and returns the combined `String`. Neither input is available
/// to the caller afterward — that's the point: this function consumes
/// what it's given, the same way a C function that calls `free()` on an
/// argument consumes it, except the compiler enforces "don't use it
/// again" instead of it being a documentation-only convention.
pub fn append_owned(data: String, suffix: String) -> String {
    todo!("append `suffix` onto `data` and return the combined String")
}

/// Takes ownership of `data`, doubles every element in place, and returns
/// the very same `Vec` (no new allocation, no clone) — ownership flowing
/// out through the return value.
pub fn double_in_place(data: Vec<i32>) -> Vec<i32> {
    todo!("double every element of `data`, then return it")
}

/// Returns an independent, cloned copy of `original`. Unlike
/// `append_owned` above, `original` is only *borrowed* here (`&str`), so
/// nothing is consumed — `.clone()` is what actually produces the new,
/// separately-owned `String`.
pub fn duplicate(original: &str) -> String {
    todo!("return a cloned/owned copy of `original`")
}
