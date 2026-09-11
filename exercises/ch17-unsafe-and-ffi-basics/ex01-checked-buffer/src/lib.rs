// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

/// Returns `Some(data[index])` if `index` is in bounds, `None`
/// otherwise — but must read the element via `data.get_unchecked(index)`
/// internally (not `data[index]`, not `data.get(index)`), inside an
/// `unsafe` block carrying a `// SAFETY:` comment that explains why the
/// access is sound at that point.
pub fn checked_get_unchecked(data: &[i32], index: usize) -> Option<i32> {
    todo!(
        "bounds-check `index` against `data.len()` yourself first, then \
         read via an `unsafe` block calling `data.get_unchecked(index)`"
    )
}
