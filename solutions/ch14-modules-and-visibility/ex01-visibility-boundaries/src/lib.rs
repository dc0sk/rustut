// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch14-modules-and-visibility/ex01-visibility-boundaries.

pub mod logging;

pub mod prelude {
    pub use crate::logging::log_line;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        assert_eq!(logging::log_line(3661, "x"), "[01:01:01] x");
        assert_eq!(prelude::log_line(59, "y"), "[00:00:59] y");
    }
}
