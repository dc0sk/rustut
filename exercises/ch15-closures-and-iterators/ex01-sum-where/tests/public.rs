// SPDX-License-Identifier: MIT OR Apache-2.0
use ch15_ex01_sum_where::sum_where;

#[test]
fn sums_squares_of_evens() {
    let data = [1, 2, 3, 4, 5, 6];
    let result = sum_where(&data, |x| x % 2 == 0, |x| i64::from(x) * i64::from(x));
    assert_eq!(result, 4 + 16 + 36);
}

#[test]
fn empty_slice_sums_to_zero() {
    assert_eq!(sum_where(&[], |_| true, i64::from), 0);
}

#[test]
fn keep_false_for_all_sums_to_zero() {
    let data = [1, 2, 3];
    assert_eq!(sum_where(&data, |_| false, i64::from), 0);
}

#[test]
fn keep_can_close_over_outside_state() {
    let threshold = 3;
    let data = [1, 2, 3, 4, 5];
    let result = sum_where(&data, |x| x > threshold, i64::from);
    assert_eq!(result, 4 + 5);
}
