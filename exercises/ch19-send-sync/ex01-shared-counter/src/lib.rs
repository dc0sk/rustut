// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

use std::sync::Mutex;

/// A counter that's safe to share across threads. Unlike the free
/// `increment` function in Chapter 19's guided example, the lock lives
/// *inside* the type: callers never see a `Mutex` at all, so there's no
/// way to construct a `SharedCounter` and forget to go through the lock —
/// the API only offers locked access.
pub struct SharedCounter {
    value: Mutex<i64>,
}

impl SharedCounter {
    pub fn new() -> Self {
        todo!("wrap 0 in a Mutex")
    }

    /// Atomically adds `amount` to the counter and returns the new value.
    pub fn add(&self, amount: i64) -> i64 {
        todo!("lock, add amount, return the new value")
    }

    pub fn get(&self) -> i64 {
        todo!("lock and return the current value")
    }
}

impl Default for SharedCounter {
    fn default() -> Self {
        Self::new()
    }
}
