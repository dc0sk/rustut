// SPDX-License-Identifier: MIT OR Apache-2.0
//! Copy of exercises/ch03-crate-anatomy/ex01-module-visibility/src/inventory.rs.

fn format_sku(id: u32) -> String {
    format!("SKU-{id:06}")
}

pub struct Item {
    id: u32,
    pub name: String,
}

impl Item {
    pub fn new(id: u32, name: impl Into<String>) -> Self {
        Item {
            id,
            name: name.into(),
        }
    }

    pub fn describe(&self) -> String {
        format!("{} ({})", self.name, format_sku(self.id))
    }
}
