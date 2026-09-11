// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: concurrent_sleeps
use std::time::Duration;
use tokio::time::sleep;

/// Spawns `count` independent tasks, each `.await`-ing a `delay_ms` sleep,
/// and waits for all of them. Because `sleep(...).await` is a genuine
/// suspend point — it hands control back to the tokio runtime instead of
/// blocking a thread — the runtime can make progress on every spawned task
/// while any one of them is "waiting." Compare the elapsed time this takes
/// (see the guided binary and the test below) against what `count *
/// delay_ms` sequential sleeps would cost.
pub async fn run_concurrent(count: usize, delay_ms: u64) -> usize {
    let mut handles = Vec::with_capacity(count);
    for _ in 0..count {
        handles.push(tokio::spawn(async move {
            sleep(Duration::from_millis(delay_ms)).await;
        }));
    }

    let mut completed = 0;
    for handle in handles {
        if handle.await.is_ok() {
            completed += 1;
        }
    }
    completed
}
// ANCHOR_END: concurrent_sleeps

// ANCHOR: blocking_vs_yielding
/// The right way to "wait" inside an async fn: `.await` on an async sleep
/// yields control back to the runtime, so other tasks on the same worker
/// thread keep making progress while this one waits.
pub async fn yields_politely() {
    sleep(Duration::from_millis(1)).await;
}

/// The trap: `std::thread::sleep` is a *blocking* call with no `.await`.
/// Calling it inside an async fn blocks the entire OS thread the tokio
/// runtime is using to drive potentially many other tasks — none of them
/// can make progress until this one wakes up, even though nothing here
/// looks different from ordinary sequential code. This is the async
/// equivalent of a C `epoll` event loop's single callback calling a
/// blocking `read()` and freezing every other file descriptor's handling
/// until it returns. It compiles and "works" for one task at a time,
/// which is exactly what makes it a dangerous, easy-to-miss mistake — it
/// is not exercised by this crate's tests on purpose, to avoid slowing
/// down every CI run for the sake of one cautionary example.
pub async fn blocks_the_runtime_thread(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis));
}
// ANCHOR_END: blocking_vs_yielding

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn tasks_run_concurrently_not_sequentially() {
        let start = Instant::now();
        let completed = run_concurrent(5, 50).await;
        let elapsed = start.elapsed();

        assert_eq!(completed, 5);
        // Sequential would cost ~250ms; concurrent should stay well under
        // that. The bound is generous on purpose to avoid CI flakiness.
        assert!(
            elapsed.as_millis() < 200,
            "took {elapsed:?}, expected well under 250ms if the 5 tasks truly ran concurrently"
        );
    }

    #[tokio::test]
    async fn yields_politely_compiles_and_runs() {
        yields_politely().await;
    }
}
