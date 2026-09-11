// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: move
/// Assigning a `String` (a heap-allocated, non-`Copy` type) moves it: the
/// old binding is no longer usable afterward. This is not a C struct
/// assignment (which bitwise-copies, leaving two independent copies that
/// can now diverge and double-free), and it is not a raw pointer copy
/// (which aliases the same memory with no tracking of who's responsible
/// for it) — it's the compiler transferring ownership and then refusing
/// to let the old name be used again.
///
/// ```compile_fail
/// let s1 = String::from("hello");
/// let s2 = s1;
/// println!("{s1}"); // error[E0382]: borrow of moved value: `s1`
/// ```
///
/// Captured on this book's toolchain, the real compiler output is:
///
/// ```text
/// error[E0382]: borrow of moved value: `s1`
///  --> moved.rs:4:16
///   |
/// 2 |     let s1 = String::from("hello");
///   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
/// 3 |     let s2 = s1;
///   |              -- value moved here
/// 4 |     println!("{s1}");
///   |                ^^ value borrowed here after move
///   |
/// help: consider cloning the value if the performance cost is acceptable
///   |
/// 3 |     let s2 = s1.clone();
///   |                ++++++++
/// ```
pub fn move_examples_are_doctests_only() {}
// ANCHOR_END: move

// ANCHOR: copy_vs_clone
/// `i32` implements `Copy`: assignment bitwise-copies it (like a C `int`
/// assignment), and the original stays perfectly usable — there is no
/// move at all for `Copy` types, because there is nothing exclusive to
/// transfer.
pub fn copy_is_not_a_move() -> (i32, i32) {
    let a = 5;
    let b = a; // copies; does NOT move
    (a, b) // both usable
}

/// `String` does not implement `Copy`. To get a second, independent copy
/// of its heap data — the explicit equivalent of a C `strdup` — call
/// `.clone()`. Unlike the move above, both bindings remain valid
/// afterward, because `.clone()` allocates a genuinely separate buffer.
pub fn clone_gives_a_second_owner() -> (String, String) {
    let original = String::from("hello");
    let duplicate = original.clone();
    (original, duplicate) // both usable — two independent allocations
}
// ANCHOR_END: copy_vs_clone

// ANCHOR: ownership_through_functions
/// Ownership flows into a function exactly like it flows into any other
/// binding: `data` is moved in, the caller's original variable is gone.
/// Ownership can flow back out through the return value just as
/// naturally — no manual "caller must free this" contract to document
/// (compare to a C function that returns a `malloc`'d pointer, where nothing
/// in the type signature tells you who's responsible for freeing it).
pub fn take_ownership_and_give_it_back(mut data: Vec<i32>) -> Vec<i32> {
    data.push(0);
    data
}
// ANCHOR_END: ownership_through_functions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_types_are_not_moved() {
        assert_eq!(copy_is_not_a_move(), (5, 5));
    }

    #[test]
    fn clone_produces_two_independent_owners() {
        let (original, duplicate) = clone_gives_a_second_owner();
        assert_eq!(original, "hello");
        assert_eq!(duplicate, "hello");
    }

    #[test]
    fn ownership_round_trips_through_a_function() {
        assert_eq!(
            take_ownership_and_give_it_back(vec![1, 2, 3]),
            vec![1, 2, 3, 0]
        );
    }
}
