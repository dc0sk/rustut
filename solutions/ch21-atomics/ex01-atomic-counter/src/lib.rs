// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch21-atomics/ex01-atomic-counter.

use std::sync::atomic::{AtomicU32, Ordering};

pub fn record_request(counter: &AtomicU32, max: u32) -> bool {
    let mut current = counter.load(Ordering::Relaxed);
    loop {
        if current >= max {
            return false;
        }
        match counter.compare_exchange_weak(
            current,
            current + 1,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => return true,
            Err(actual) => current = actual,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        let counter = AtomicU32::new(0);
        assert!(record_request(&counter, 5));
        assert_eq!(counter.load(Ordering::Relaxed), 1);

        let counter = AtomicU32::new(5);
        assert!(!record_request(&counter, 5));
    }
}
