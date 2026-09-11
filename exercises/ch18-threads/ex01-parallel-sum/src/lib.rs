// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

/// Split `data` into `num_workers` roughly-equal chunks, sum each chunk on
/// its own scoped thread, and return the total. Must produce the same
/// result as `data.iter().sum::<i64>()` for any `num_workers >= 1`,
/// including when `num_workers` doesn't evenly divide `data.len()`.
///
/// Use `std::thread::scope` so the worker threads can borrow slices of
/// `data` directly — no cloning, no `Arc`, no `'static` bound needed.
pub fn parallel_sum(data: &[i64], num_workers: usize) -> i64 {
    todo!("split data into num_workers chunks, sum each on a scoped thread, add the results")
}
