// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

pub trait HasArea {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

/// Sum the area of a heterogeneous collection of shapes stored behind
/// `Box<dyn HasArea>`. The exercise is dispatching through the trait
/// object — don't match on concrete types or downcast; treat every
/// element purely through the `HasArea` interface.
pub fn total_area(shapes: &[Box<dyn HasArea>]) -> f64 {
    todo!("sum shapes[i].area() for every shape, via the HasArea trait")
}
