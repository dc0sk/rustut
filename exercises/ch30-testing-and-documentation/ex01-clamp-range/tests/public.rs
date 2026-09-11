// SPDX-License-Identifier: MIT OR Apache-2.0
use ch30_ex01_clamp_range::clamp_all;

#[test]
fn clamps_values_outside_the_range() {
    assert_eq!(clamp_all(&[-5, 0, 5, 10, 15], 0, 10), vec![0, 0, 5, 10, 10]);
}

#[test]
fn values_already_in_range_are_unchanged() {
    assert_eq!(clamp_all(&[1, 2, 3], 0, 10), vec![1, 2, 3]);
}

#[test]
#[should_panic(expected = "lo")]
fn an_invalid_range_panics() {
    clamp_all(&[1, 2, 3], 10, 0);
}
