// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

use std::sync::mpsc;
use std::thread;

/// Spawn one thread per entry in `workers`, each computing the square of
/// its input and sending the result back over a shared channel. Collect
/// every result (order doesn't matter) into a `Vec<i32>`, then return it
/// sorted ascending. Every spawned thread must be joined before this
/// function returns.
pub fn square_all_concurrently(workers: Vec<i32>) -> Vec<i32> {
    todo!(
        "for each value in `workers`, spawn a thread that sends back its \
         square over a channel; collect all results, sort them, and \
         return them — join every thread before returning"
    )
}
