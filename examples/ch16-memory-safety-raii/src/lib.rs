// SPDX-License-Identifier: MIT OR Apache-2.0
//! Guided example for Chapter 16 — Drop order, unwind-safety, and the leak
//! caveat.

use std::cell::RefCell;
use std::rc::Rc;

type Log = Rc<RefCell<Vec<&'static str>>>;

// ANCHOR: drop_order
pub struct LoggedGuard {
    name: &'static str,
    log: Log,
}

impl LoggedGuard {
    pub fn new(name: &'static str, log: Log) -> Self {
        LoggedGuard { name, log }
    }
}

impl Drop for LoggedGuard {
    fn drop(&mut self) {
        self.log.borrow_mut().push(self.name);
    }
}

/// Stack-local values drop in **reverse** declaration order — the last
/// one created is the first one destroyed. It's the same rule a call
/// stack itself unwinds by; there's nothing extra to remember, unlike a
/// hand-written C cleanup sequence where the order is whatever you typed.
pub fn drop_order_demo(log: Log) {
    let _first = LoggedGuard::new("first", log.clone());
    let _second = LoggedGuard::new("second", log.clone());
    let _third = LoggedGuard::new("third", log.clone());
    // Drop order on scope exit: third, second, first.
}
// ANCHOR_END: drop_order

// ANCHOR: drop_through_panic
/// `Drop` runs even when a panic unwinds through this scope — contrast
/// with C, where an uncaught signal or a `longjmp` past this point skips
/// every cleanup call you would have written after it. Here there is
/// nothing to skip: the guard's release isn't a line of code in this
/// function body at all, so unwinding can't jump past it.
pub fn drop_runs_even_on_panic(log: Log) {
    let _guard = LoggedGuard::new("panicking-scope", log.clone());
    panic!("boom");
}
// ANCHOR_END: drop_through_panic

// ANCHOR: leak_caveat
/// A node in a doubly-linked `Rc` cycle. `Drop` only runs once a value's
/// last owner disappears — an `Rc` cycle means each node keeps the other
/// alive forever, so the strong count never reaches zero and neither
/// `Drop` impl ever runs. This is a real, safe-Rust memory leak: no
/// `unsafe`, no bug in the borrow checker, just a data structure that
/// keeps itself alive. `Drop`'s guarantee is against use-after-free and
/// double-free, never against leaks — `std::mem::forget` is the other
/// standard way to produce one deliberately.
pub struct Node {
    name: &'static str,
    next: RefCell<Option<Rc<Node>>>,
    log: Log,
}

impl Drop for Node {
    fn drop(&mut self) {
        self.log.borrow_mut().push(self.name);
    }
}

pub fn leak_a_cycle(log: Log) -> (Rc<Node>, Rc<Node>) {
    let a = Rc::new(Node {
        name: "a",
        next: RefCell::new(None),
        log: log.clone(),
    });
    let b = Rc::new(Node {
        name: "b",
        next: RefCell::new(Some(Rc::clone(&a))),
        log: log.clone(),
    });
    *a.next.borrow_mut() = Some(Rc::clone(&b));
    (a, b)
}
// ANCHOR_END: leak_caveat

/// Calling a destructor directly, rather than letting scope-exit call it,
/// is rejected at compile time — if it were allowed, the value would
/// still be sitting there afterward, `Drop`'s "already released" flag
/// notwithstanding, which is exactly the double-free shape all over
/// again. `std::mem::drop` is not special syntax: it's an ordinary
/// function that takes ownership of its argument and does nothing else,
/// so the value's normal end-of-scope `Drop` fires immediately instead.
///
/// ```rust,compile_fail
/// struct Foo;
/// impl Drop for Foo {
///     fn drop(&mut self) {}
/// }
/// fn main() {
///     let f = Foo;
///     f.drop(); // error[E0040]: explicit use of destructor method
/// }
/// ```
///
/// The real compiler output (rustc 1.98, stable):
///
/// ```text
/// error[E0040]: explicit use of destructor method
///  --> src/lib.rs:7:7
///   |
/// 7 |     f.drop();
///   |       ^^^^ explicit destructor calls not allowed
///   |
/// help: consider using `drop` function
///   |
/// 7 -     f.drop();
/// 7 +     drop(f);
///   |
/// ```
pub fn explicit_drop_is_a_compile_error() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drops_in_reverse_declaration_order() {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        drop_order_demo(log.clone());
        assert_eq!(*log.borrow(), vec!["third", "second", "first"]);
    }

    #[test]
    fn drop_runs_during_unwind() {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let log_for_closure = log.clone();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            drop_runs_even_on_panic(log_for_closure);
        }));
        assert!(result.is_err(), "the closure was expected to panic");
        assert_eq!(*log.borrow(), vec!["panicking-scope"]);
    }

    #[test]
    fn rc_cycle_leaks_instead_of_dropping() {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let (a, b) = leak_a_cycle(log.clone());
        drop(a);
        drop(b);
        assert!(
            log.borrow().is_empty(),
            "an Rc cycle must leak, not drop, both nodes"
        );
    }
}
