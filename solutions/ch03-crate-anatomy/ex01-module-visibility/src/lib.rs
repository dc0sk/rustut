// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch03-crate-anatomy/ex01-module-visibility.

mod inventory;

pub use inventory::Item;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        let item = Item::new(42, "Widget");
        assert_eq!(item.describe(), "Widget (SKU-000042)");
    }
}
