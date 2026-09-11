// SPDX-License-Identifier: MIT OR Apache-2.0
use ch06_ex01_largest_without_cloning::{grow_then_summarize, largest};

#[test]
fn finds_the_largest() {
    let nums = [3, 7, 2, 9, 4];
    assert_eq!(largest(&nums), Some(&9));
}

#[test]
fn empty_slice_has_no_largest() {
    let nums: [i32; 0] = [];
    assert_eq!(largest(&nums), None);
}

#[test]
fn single_element_is_its_own_largest() {
    assert_eq!(largest(&[5]), Some(&5));
}

#[test]
fn negative_numbers_are_compared_correctly() {
    assert_eq!(largest(&[-5, -1, -9]), Some(&-1));
}

#[test]
fn grow_then_summarize_reports_new_state() {
    let mut data = vec![1, 2, 3, 4];
    assert_eq!(grow_then_summarize(&mut data, 5), "5 numbers, sum 15");
    assert_eq!(data, vec![1, 2, 3, 4, 5]);
}
