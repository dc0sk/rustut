// SPDX-License-Identifier: MIT OR Apache-2.0
// ANCHOR: build_rs
fn main() {
    cc::Build::new().file("csrc/geometry.c").compile("geometry");
}
// ANCHOR_END: build_rs
