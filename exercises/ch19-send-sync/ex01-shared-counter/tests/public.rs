// SPDX-License-Identifier: MIT OR Apache-2.0
use ch19_ex01_shared_counter::SharedCounter;
use std::sync::Arc;
use std::thread;

#[test]
fn starts_at_zero() {
    let counter = SharedCounter::new();
    assert_eq!(counter.get(), 0);
}

#[test]
fn add_returns_the_new_value() {
    let counter = SharedCounter::new();
    assert_eq!(counter.add(5), 5);
    assert_eq!(counter.add(3), 8);
}

#[test]
fn concurrent_adds_are_race_free() {
    let counter = Arc::new(SharedCounter::new());
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..1000 {
                    counter.add(1);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(counter.get(), 8000);
}
