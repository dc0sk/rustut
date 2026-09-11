// SPDX-License-Identifier: MIT OR Apache-2.0
use ch05_ex01_move_and_clone::{append_owned, double_in_place, duplicate};

#[test]
fn append_owned_combines_both_strings() {
    assert_eq!(
        append_owned(String::from("hello"), String::from(" world")),
        "hello world"
    );
}

#[test]
fn append_owned_works_with_an_empty_suffix() {
    assert_eq!(append_owned(String::from("hello"), String::new()), "hello");
}

#[test]
fn double_in_place_doubles_every_element() {
    assert_eq!(double_in_place(vec![1, 2, 3]), vec![2, 4, 6]);
}

#[test]
fn double_in_place_handles_an_empty_vec() {
    let empty: Vec<i32> = Vec::new();
    assert_eq!(double_in_place(empty), Vec::<i32>::new());
}

#[test]
fn duplicate_matches_the_original_contents() {
    assert_eq!(duplicate("abc"), "abc");
}

#[test]
fn duplicate_leaves_the_original_usable() {
    let original = String::from("abc");
    let copy = duplicate(&original);
    // If this compiles and both assertions pass, `original` was never
    // moved — only borrowed — by `duplicate`.
    assert_eq!(copy, "abc");
    assert_eq!(original, "abc");
}
