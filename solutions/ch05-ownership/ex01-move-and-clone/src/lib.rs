// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch05-ownership/ex01-move-and-clone.

pub fn append_owned(mut data: String, suffix: String) -> String {
    data.push_str(&suffix);
    data
}

pub fn double_in_place(mut data: Vec<i32>) -> Vec<i32> {
    for value in &mut data {
        *value *= 2;
    }
    data
}

pub fn duplicate(original: &str) -> String {
    original.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        assert_eq!(
            append_owned(String::from("hello"), String::from(" world")),
            "hello world"
        );
        assert_eq!(double_in_place(vec![1, 2, 3]), vec![2, 4, 6]);

        let original = String::from("abc");
        let copy = duplicate(&original);
        assert_eq!(copy, "abc");
        assert_eq!(original, "abc");
    }
}
