// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch13-trait-objects/ex01-shape-collection.

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

pub fn total_area(shapes: &[Box<dyn HasArea>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        let shapes: Vec<Box<dyn HasArea>> = vec![
            Box::new(Circle { radius: 1.0 }),
            Box::new(Rectangle {
                width: 2.0,
                height: 3.0,
            }),
        ];
        let total = total_area(&shapes);
        assert!((total - (std::f64::consts::PI + 6.0)).abs() < 1e-9);
    }
}
