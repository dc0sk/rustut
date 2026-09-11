// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

/// Sums `f(x)` for every element `x` of `data` that satisfies `keep`.
/// Implement this using iterator combinators (`.iter()`, `.filter()`,
/// `.map()`, `.sum()`) — no manual indexing or `for` loop with an index.
pub fn sum_where(data: &[i32], keep: impl Fn(i32) -> bool, f: impl Fn(i32) -> i64) -> i64 {
    todo!("filter `data` by `keep`, transform survivors with `f`, sum the results")
}
