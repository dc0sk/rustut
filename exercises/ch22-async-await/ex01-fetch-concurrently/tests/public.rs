// SPDX-License-Identifier: MIT OR Apache-2.0
use ch22_ex01_fetch_concurrently::fetch_all;
use std::time::Instant;

#[tokio::test]
async fn preserves_order() {
    let results = fetch_all(vec![10, 20, 30]).await;
    assert_eq!(results, vec![10, 20, 30]);
}

#[tokio::test]
async fn runs_concurrently_not_sequentially() {
    let start = Instant::now();
    let results = fetch_all(vec![60, 60, 60, 60]).await;
    let elapsed = start.elapsed();

    assert_eq!(results, vec![60, 60, 60, 60]);
    // Sequential would cost ~240ms; concurrent should stay well under
    // that. The bound is generous on purpose to avoid CI flakiness.
    assert!(
        elapsed.as_millis() < 150,
        "took {elapsed:?}, expected well under 240ms if all 4 fetches ran concurrently"
    );
}

#[tokio::test]
async fn empty_input_returns_empty_output() {
    let results = fetch_all(vec![]).await;
    assert!(results.is_empty());
}
