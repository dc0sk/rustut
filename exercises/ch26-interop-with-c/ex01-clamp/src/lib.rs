// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.
//!
//! `csrc/stats.c` is complete and already compiled by `build.rs` — the
//! exercise is writing the Rust-side binding and wrapper.

// TODO: declare the `extern "C"` block for `round_to_nearest` (it takes a
// `double`/`f64` and returns an `int`/`i32`). Remember edition 2024's
// syntax: the block itself is `unsafe extern "C" { ... }`, and you can
// mark an item `safe fn` if you're vouching it's safe to call with any
// input (rounding a `f64` can't do anything unsound).

pub fn round_half_away_from_zero(value: f64) -> i32 {
    todo!("call the C function you declared above")
}
