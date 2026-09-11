// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

use std::sync::atomic::AtomicU32;

/// A saturating request counter: `record_request` increments the counter
/// unless it has already reached `max`, in which case it leaves the
/// counter unchanged and returns `false` (the request was "rejected").
/// Returns `true` if the counter was incremented.
///
/// This must be race-free under concurrent calls from multiple threads —
/// a plain `load` then `store` (read the value, check it, write value+1)
/// is not good enough: two threads could both read `max - 1`, both decide
/// there's room, and both increment, overshooting `max`. Use an atomic
/// compare-and-swap loop instead.
pub fn record_request(counter: &AtomicU32, max: u32) -> bool {
    todo!(
        "loop: load the current value, if it's already >= max return false; \
         otherwise try to compare_exchange it to value + 1, retrying on \
         failure (another thread changed it first) until you either \
         succeed or find the counter has reached max"
    )
}
