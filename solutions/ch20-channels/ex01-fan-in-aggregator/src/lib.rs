// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch20-channels/ex01-fan-in-aggregator.

use std::sync::mpsc;
use std::thread;

pub fn square_all_concurrently(workers: Vec<i32>) -> Vec<i32> {
    let (tx, rx) = mpsc::channel();

    let handles: Vec<_> = workers
        .into_iter()
        .map(|n| {
            let tx = tx.clone();
            thread::spawn(move || {
                tx.send(n * n).expect("receiver dropped");
            })
        })
        .collect();
    drop(tx);

    let mut results: Vec<i32> = rx.iter().collect();
    for h in handles {
        h.join().expect("worker thread panicked");
    }

    results.sort_unstable();
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        assert_eq!(square_all_concurrently(vec![3, 1, 2]), vec![1, 4, 9]);
        assert_eq!(square_all_concurrently(vec![]), Vec::<i32>::new());
    }
}
