// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: trait_def
/// A contract every "describable" type implements — the Rust analogue of
/// a hand-rolled C vtable struct, except the compiler resolves a call to
/// `describe` at compile time by default (static dispatch), not through
/// an indirect function-pointer call.
pub trait Describe {
    fn describe(&self) -> String;

    /// A default method body: every implementor gets this for free unless
    /// it overrides it — there's no equivalent in a C vtable, where every
    /// function pointer slot must be filled in by hand.
    fn describe_loudly(&self) -> String {
        format!("{}!!!", self.describe().to_uppercase())
    }
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
// ANCHOR_END: trait_def

// ANCHOR: static_dispatch
/// A generic function bounded by a trait: at compile time the compiler
/// generates one specialized copy of `print_description` per concrete
/// type it's actually called with ("monomorphization") — no indirection
/// at runtime, as if you'd hand-written a version per type, but without
/// duplicating the source.
pub fn print_description<T: Describe>(item: &T) -> String {
    item.describe()
}
// ANCHOR_END: static_dispatch

// ANCHOR: generic_largest
/// The Rust answer to C's "write this once, for every type" problem —
/// usually solved in C with `void*` plus manual casting, a macro, or
/// duplicating the function per type. `T: PartialOrd + Copy` is a
/// compile-time-checked constraint: this simply won't compile for a `T`
/// that can't be ordered and copied, unlike a `void*` version, which
/// would happily compile and then crash or misbehave on the wrong type.
pub fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut max = items[0];
    for &item in &items[1..] {
        if item > max {
            max = item;
        }
    }
    max
}
// ANCHOR_END: generic_largest

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_dispatch_calls_the_right_impl() {
        let c = Circle { radius: 2.0 };
        let s = Square { side: 3.0 };
        assert_eq!(print_description(&c), "circle, radius 2.0");
        assert_eq!(print_description(&s), "square, side 3.0");
    }

    #[test]
    fn default_method_uses_the_override() {
        let c = Circle { radius: 1.0 };
        assert_eq!(c.describe_loudly(), "CIRCLE, RADIUS 1.0!!!");
    }

    #[test]
    fn largest_works_for_integers_and_floats() {
        assert_eq!(largest(&[3, 7, 2, 9, 4]), 9);
        assert_eq!(largest(&[1.5, 2.5, 0.5]), 2.5);
    }
}
