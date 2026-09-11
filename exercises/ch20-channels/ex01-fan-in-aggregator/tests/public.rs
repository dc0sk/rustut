// SPDX-License-Identifier: MIT OR Apache-2.0
use ch20_ex01_fan_in_aggregator::square_all_concurrently;

#[test]
fn squares_and_sorts() {
    assert_eq!(square_all_concurrently(vec![3, 1, 2]), vec![1, 4, 9]);
}

#[test]
fn handles_negative_numbers() {
    assert_eq!(square_all_concurrently(vec![-2, 2]), vec![4, 4]);
}

#[test]
fn empty_input_gives_empty_output() {
    assert_eq!(square_all_concurrently(vec![]), Vec::<i32>::new());
}

#[test]
fn larger_batch() {
    let input: Vec<i32> = (1..=20).collect();
    let expected: Vec<i32> = input.iter().map(|n| n * n).collect();
    assert_eq!(square_all_concurrently(input), expected);
}
