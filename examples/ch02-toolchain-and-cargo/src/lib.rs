// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: doc_comment
/// Converts Celsius to Fahrenheit.
///
/// `cargo doc --open` renders this comment as an HTML page, the same job
/// Doxygen's `/** ... */` blocks do for a `.h` file — except it's driven
/// straight off the source, with no separate Doxyfile to configure, and
/// runnable examples in the doc comment are executed by `cargo test`:
///
/// ```
/// assert_eq!(ch02_toolchain_and_cargo_example::celsius_to_fahrenheit(0.0), 32.0);
/// ```
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
// ANCHOR_END: doc_comment
