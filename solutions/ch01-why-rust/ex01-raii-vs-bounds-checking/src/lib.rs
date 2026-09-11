// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch01-why-rust/ex01-raii-vs-bounds-checking.
//! Try the exercise yourself before reading this.

use std::cell::Cell;

pub fn safe_get(data: &[i32], index: usize) -> Option<i32> {
    data.get(index).copied()
}

pub struct Resource<'a> {
    releases: &'a Cell<u32>,
}

impl<'a> Resource<'a> {
    pub fn new(releases: &'a Cell<u32>) -> Self {
        Resource { releases }
    }
}

impl Drop for Resource<'_> {
    fn drop(&mut self) {
        self.releases.set(self.releases.get() + 1);
    }
}

pub fn use_resource(releases: &Cell<u32>, take_early_exit: bool) -> &'static str {
    let _guard = Resource::new(releases);
    if take_early_exit {
        return "early";
    }
    "normal"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        let data = [10, 20, 30];
        assert_eq!(safe_get(&data, 1), Some(20));
        assert_eq!(safe_get(&data, 3), None);

        let releases = Cell::new(0);
        use_resource(&releases, true);
        use_resource(&releases, false);
        assert_eq!(releases.get(), 2);
    }
}
