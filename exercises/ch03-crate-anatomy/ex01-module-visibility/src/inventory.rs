// SPDX-License-Identifier: MIT OR Apache-2.0
//! Complete — nothing to fix in this file. The exercise is in `lib.rs`.

/// Kept private: nothing outside this module should format a SKU directly.
/// Compare to a C helper declared `static` in a .c file, visible only
/// within that translation unit — except this is enforced across the
/// whole module, and by the compiler, not by a naming convention.
fn format_sku(id: u32) -> String {
    format!("SKU-{id:06}")
}

pub struct Item {
    id: u32,
    pub name: String,
}

impl Item {
    pub fn new(id: u32, name: impl Into<String>) -> Self {
        Item { id, name: name.into() }
    }

    /// The only way to learn an item's formatted SKU from outside this
    /// module — `format_sku` itself stays private.
    pub fn describe(&self) -> String {
        format!("{} ({})", self.name, format_sku(self.id))
    }
}
