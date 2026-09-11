// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

/// Task 1: a plain struct with an `impl` block.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        todo!("construct a Rectangle from width/height")
    }

    pub fn area(&self) -> f64 {
        todo!("width * height")
    }

    pub fn perimeter(&self) -> f64 {
        todo!("2 * (width + height)")
    }
}

/// Task 2: an enum whose variants carry different data — a tagged union
/// the compiler forces you to handle exhaustively.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Vehicle {
    Car { doors: u8 },
    Truck { axles: u8 },
    Motorcycle,
}

impl Vehicle {
    /// A `Car` has 4 wheels regardless of door count. A `Truck` has 2
    /// wheels per axle. A `Motorcycle` has 2 wheels.
    pub fn wheel_count(&self) -> u32 {
        todo!("match on `self` and return the wheel count for each variant")
    }
}
