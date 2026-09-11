// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch17-unsafe-and-ffi-basics/ex01-checked-buffer.

pub fn checked_get_unchecked(data: &[i32], index: usize) -> Option<i32> {
    if index >= data.len() {
        return None;
    }
    // SAFETY: just checked `index < data.len()` above, so this index is
    // in bounds for `data`.
    Some(unsafe { *data.get_unchecked(index) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        let data = [10, 20, 30];
        assert_eq!(checked_get_unchecked(&data, 1), Some(20));
        assert_eq!(checked_get_unchecked(&data, 3), None);
    }
}
