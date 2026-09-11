// SPDX-License-Identifier: MIT OR Apache-2.0
use ch04_ex01_safe_narrowing::{checked_sum, to_port};

#[test]
fn in_range_value_converts() {
    assert_eq!(to_port(8080), Ok(8080));
}

#[test]
fn out_of_range_value_is_rejected_not_truncated() {
    assert_eq!(to_port(70_000), Err(70_000));
    assert_eq!(to_port(-1), Err(-1));
}

#[test]
fn checked_sum_returns_value_when_it_fits() {
    assert_eq!(checked_sum(100, 50), Some(150));
}

#[test]
fn checked_sum_returns_none_on_overflow() {
    assert_eq!(checked_sum(200, 100), None);
}

#[test]
fn checked_sum_handles_the_boundary() {
    assert_eq!(checked_sum(255, 0), Some(255));
    assert_eq!(checked_sum(255, 1), None);
}
