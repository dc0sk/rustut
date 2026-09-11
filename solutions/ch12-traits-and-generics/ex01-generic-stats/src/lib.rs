// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch12-traits-and-generics/ex01-generic-stats.

pub trait Scored {
    fn score(&self) -> i64;
}

pub fn highest_scored<T: Scored>(items: &[T]) -> &T {
    items
        .iter()
        .max_by_key(|item| item.score())
        .expect("items must not be empty")
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Bid(i64);

    impl Scored for Bid {
        fn score(&self) -> i64 {
            self.0
        }
    }

    #[test]
    fn matches_public_test_expectations() {
        let bids = vec![Bid(10), Bid(50), Bid(30)];
        assert_eq!(highest_scored(&bids).0, 50);
    }
}
