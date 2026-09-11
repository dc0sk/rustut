// SPDX-License-Identifier: MIT OR Apache-2.0

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;

// ANCHOR: counter
/// A lock-free counter shared across threads. `fetch_add` is the same
/// operation as C11's `atomic_fetch_add`/GCC's `__sync_fetch_and_add`: an
/// indivisible read-modify-write, no lock involved. `Ordering::Relaxed` is
/// appropriate here because the *only* invariant this function cares
/// about is the final total — nothing else needs to be synchronized
/// relative to these increments (contrast with the guard example below,
/// where ordering does matter).
pub fn count_concurrently(increments_per_thread: u64, thread_count: usize) -> u64 {
    let counter = Arc::new(AtomicU64::new(0));

    let handles: Vec<_> = (0..thread_count)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..increments_per_thread {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();

    for h in handles {
        h.join().expect("counter thread panicked");
    }

    counter.load(Ordering::Relaxed)
}
// ANCHOR_END: counter

// ANCHOR: guard
/// A one-shot "has this already happened?" flag — the atomic equivalent
/// of a `Mutex<bool>`, but lock-free, exactly like C11's
/// `atomic_compare_exchange_strong` on an `atomic_bool` used the same way.
/// `compare_exchange` here uses `Ordering::Acquire`/`Ordering::Relaxed` —
/// the same acquire/release vocabulary as `stdatomic.h`, with the same
/// meaning: `Acquire` on success ensures nothing after this call can be
/// reordered before it, which matters if other memory writes are meant to
/// become visible only once the flag flips.
pub fn run_once(already_ran: &AtomicBool) -> bool {
    already_ran
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_ok()
}
// ANCHOR_END: guard

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_across_threads() {
        assert_eq!(count_concurrently(1000, 8), 8000);
    }

    #[test]
    fn run_once_only_succeeds_the_first_time() {
        let flag = AtomicBool::new(false);
        assert!(run_once(&flag));
        assert!(!run_once(&flag));
        assert!(!run_once(&flag));
    }
}
