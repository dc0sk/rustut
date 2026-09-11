// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch06-borrowing-and-references/ex01-largest-without-cloning.

pub fn largest(nums: &[i32]) -> Option<&i32> {
    nums.iter().max()
}

pub fn grow_then_summarize(data: &mut Vec<i32>, extra: i32) -> String {
    data.push(extra);
    format!("{} numbers, sum {}", data.len(), data.iter().sum::<i32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        assert_eq!(largest(&[3, 7, 2, 9, 4]), Some(&9));
        let empty: [i32; 0] = [];
        assert_eq!(largest(&empty), None);
        let mut data = vec![1, 2, 3];
        assert_eq!(grow_then_summarize(&mut data, 1), "4 numbers, sum 7");
    }
}
