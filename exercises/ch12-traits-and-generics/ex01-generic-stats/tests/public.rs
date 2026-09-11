// SPDX-License-Identifier: MIT OR Apache-2.0
use ch12_ex01_generic_stats::{Scored, highest_scored};

struct Player {
    name: &'static str,
    points: i64,
}

impl Scored for Player {
    fn score(&self) -> i64 {
        self.points
    }
}

struct Bid(i64);

impl Scored for Bid {
    fn score(&self) -> i64 {
        self.0
    }
}

#[test]
fn finds_the_highest_scored_player() {
    let players = vec![
        Player {
            name: "Ada",
            points: 42,
        },
        Player {
            name: "Grace",
            points: 99,
        },
        Player {
            name: "Linus",
            points: 7,
        },
    ];
    assert_eq!(highest_scored(&players).name, "Grace");
}

#[test]
fn works_with_a_different_scored_type() {
    let bids = vec![Bid(10), Bid(50), Bid(30)];
    assert_eq!(highest_scored(&bids).0, 50);
}

#[test]
fn single_item_is_trivially_the_highest() {
    let players = vec![Player {
        name: "Solo",
        points: 1,
    }];
    assert_eq!(highest_scored(&players).name, "Solo");
}
