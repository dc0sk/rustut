// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md for the task. The doctest below is complete and
//! illustrative (not part of the task) — it's a `compile_fail` doctest
//! demonstrating the aliasing rule this chapter is about:
//!
//! ```rust,compile_fail
//! let mut data = vec![1, 2, 3];
//! let view = &data;
//! data.push(4);
//! println!("{:?}", view);
//! ```

/// Returns a reference to the largest element in `nums`, or `None` if
/// `nums` is empty. Must not clone or copy any element out — return a
/// borrow into the original slice, the way a C function might return a
/// pointer into an array it doesn't own.
pub fn largest(nums: &[i32]) -> Option<&i32> {
    todo!("find the largest element and return a reference to it")
}

/// Pushes `extra` onto `data`, then reports its new length and sum.
pub fn grow_then_summarize(data: &mut Vec<i32>, extra: i32) -> String {
    todo!("push `extra`, then format a message like \"5 numbers, sum 15\"")
}
