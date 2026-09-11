// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch22-async-await/ex01-fetch-concurrently.

use std::time::Duration;
use tokio::time::sleep;

async fn fetch_one(delay_ms: u64) -> u64 {
    sleep(Duration::from_millis(delay_ms)).await;
    delay_ms
}

pub async fn fetch_all(delays_ms: Vec<u64>) -> Vec<u64> {
    let handles: Vec<_> = delays_ms
        .into_iter()
        .map(|delay| tokio::spawn(fetch_one(delay)))
        .collect();

    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        results.push(handle.await.expect("fetch task panicked"));
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn matches_public_test_expectations() {
        assert_eq!(fetch_all(vec![10, 20, 30]).await, vec![10, 20, 30]);

        let start = Instant::now();
        let results = fetch_all(vec![60, 60, 60, 60]).await;
        assert_eq!(results, vec![60, 60, 60, 60]);
        assert!(start.elapsed().as_millis() < 150);

        assert!(fetch_all(vec![]).await.is_empty());
    }
}
