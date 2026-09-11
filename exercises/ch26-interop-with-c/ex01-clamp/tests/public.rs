// SPDX-License-Identifier: MIT OR Apache-2.0
use ch26_ex01_clamp::round_half_away_from_zero;

#[test]
fn rounds_positive_half_up() {
    assert_eq!(round_half_away_from_zero(2.5), 3);
}

#[test]
fn rounds_negative_half_down() {
    assert_eq!(round_half_away_from_zero(-2.5), -3);
}

#[test]
fn rounds_ordinary_values() {
    assert_eq!(round_half_away_from_zero(2.4), 2);
    assert_eq!(round_half_away_from_zero(0.0), 0);
}
