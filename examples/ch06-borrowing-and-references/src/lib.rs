// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: sum_slice
/// Sums a slice without taking ownership of it or copying its contents.
///
/// The parameter is a *fat pointer*: a `(pointer, length)` pair the
/// compiler tracks as one unit. Compare to the C signature for the same
/// job — `int sum(const int *data, size_t len)` — where nothing stops a
/// caller from passing a `len` that doesn't match `data`'s real
/// allocation. Here, `data.len()` can never disagree with `data`'s actual
/// bounds: there is no way to construct a `&[i32]` whose length lies.
pub fn sum_slice(data: &[i32]) -> i32 {
    data.iter().sum()
}
// ANCHOR_END: sum_slice

// ANCHOR: aliasing_ok
/// Two immutable borrows of the same data, alive at the same time, are
/// fine — reading never races with reading, so the compiler allows any
/// number of `&T` borrows simultaneously.
pub fn describe_twice(data: &[i32]) -> String {
    let first = data;
    let second = data;
    format!("{} numbers, sum {}", first.len(), sum_slice(second))
}
// ANCHOR_END: aliasing_ok

// ANCHOR: aliasing_sequenced
/// A mutable borrow and an immutable borrow of the same `Vec`, used one
/// after the other rather than at the same time, also compiles: the
/// compiler tracks a borrow's lifetime by how long it's *actually used*
/// (an analysis called non-lexical lifetimes), not by textual scope. The
/// `&mut Vec<i32>` borrow used by `push` has already ended by the time
/// `sum_slice` immutably borrows the same data.
pub fn grow_then_report(data: &mut Vec<i32>, extra: i32) -> String {
    data.push(extra);
    let total = sum_slice(data);
    format!("now {} numbers, sum {}", data.len(), total)
}
// ANCHOR_END: aliasing_sequenced

// ANCHOR: aliasing_violation
/// What the compiler *does* reject: a mutable borrow while an immutable
/// borrow of the same value is still in use. This is the exact shape of
/// a C bug where one pointer mutates a buffer while another pointer is
/// mid-read through it — except here it's a compile error, not a data
/// race that only shows up under load:
///
/// ```rust,compile_fail
/// let mut data = vec![1, 2, 3];
/// let view = &data;              // immutable borrow starts
/// data.push(4);                  // ERROR: `data` is also borrowed as immutable
/// println!("{:?}", view);        // immutable borrow still in use here
/// ```
///
/// The real compiler output for that snippet (rustc 1.98, stable, and
/// this wording is stable across recent releases since it's a core
/// borrow-check diagnostic):
///
/// ```text
/// error[E0502]: cannot borrow `data` as mutable because it is also borrowed as immutable
///  --> aliasing.rs:4:5
///   |
/// 3 |     let view = &data;
///   |                ----- immutable borrow occurs here
/// 4 |     data.push(4);
///   |     ^^^^^^^^^^^^ mutable borrow occurs here
/// 5 |     println!("{:?}", view);
///   |                      ---- immutable borrow later used here
/// ```
pub fn aliasing_violation_reference_only() {}
// ANCHOR_END: aliasing_violation
