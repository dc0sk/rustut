// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch19-send-sync/ex01-shared-counter.

use std::sync::Mutex;

pub struct SharedCounter {
    value: Mutex<i64>,
}

impl SharedCounter {
    pub fn new() -> Self {
        SharedCounter {
            value: Mutex::new(0),
        }
    }

    pub fn add(&self, amount: i64) -> i64 {
        let mut guard = self.value.lock().expect("mutex poisoned");
        *guard += amount;
        *guard
    }

    pub fn get(&self) -> i64 {
        *self.value.lock().expect("mutex poisoned")
    }
}

impl Default for SharedCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn matches_public_test_expectations() {
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
}
