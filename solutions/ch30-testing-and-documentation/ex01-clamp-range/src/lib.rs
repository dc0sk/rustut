// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch30-testing-and-documentation/ex01-clamp-range.

pub fn clamp_all(values: &[i32], lo: i32, hi: i32) -> Vec<i32> {
    assert!(lo <= hi, "invalid range: lo ({lo}) must be <= hi ({hi})");
    values.iter().map(|&v| v.clamp(lo, hi)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        assert_eq!(clamp_all(&[-5, 0, 5, 10, 15], 0, 10), vec![0, 0, 5, 10, 10]);
        assert_eq!(clamp_all(&[], 0, 10), Vec::<i32>::new());
    }

    #[test]
    #[should_panic(expected = "lo")]
    fn invalid_range_panics() {
        clamp_all(&[1, 2, 3], 10, 0);
    }
}
