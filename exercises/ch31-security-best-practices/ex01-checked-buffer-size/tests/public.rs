// SPDX-License-Identifier: MIT OR Apache-2.0
use ch31_ex01_checked_buffer_size::buffer_size;

#[test]
fn computes_the_ordinary_case() {
    assert_eq!(buffer_size(16, 10, 100), Some(16 + 10 * 100));
}

#[test]
fn zero_records_is_just_the_header() {
    assert_eq!(buffer_size(16, 0, 100), Some(16));
}

#[test]
fn multiplication_overflow_returns_none_not_a_wrapped_value() {
    // record_count * record_size overflows u32 here.
    assert_eq!(buffer_size(0, u32::MAX, 2), None);
}

#[test]
fn addition_overflow_returns_none_not_a_wrapped_value() {
    // record_count * record_size fits, but adding header_bytes overflows.
    assert_eq!(buffer_size(u32::MAX, 1, 1), None);
}

#[test]
fn right_at_the_boundary_still_succeeds() {
    // Chosen so header + count * size lands exactly on u32::MAX: no overflow.
    assert_eq!(buffer_size(u32::MAX - 100, 10, 10), Some(u32::MAX));
}
