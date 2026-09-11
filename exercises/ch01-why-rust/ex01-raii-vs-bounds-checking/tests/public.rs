// SPDX-License-Identifier: MIT OR Apache-2.0
use ch01_ex01_raii_vs_bounds_checking::{safe_get, use_resource};
use std::cell::Cell;

#[test]
fn in_bounds_index_returns_the_value() {
    let data = [10, 20, 30];
    assert_eq!(safe_get(&data, 0), Some(10));
    assert_eq!(safe_get(&data, 2), Some(30));
}

#[test]
fn out_of_bounds_index_returns_none_not_a_panic() {
    let data = [10, 20, 30];
    assert_eq!(safe_get(&data, 3), None);
    assert_eq!(safe_get(&data, usize::MAX), None);
}

#[test]
fn empty_slice_is_always_out_of_bounds() {
    let data: [i32; 0] = [];
    assert_eq!(safe_get(&data, 0), None);
}

#[test]
fn resource_is_released_on_normal_return() {
    let releases = Cell::new(0);
    assert_eq!(use_resource(&releases, false), "normal");
    assert_eq!(releases.get(), 1);
}

#[test]
fn resource_is_released_on_early_return_too() {
    let releases = Cell::new(0);
    assert_eq!(use_resource(&releases, true), "early");
    assert_eq!(
        releases.get(),
        1,
        "Drop must run even when the function returns early"
    );
}

#[test]
fn each_call_releases_independently() {
    let releases = Cell::new(0);
    use_resource(&releases, true);
    use_resource(&releases, false);
    use_resource(&releases, true);
    assert_eq!(releases.get(), 3);
}
