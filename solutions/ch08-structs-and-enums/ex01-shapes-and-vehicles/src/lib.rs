// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch08-structs-and-enums/ex01-shapes-and-vehicles.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Vehicle {
    Car { doors: u8 },
    Truck { axles: u8 },
    Motorcycle,
}

impl Vehicle {
    pub fn wheel_count(&self) -> u32 {
        match self {
            Vehicle::Car { .. } => 4,
            Vehicle::Truck { axles } => u32::from(*axles) * 2,
            Vehicle::Motorcycle => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        let r = Rectangle::new(3.0, 4.0);
        assert_eq!(r.area(), 12.0);
        assert_eq!(r.perimeter(), 14.0);
        assert_eq!(Vehicle::Car { doors: 2 }.wheel_count(), 4);
        assert_eq!(Vehicle::Truck { axles: 3 }.wheel_count(), 6);
        assert_eq!(Vehicle::Motorcycle.wheel_count(), 2);
    }
}
