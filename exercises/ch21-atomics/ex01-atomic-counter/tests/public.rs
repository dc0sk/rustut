// SPDX-License-Identifier: MIT OR Apache-2.0
use ch21_ex01_atomic_counter::record_request;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

#[test]
fn increments_below_max() {
    let counter = AtomicU32::new(0);
    assert!(record_request(&counter, 5));
    assert_eq!(counter.load(Ordering::Relaxed), 1);
}

#[test]
fn rejects_at_max() {
    let counter = AtomicU32::new(5);
    assert!(!record_request(&counter, 5));
    assert_eq!(counter.load(Ordering::Relaxed), 5);
}

#[test]
fn never_overshoots_under_concurrent_load() {
    let counter = Arc::new(AtomicU32::new(0));
    let max = 100;

    let handles: Vec<_> = (0..16)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                let mut accepted = 0;
                for _ in 0..50 {
                    if record_request(&counter, max) {
                        accepted += 1;
                    }
                }
                accepted
            })
        })
        .collect();

    let total_accepted: u32 = handles.into_iter().map(|h| h.join().unwrap()).sum();

    assert_eq!(counter.load(Ordering::Relaxed), max);
    assert_eq!(total_accepted, max);
}
