// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch15-closures-and-iterators/ex01-sum-where.

pub fn sum_where(data: &[i32], keep: impl Fn(i32) -> bool, f: impl Fn(i32) -> i64) -> i64 {
    data.iter().copied().filter(|&x| keep(x)).map(f).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        let data = [1, 2, 3, 4, 5, 6];
        assert_eq!(
            sum_where(&data, |x| x % 2 == 0, |x| i64::from(x) * i64::from(x)),
            4 + 16 + 36
        );
        assert_eq!(sum_where(&[], |_| true, i64::from), 0);
    }
}
