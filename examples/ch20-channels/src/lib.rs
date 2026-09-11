// SPDX-License-Identifier: MIT OR Apache-2.0

use std::sync::mpsc;
use std::thread;

// ANCHOR: single_producer
/// One producer thread sends values; the main thread receives them.
/// `recv()` blocks until a value arrives, and `for v in rx` (used below)
/// keeps yielding values until the channel is *closed* — which happens
/// automatically once every `Sender` has been dropped. No sentinel value,
/// no separate "done" flag to remember to check under the same lock.
pub fn sum_via_channel(values: Vec<i32>) -> i32 {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for v in values {
            // `send` moves `v` to the receiving side — this is "share
            // memory by communicating": ownership of each value crosses
            // the thread boundary instead of both threads reading/writing
            // one shared location under a lock (Ch. 19's Mutex approach).
            tx.send(v).expect("receiver dropped");
        }
        // `tx` is dropped here, at the end of the closure — that's what
        // closes the channel.
    });

    rx.iter().sum()
}
// ANCHOR_END: single_producer

// ANCHOR: multi_producer
/// Cloning a `Sender` gives you another handle to the *same* channel —
/// this is the "mpsc" in `mpsc::channel`: multiple producers, one
/// consumer. The channel only closes once *every* clone of `tx` (plus the
/// original) has been dropped.
pub fn sum_via_multiple_producers(chunks: Vec<Vec<i32>>) -> i32 {
    let (tx, rx) = mpsc::channel();

    let mut handles = Vec::new();
    for chunk in chunks {
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            for v in chunk {
                tx.send(v).expect("receiver dropped");
            }
        }));
    }
    // Drop our own original `tx` too — otherwise the channel never closes,
    // since one live Sender (this one) would always remain.
    drop(tx);

    let total = rx.iter().sum();
    for h in handles {
        h.join().expect("producer thread panicked");
    }
    total
}
// ANCHOR_END: multi_producer

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_single_producer() {
        assert_eq!(sum_via_channel(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn sums_multiple_producers() {
        assert_eq!(
            sum_via_multiple_producers(vec![vec![1, 2, 3], vec![4, 5], vec![6]]),
            21
        );
    }
}
