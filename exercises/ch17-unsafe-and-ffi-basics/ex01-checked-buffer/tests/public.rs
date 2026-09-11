// SPDX-License-Identifier: MIT OR Apache-2.0
use ch17_ex01_checked_buffer::checked_get_unchecked;

#[test]
fn in_bounds_index_returns_the_value() {
    let data = [10, 20, 30];
    assert_eq!(checked_get_unchecked(&data, 0), Some(10));
    assert_eq!(checked_get_unchecked(&data, 2), Some(30));
}

#[test]
fn out_of_bounds_index_returns_none_not_ub() {
    let data = [10, 20, 30];
    assert_eq!(checked_get_unchecked(&data, 3), None);
    assert_eq!(checked_get_unchecked(&data, usize::MAX), None);
}

#[test]
fn empty_slice_is_always_out_of_bounds() {
    let data: [i32; 0] = [];
    assert_eq!(checked_get_unchecked(&data, 0), None);
}
