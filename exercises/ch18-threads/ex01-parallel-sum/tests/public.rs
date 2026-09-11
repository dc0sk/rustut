// SPDX-License-Identifier: MIT OR Apache-2.0
use ch18_ex01_parallel_sum::parallel_sum;

#[test]
fn single_worker_matches_sequential_sum() {
    let data: Vec<i64> = (1..=100).collect();
    assert_eq!(parallel_sum(&data, 1), data.iter().sum());
}

#[test]
fn multiple_workers_match_sequential_sum() {
    let data: Vec<i64> = (1..=997).collect();
    assert_eq!(parallel_sum(&data, 4), data.iter().sum());
}

#[test]
fn more_workers_than_elements_still_works() {
    let data = vec![1_i64, 2, 3];
    assert_eq!(parallel_sum(&data, 8), 6);
}

#[test]
fn empty_slice_sums_to_zero() {
    let data: Vec<i64> = vec![];
    assert_eq!(parallel_sum(&data, 4), 0);
}
