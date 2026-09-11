// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

pub mod logging;

// TODO: add a `prelude` module here that re-exports `logging::log_line`
// at `crate::prelude::log_line`, so callers can write
// `use ch14_ex01_visibility_boundaries::prelude::*;` instead of reaching
// into `logging` directly.
