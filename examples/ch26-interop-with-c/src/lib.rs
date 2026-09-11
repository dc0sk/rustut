// SPDX-License-Identifier: MIT OR Apache-2.0
//! Guided example for Chapter 26 — wrapping a small, real C library
//! compiled by this crate's own `build.rs` (see `build.rs` and
//! `csrc/geometry.c`).

// ANCHOR: repr_c_point
/// `#[repr(C)]` pins this struct's layout to match what `geometry.c`'s
/// `point_t` actually has in memory — plain `#[derive(...)]` Rust structs
/// have an *unspecified* layout the compiler is free to reorder/pack for
/// its own reasons (Ch. 8), which is fine for Rust-only code but would be
/// silently wrong across an FFI boundary.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
// ANCHOR_END: repr_c_point

// ANCHOR: extern_block
// A hand-written binding, not `bindgen`-generated: `bindgen` is the right
// tool for wrapping an existing, larger C library from its header, but it
// needs `libclang` discoverable at build time, which this repository
// deliberately avoids depending on. For a small C file you're authoring
// yourself, writing the `extern` block by hand is simpler and has no such
// external requirement.
unsafe extern "C" {
    safe fn point_distance(a: Point, b: Point) -> f32;
    safe fn clamp_int(value: i32, min: i32, max: i32) -> i32;
}
// ANCHOR_END: extern_block

pub fn distance(a: Point, b: Point) -> f32 {
    point_distance(a, b)
}

pub fn clamp(value: i32, min: i32, max: i32) -> i32 {
    clamp_int(value, min, max)
}

// ANCHOR: reverse_direction
// The other direction: exposing a Rust function so *C* code can call it.
// `#[unsafe(no_mangle)]` (edition 2024's syntax for this unsafe attribute)
// stops the compiler from mangling the symbol name, `extern "C"` picks the
// C calling convention, and a `crate-type = ["cdylib"]` in Cargo.toml (not
// added here, to keep this crate an ordinary library the rest of the book
// can depend on/test) produces a `.so`/`.dll` a C program can `dlopen` or
// link directly, no different from linking against any other C library.
#[unsafe(no_mangle)]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}
// ANCHOR_END: reverse_direction

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_is_pythagorean() {
        let a = Point { x: 0.0, y: 0.0 };
        let b = Point { x: 3.0, y: 4.0 };
        assert!((distance(a, b) - 5.0).abs() < 1e-5);
    }

    #[test]
    fn clamp_bounds_the_value() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-5, 0, 10), 0);
        assert_eq!(clamp(50, 0, 10), 10);
    }

    #[test]
    fn add_one_is_callable_as_a_plain_rust_fn_too() {
        assert_eq!(add_one(41), 42);
    }
}
