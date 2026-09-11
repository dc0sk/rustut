// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch10-option-result/ex01-lookup-and-parse.

pub fn first_positive(values: &[i32]) -> Option<i32> {
    values.iter().find(|&&v| v > 0).copied()
}

pub fn parse_pair(input: &str) -> Result<(i64, i64), String> {
    let (left, right) = input
        .split_once(',')
        .ok_or_else(|| format!("missing ',' in {input:?}"))?;
    let left: i64 = left
        .trim()
        .parse()
        .map_err(|e| format!("bad left value: {e}"))?;
    let right: i64 = right
        .trim()
        .parse()
        .map_err(|e| format!("bad right value: {e}"))?;
    Ok((left, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        assert_eq!(first_positive(&[-3, -1, 0, 2, 5]), Some(2));
        assert_eq!(first_positive(&[-3, -1, 0]), None);
        assert_eq!(parse_pair("3,4"), Ok((3, 4)));
        assert!(parse_pair("34").is_err());
        assert!(parse_pair("3,x").is_err());
    }
}
