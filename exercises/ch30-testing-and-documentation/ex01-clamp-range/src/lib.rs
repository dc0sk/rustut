// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

/// Returns a copy of `values` with every element clamped into
/// `lo..=hi`.
///
/// # Panics
/// Panics if `lo > hi` — an invalid range, not a value to silently
/// tolerate.
pub fn clamp_all(values: &[i32], lo: i32, hi: i32) -> Vec<i32> {
    todo!("assert lo <= hi, then map each value through i32::clamp(lo, hi)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_outside_the_range_are_pulled_in() {
        assert_eq!(clamp_all(&[-5, 0, 5, 10, 15], 0, 10), vec![0, 0, 5, 10, 10]);
    }

    #[test]
    fn empty_input_is_empty_output() {
        assert_eq!(clamp_all(&[], 0, 10), Vec::<i32>::new());
    }

    #[test]
    #[should_panic(expected = "lo")]
    fn an_invalid_range_panics() {
        clamp_all(&[1, 2, 3], 10, 0);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn every_output_value_is_within_the_range(
            values in prop::collection::vec(-1000i32..1000, 0..50),
            a in -1000i32..1000,
            b in -1000i32..1000,
        ) {
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            let clamped = clamp_all(&values, lo, hi);
            prop_assert_eq!(clamped.len(), values.len());
            for v in clamped {
                prop_assert!(v >= lo && v <= hi);
            }
        }
    }
}
