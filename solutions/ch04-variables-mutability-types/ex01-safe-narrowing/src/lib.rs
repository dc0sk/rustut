// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch04-variables-mutability-types/ex01-safe-narrowing.

pub fn to_port(value: i64) -> Result<u16, i64> {
    u16::try_from(value).map_err(|_| value)
}

pub fn checked_sum(a: u8, b: u8) -> Option<u8> {
    a.checked_add(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        assert_eq!(to_port(8080), Ok(8080));
        assert_eq!(to_port(70_000), Err(70_000));
        assert_eq!(checked_sum(200, 100), None);
        assert_eq!(checked_sum(255, 0), Some(255));
    }
}
