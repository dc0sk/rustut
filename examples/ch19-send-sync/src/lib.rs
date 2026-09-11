// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: rc_not_send
/// `Rc<T>`'s reference count is a plain (non-atomic) integer, incremented
/// and decremented on every clone/drop. Two threads racing on that
/// counter would be a data race on the count itself — so `Rc<T>` is
/// deliberately **not** `Send`, and the compiler enforces it: this does
/// not compile.
///
/// ```rust,compile_fail
/// use std::rc::Rc;
/// use std::thread;
///
/// let shared = Rc::new(42);
/// let handle = thread::spawn(move || {
///     println!("{}", shared);
/// });
/// handle.join().unwrap();
/// ```
///
/// The real error (captured against this crate's toolchain):
///
/// ```text
/// error[E0277]: `Rc<i32>` cannot be sent between threads safely
///   = help: within `{closure@...}`, the trait `Send` is not implemented for `Rc<i32>`
///   note: required by a bound in `spawn`
/// ```
///
/// `Arc<T>` is the same API with an atomic reference count, and *is*
/// `Send`/`Sync` — pay the (small) cost of an atomic increment only when
/// you actually need to share across threads.
pub fn rc_docs_only() {}
// ANCHOR_END: rc_not_send

// ANCHOR: mutex_guard
use std::sync::Mutex;

/// The point that has no C equivalent: there is no way to read or write
/// the `i32` inside `counter` without going through `.lock()`. The data
/// and its lock are *one value*, not two things a caller has to
/// remember to use together.
///
/// Compare to C's `pthread_mutex_t mu; int counter;` — `counter` is an
/// ordinary variable. Nothing in the type system stops any function,
/// anywhere in the codebase, from reading or writing `counter` directly
/// without ever calling `pthread_mutex_lock(&mu)`. The association
/// between the lock and the data it protects is a comment, a naming
/// convention, a code review discipline — never something the compiler
/// checks. This is the concrete mechanism behind Chapter 1's "data race"
/// row: the bug class isn't just harder to hit in Rust, the specific
/// mistake ("touched the data without taking the lock") has no syntax
/// that type-checks.
pub fn increment(counter: &Mutex<i32>) {
    let mut guard = counter.lock().expect("mutex poisoned");
    *guard += 1;
}
// ANCHOR_END: mutex_guard

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn increment_is_race_free_across_threads() {
        let counter = Arc::new(Mutex::new(0));
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let counter = Arc::clone(&counter);
                thread::spawn(move || {
                    for _ in 0..1000 {
                        increment(&counter);
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(*counter.lock().unwrap(), 8000);
    }
}
