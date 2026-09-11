// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: trait_object
/// The same `Describe`-shaped idea as Chapter 12, but every shape here is
/// stored behind `Box<dyn Describe>` — Rust's actual vtable-based dynamic
/// dispatch. At runtime, that `Box` genuinely is a (data pointer, vtable
/// pointer) pair — the same shape as a hand-rolled C
/// `struct { void *data; const struct describe_vtable *vtable; }`.
pub trait Describe {
    fn describe(&self) -> String;
}

pub struct Circle {
    pub radius: f64,
}

pub struct Square {
    pub side: f64,
}

impl Describe for Circle {
    fn describe(&self) -> String {
        format!("circle, radius {:.1}", self.radius)
    }
}

impl Describe for Square {
    fn describe(&self) -> String {
        format!("square, side {:.1}", self.side)
    }
}

/// A `Vec<T>` can only ever hold one concrete type — `Vec<Circle>` cannot
/// also hold a `Square`. Boxing each item as `dyn Describe` erases the
/// concrete type behind a shared interface, the way a C array of
/// `struct shape_vtable *` pointers would, at the cost of one indirect
/// call per `.describe()` instead of Chapter 12's zero-cost static call.
pub fn describe_all(shapes: &[Box<dyn Describe>]) -> Vec<String> {
    shapes.iter().map(|s| s.describe()).collect()
}
// ANCHOR_END: trait_object

// ANCHOR: object_safety
/// Not every trait can become `dyn Trait`. A trait with a generic method
/// isn't **dyn compatible** (the older term you'll still see everywhere
/// is "object safe" — same concept): the vtable would need one
/// function-pointer slot per concrete type that method could ever be
/// called with, which is unbounded and unknowable when the trait is
/// defined — so the compiler refuses outright:
///
/// ```compile_fail
/// trait NotObjectSafe {
///     fn generic_method<T>(&self, value: T);
/// }
///
/// fn takes_dyn(x: &dyn NotObjectSafe) {
///     let _ = x;
/// }
/// ```
///
/// Captured verbatim from a real `rustc` run on this repo's toolchain:
///
/// ```text
/// error[E0038]: the trait `NotObjectSafe` is not dyn compatible
///  --> src/lib.rs:5:18
///   |
/// 5 | fn takes_dyn(x: &dyn NotObjectSafe) {
///   |                  ^^^^^^^^^^^^^^^^^ `NotObjectSafe` is not dyn compatible
///   |
/// note: for a trait to be dyn compatible it needs to allow building a vtable
///       for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
///  --> src/lib.rs:2:8
///   |
/// 1 | trait NotObjectSafe {
///   |       ------------- this trait is not dyn compatible...
/// 2 |     fn generic_method<T>(&self, value: T);
///   |        ^^^^^^^^^^^^^^ ...because method `generic_method` has generic type parameters
/// ```
///
/// (This function's only purpose is to hold the doctest above — doctests
/// on private items aren't run by `cargo test`, so it must be `pub`.)
pub fn object_safety_notes() {}
// ANCHOR_END: object_safety

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_all_handles_mixed_shapes() {
        let shapes: Vec<Box<dyn Describe>> = vec![
            Box::new(Circle { radius: 1.0 }),
            Box::new(Square { side: 2.0 }),
        ];
        assert_eq!(
            describe_all(&shapes),
            vec!["circle, radius 1.0", "square, side 2.0"]
        );
    }
}
