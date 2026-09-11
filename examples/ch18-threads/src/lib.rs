// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: spawn_join
use std::thread;

/// Spawns a thread that takes ownership of `data` (via the `move`
/// closure), doubles every element, and returns the result through
/// `JoinHandle::join()`.
///
/// The `move` keyword isn't decoration: it's what makes the compiler
/// require that nothing else in this function can still touch `data`
/// after the spawn. `pthread_create(&t, NULL, fn, &data)` in C compiles
/// no matter what the calling thread does with `data` afterward — Rust's
/// ownership transfer is what turns "who's allowed to touch this" from a
/// convention into a compiler-checked fact.
pub fn double_in_background(data: Vec<i32>) -> Vec<i32> {
    let handle = thread::spawn(move || data.into_iter().map(|x| x * 2).collect());
    handle.join().expect("background thread panicked")
}
// ANCHOR_END: spawn_join

// ANCHOR: thread_panic
/// Runs `f` on a background thread and reports whether it panicked,
/// instead of letting the panic take down the whole process.
///
/// A panic inside a spawned thread unwinds *that thread only* and is
/// caught at the thread boundary; `join()` returns `Err` with the panic
/// payload. Contrast with C: an unhandled fault in one thread (a bad
/// pointer dereference, say) typically delivers a process-wide signal
/// (SIGSEGV) that takes every thread down with it, not just the one that
/// misbehaved.
pub fn run_isolated<F>(f: F) -> Result<(), &'static str>
where
    F: FnOnce() + Send + 'static,
{
    match thread::spawn(f).join() {
        Ok(()) => Ok(()),
        Err(_) => Err("background thread panicked, but the process is still alive"),
    }
}
// ANCHOR_END: thread_panic

// ANCHOR: scoped_threads
/// Sums `data` by splitting it across two scoped threads that each
/// borrow a slice of it directly — no `Arc`, no cloning, no `'static`
/// bound required.
///
/// `thread::scope` guarantees every thread spawned inside it is joined
/// before the scope returns, which is what lets the borrow checker allow
/// threads to hold references to stack data that doesn't outlive the
/// function. In C, the equivalent — handing a spawned thread a pointer
/// into the parent's stack frame — compiles fine and is only safe if you,
/// by hand, guarantee the parent doesn't return first; here the compiler
/// guarantees it for you.
pub fn scoped_sum(data: &[i32]) -> i32 {
    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);

    thread::scope(|s| {
        let left_handle = s.spawn(|| left.iter().sum::<i32>());
        let right_handle = s.spawn(|| right.iter().sum::<i32>());
        left_handle.join().unwrap() + right_handle.join().unwrap()
    })
}
// ANCHOR_END: scoped_threads

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_in_background_works() {
        assert_eq!(double_in_background(vec![1, 2, 3]), vec![2, 4, 6]);
    }

    #[test]
    fn run_isolated_reports_panics_without_crashing() {
        assert_eq!(run_isolated(|| {}), Ok(()));
        assert!(run_isolated(|| panic!("boom")).is_err());
    }

    #[test]
    fn scoped_sum_matches_sequential_sum() {
        let data: Vec<i32> = (1..=100).collect();
        assert_eq!(scoped_sum(&data), data.iter().sum());
    }
}
