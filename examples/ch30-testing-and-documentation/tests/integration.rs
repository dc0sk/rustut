// SPDX-License-Identifier: MIT OR Apache-2.0
// ANCHOR: integration_test
// An integration test: this file compiles as a wholly separate crate that
// can only see `average` through the library's public API — exactly the
// relationship every exercise's `tests/public.rs` has with its exercise
// crate throughout this book, formalized here as its own testing category.
use ch30_testing_and_documentation_example::average;

#[test]
fn integration_test_sees_only_the_public_api() {
    assert_eq!(average(&[10, 20, 30]), 20.0);
}
// ANCHOR_END: integration_test
