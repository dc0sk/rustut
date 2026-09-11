// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch31-security-best-practices/ex01-checked-buffer-size.

pub fn buffer_size(header_bytes: u32, record_count: u32, record_size: u32) -> Option<u32> {
    record_count
        .checked_mul(record_size)
        .and_then(|records_total| records_total.checked_add(header_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        assert_eq!(buffer_size(16, 10, 100), Some(1016));
        assert_eq!(buffer_size(0, u32::MAX, 2), None);
        assert_eq!(buffer_size(u32::MAX, 1, 1), None);
        assert_eq!(buffer_size(u32::MAX - 100, 10, 10), Some(u32::MAX));
    }
}
