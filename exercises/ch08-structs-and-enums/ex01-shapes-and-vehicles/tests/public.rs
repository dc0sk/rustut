// SPDX-License-Identifier: MIT OR Apache-2.0
use ch08_ex01_shapes_and_vehicles::{Rectangle, Vehicle};

#[test]
fn rectangle_area_and_perimeter() {
    let r = Rectangle::new(3.0, 4.0);
    assert_eq!(r.area(), 12.0);
    assert_eq!(r.perimeter(), 14.0);
}

#[test]
fn rectangle_derives_debug_and_eq() {
    let a = Rectangle::new(2.0, 5.0);
    let b = Rectangle::new(2.0, 5.0);
    assert_eq!(a, b);
    assert_eq!(format!("{a:?}"), "Rectangle { width: 2.0, height: 5.0 }");
}

#[test]
fn car_wheel_count_ignores_door_count() {
    assert_eq!(Vehicle::Car { doors: 2 }.wheel_count(), 4);
    assert_eq!(Vehicle::Car { doors: 4 }.wheel_count(), 4);
}

#[test]
fn truck_wheel_count_scales_with_axles() {
    assert_eq!(Vehicle::Truck { axles: 2 }.wheel_count(), 4);
    assert_eq!(Vehicle::Truck { axles: 3 }.wheel_count(), 6);
}

#[test]
fn motorcycle_has_two_wheels() {
    assert_eq!(Vehicle::Motorcycle.wheel_count(), 2);
}
