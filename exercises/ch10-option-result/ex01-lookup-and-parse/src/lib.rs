// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

/// Return the first positive value in `values`, or `None` if there isn't
/// one. No manual loop with a `found` flag — use iterator combinators.
pub fn first_positive(values: &[i32]) -> Option<i32> {
    todo!("find the first value > 0, without writing a manual for-loop with a flag variable")
}

/// Parse `"3,4"` into `(3, 4)`. Two distinct failure modes must both be
/// reported as `Err(String)`:
/// - no `,` in the input at all
/// - either side doesn't parse as an `i64`
///
/// Combine `Option` (from `str::split_once`) and `Result` (from `str::parse`)
/// into a single `Result<(i64, i64), String>` using `.ok_or_else()` and
/// `.map_err()`.
pub fn parse_pair(input: &str) -> Result<(i64, i64), String> {
    todo!(
        "split on ',', turn the missing-comma case and both parse failures into Err(String), and use `?` to propagate them"
    )
}
