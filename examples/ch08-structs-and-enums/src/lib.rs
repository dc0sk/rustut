// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: struct_kinds
/// A named-field struct — the everyday case, one field per name.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// A tuple struct — fields identified by position (`.0`), not name. In C,
/// `typedef double Meters;` gives you a readable name but no real type
/// distinction: a `Meters` and a raw `double` are freely interchangeable
/// to the compiler. `Meters(f64)` and `f64` are not interchangeable to
/// Rust's — passing a bare `f64` where `Meters` is expected is a type
/// error, caught at compile time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meters(pub f64);

/// A unit struct — no fields, zero runtime size. Useful purely as a
/// distinct marker type; C has no equivalent since it has no zero-sized
/// types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Origin;
// ANCHOR_END: struct_kinds

// ANCHOR: impl_block
impl Point {
    /// An *associated function* — called as `Point::new(...)`, not on an
    /// existing value. The closest C analogue is a `point_create(...)`
    /// constructor function, except namespaced under the type itself
    /// rather than sharing the global symbol table.
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    /// A *method* — takes `&self`, called as `point.distance_from_origin()`.
    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
// ANCHOR_END: impl_block

// ANCHOR: shape_enum
/// A tagged union, done right. In C you would hand-roll:
///
/// ```c
/// enum ShapeTag { CIRCLE, RECTANGLE, TRIANGLE };
/// struct Shape {
///     enum ShapeTag tag;
///     union {
///         double radius;
///         struct { double width, height; } rectangle;
///         struct { double base, height; } triangle;
///     } payload;
/// };
/// ```
///
/// Nothing in that C code stops you from reading `payload.rectangle`
/// while `tag == CIRCLE` — that's undefined behavior, not a caught bug,
/// and it is a real, recurring class of incident in C codebases. Rust's
/// `enum` makes it structurally impossible: each variant carries exactly
/// its own payload type, and the only way to read one back out is a
/// `match` the compiler forces to handle every variant (Chapter 9).
///
/// One more difference worth flagging early: this `enum`'s in-memory
/// layout (`repr(Rust)`, the default) is deliberately *unspecified* — the
/// compiler is free to reorder fields and pick a tag representation to
/// minimize size, unlike a C `struct`, whose layout the language
/// guarantees. When you need C's layout guarantees — talking to a C
/// library over FFI — `#[repr(C)]` exists for exactly that; Chapter 26
/// covers it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}
// ANCHOR_END: shape_enum

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_distance() {
        let p = Point::new(3.0, 4.0);
        assert_eq!(p.distance_from_origin(), 5.0);
    }

    #[test]
    fn meters_is_not_a_bare_f64() {
        let m = Meters(2.5);
        assert_eq!(m.0, 2.5);
    }

    #[test]
    fn origin_is_zero_sized_and_equatable() {
        assert_eq!(Origin, Origin);
        assert_eq!(std::mem::size_of::<Origin>(), 0);
    }

    #[test]
    fn shape_areas() {
        assert!((Shape::Circle { radius: 2.0 }.area() - std::f64::consts::PI * 4.0).abs() < 1e-9);
        assert_eq!(
            Shape::Rectangle {
                width: 3.0,
                height: 4.0
            }
            .area(),
            12.0
        );
        assert_eq!(
            Shape::Triangle {
                base: 6.0,
                height: 2.0
            }
            .area(),
            6.0
        );
    }
}
