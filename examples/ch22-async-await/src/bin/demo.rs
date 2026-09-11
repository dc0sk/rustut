// SPDX-License-Identifier: MIT OR Apache-2.0
use ch22_async_await_example::run_concurrent;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let completed = run_concurrent(5, 100).await;
    let elapsed = start.elapsed();
    println!(
        "{completed} tasks completed in {elapsed:?} — not ~500ms, because they ran concurrently"
    );
}
