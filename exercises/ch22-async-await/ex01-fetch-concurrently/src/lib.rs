// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

use std::time::Duration;
use tokio::time::sleep;

/// Simulates fetching one item: "the network" takes `delay_ms` to
/// respond, then hands back `delay_ms` itself as a stand-in payload.
async fn fetch_one(delay_ms: u64) -> u64 {
    sleep(Duration::from_millis(delay_ms)).await;
    delay_ms
}

/// Fetch every delay in `delays_ms` **concurrently** (not one after
/// another), returning the results in the same order as the input.
///
/// A correct implementation should take roughly as long as the *slowest*
/// single fetch, not the sum of every fetch's delay.
pub async fn fetch_all(delays_ms: Vec<u64>) -> Vec<u64> {
    todo!("spawn a task per delay with fetch_one, then await all of them in order")
}
