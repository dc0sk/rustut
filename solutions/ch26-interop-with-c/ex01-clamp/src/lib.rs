// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch26-interop-with-c/ex01-clamp.

unsafe extern "C" {
    safe fn round_to_nearest(value: f64) -> i32;
}

pub fn round_half_away_from_zero(value: f64) -> i32 {
    round_to_nearest(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        assert_eq!(round_half_away_from_zero(2.5), 3);
        assert_eq!(round_half_away_from_zero(-2.5), -3);
    }
}
