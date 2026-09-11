// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

/// A minimal trait for anything that carries a numeric "score."
pub trait Scored {
    fn score(&self) -> i64;
}

/// Return a reference to the item in `items` with the highest `score()`.
///
/// Bound the generic on `Scored`, not on a concrete type — the point of
/// this exercise is writing a function that works for *any* `Scored`
/// type, the same way `largest<T: PartialOrd + Copy>` in the chapter's
/// guided example works for any orderable, copyable type.
///
/// Panics if `items` is empty.
pub fn highest_scored<T: Scored>(items: &[T]) -> &T {
    todo!("find and return a reference to the item with the maximum score()")
}
