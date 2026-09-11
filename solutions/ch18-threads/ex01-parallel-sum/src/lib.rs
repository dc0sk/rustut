// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch18-threads/ex01-parallel-sum.

use std::thread;

pub fn parallel_sum(data: &[i64], num_workers: usize) -> i64 {
    if data.is_empty() || num_workers == 0 {
        return data.iter().sum();
    }
    let chunk_size = data.len().div_ceil(num_workers).max(1);

    thread::scope(|s| {
        data.chunks(chunk_size)
            .map(|chunk| s.spawn(move || chunk.iter().sum::<i64>()))
            .collect::<Vec<_>>()
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .sum()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        let data: Vec<i64> = (1..=997).collect();
        assert_eq!(parallel_sum(&data, 4), data.iter().sum());
        assert_eq!(parallel_sum(&[], 4), 0);
        assert_eq!(parallel_sum(&[1, 2, 3], 8), 6);
    }
}
