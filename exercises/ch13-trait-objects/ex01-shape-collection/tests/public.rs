// SPDX-License-Identifier: MIT OR Apache-2.0
use ch13_ex01_shape_collection::{Circle, HasArea, Rectangle, total_area};

#[test]
fn sums_area_of_mixed_shapes() {
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

#[test]
fn empty_collection_has_zero_area() {
    let shapes: Vec<Box<dyn HasArea>> = vec![];
    assert_eq!(total_area(&shapes), 0.0);
}

#[test]
fn single_shape_matches_its_own_area() {
    let shapes: Vec<Box<dyn HasArea>> = vec![Box::new(Rectangle {
        width: 4.0,
        height: 5.0,
    })];
    assert_eq!(total_area(&shapes), 20.0);
}
