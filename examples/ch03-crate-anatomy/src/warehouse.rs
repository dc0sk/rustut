// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: visibility
/// Visible anywhere the crate is used from (`pub`) — like a function
/// declared in a public header.
pub struct Crate {
    pub label: String,
    // No `pub`: private to this module by default, like a `static` file
    // scope variable in a C translation unit — except enforced across the
    // whole module tree, not just one file, and checked by the compiler
    // everywhere, not just within the one .c file that can see it.
    weight_kg: f64,
}

impl Crate {
    pub fn new(label: impl Into<String>, weight_kg: f64) -> Self {
        Crate {
            label: label.into(),
            weight_kg,
        }
    }

    pub fn weight_kg(&self) -> f64 {
        self.weight_kg
    }

    pub fn shipping_label(&self) -> String {
        shipping_label(self)
    }
}

/// `pub(crate)`: visible anywhere *in this crate*, but not to callers
/// outside it — narrower than `pub`, wider than private. There's no C
/// equivalent with this exact granularity; the closest analogue is a
/// symbol given external linkage but only ever declared in an internal,
/// unshipped header.
pub(crate) fn shipping_label(c: &Crate) -> String {
    format!("{} ({:.1} kg)", c.label, c.weight_kg)
}
// ANCHOR_END: visibility

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shipping_label_formats_weight() {
        let c = Crate::new("Widgets", 12.5);
        assert_eq!(shipping_label(&c), "Widgets (12.5 kg)");
    }
}
