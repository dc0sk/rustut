// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: index_loop
/// Deliberately written the C-style, indexed way for comparison with
/// `sum_even_squares_iter` below — clippy's `needless_range_loop` lint
/// would otherwise (correctly, in normal code) tell you to write this as
/// an iterator instead, which is exactly the point being illustrated, so
/// it's silenced here on purpose rather than worked around.
#[allow(clippy::needless_range_loop)]
pub fn sum_even_squares_loop(data: &[i32]) -> i64 {
    let mut total: i64 = 0;
    for i in 0..data.len() {
        let v = data[i];
        if v % 2 == 0 {
            total += i64::from(v) * i64::from(v);
        }
    }
    total
}
// ANCHOR_END: index_loop

// ANCHOR: iterator_chain
pub fn sum_even_squares_iter(data: &[i32]) -> i64 {
    data.iter()
        .filter(|&&v| v % 2 == 0)
        .map(|&v| i64::from(v) * i64::from(v))
        .sum()
}
// ANCHOR_END: iterator_chain

// ANCHOR: closure_vs_fn_pointer
/// Takes any closure or function matching `Fn(i32) -> i32`. In C you'd
/// need a function pointer *plus* a separate `void *context` parameter
/// threaded through by hand to capture anything; here the compiler builds
/// that "context struct" for you, and the closure's type encodes exactly
/// what it captured and how (by reference, by value, or by move).
pub fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}
// ANCHOR_END: closure_vs_fn_pointer

// ANCHOR: fnmut_example
/// Returns a closure that mutates state it captured across calls — this
/// is `FnMut`, not `Fn`: calling it requires `&mut` access to the closure
/// itself, which the compiler enforces the same way it enforces `&mut`
/// access to anything else.
pub fn make_counter() -> impl FnMut() -> u32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}
// ANCHOR_END: fnmut_example

/// ```
/// use ch15_closures_and_iterators_example::apply_twice;
/// let offset = 3;
/// let add_offset = |x| x + offset; // captures `offset` by reference
/// assert_eq!(apply_twice(add_offset, 10), 16);
/// ```
///
/// ```
/// use ch15_closures_and_iterators_example::make_counter;
/// let mut next = make_counter();
/// assert_eq!(next(), 1);
/// assert_eq!(next(), 2);
/// assert_eq!(next(), 3);
/// ```
fn _doc_anchor() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_and_iterator_versions_agree() {
        for data in [
            &[][..],
            &[1, 2, 3, 4, 5, 6][..],
            &[-4, -3, -2, -1, 0][..],
            &[7, 9, 11][..],
        ] {
            assert_eq!(sum_even_squares_loop(data), sum_even_squares_iter(data));
        }
    }

    #[test]
    fn known_value() {
        assert_eq!(sum_even_squares_iter(&[1, 2, 3, 4, 5, 6]), 4 + 16 + 36);
    }
}
