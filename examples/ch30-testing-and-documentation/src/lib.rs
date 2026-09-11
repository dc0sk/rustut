// SPDX-License-Identifier: MIT OR Apache-2.0
//! Guided example for Chapter 30 — the four flavors of Rust testing.

// ANCHOR: average
/// Returns the arithmetic mean of `values`.
///
/// # Panics
/// Panics if `values` is empty — there's no meaningful average of zero
/// numbers, and forcing the caller to guarantee a non-empty slice up
/// front is simpler here than threading an `Option` through every call
/// site. (A production API might prefer returning `Option<f64>` instead;
/// this chapter deliberately uses the panicking version so there's
/// something for `#[should_panic]`, below, to test.)
///
/// ```
/// assert_eq!(ch30_testing_and_documentation_example::average(&[2, 4, 6]), 4.0);
/// ```
pub fn average(values: &[i32]) -> f64 {
    assert!(!values.is_empty(), "average of an empty slice is undefined");
    values.iter().map(|&v| f64::from(v)).sum::<f64>() / values.len() as f64
}
// ANCHOR_END: average

// ANCHOR: unit_test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_of_a_few_numbers() {
        assert_eq!(average(&[1, 2, 3]), 2.0);
    }

    #[test]
    #[should_panic(expected = "empty slice")]
    fn average_of_empty_slice_panics() {
        average(&[]);
    }
}
// ANCHOR_END: unit_test

// ANCHOR: property_test
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// For any non-empty slice, the average is never less than its
        /// minimum element or greater than its maximum — an invariant
        /// that holds for every possible input, not just the handful of
        /// examples a human happens to pick.
        #[test]
        fn average_is_between_min_and_max(
            values in prop::collection::vec(-1000i32..1000, 1..50)
        ) {
            let avg = average(&values);
            let min = f64::from(*values.iter().min().unwrap());
            let max = f64::from(*values.iter().max().unwrap());
            prop_assert!(avg >= min && avg <= max);
        }
    }
}
// ANCHOR_END: property_test
