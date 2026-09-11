// SPDX-License-Identifier: MIT OR Apache-2.0
use ch03_ex01_module_visibility::Item;

#[test]
fn describe_formats_the_sku() {
    let item = Item::new(42, "Widget");
    assert_eq!(item.describe(), "Widget (SKU-000042)");
}

#[test]
fn name_is_directly_readable() {
    let item = Item::new(1, "Gadget");
    assert_eq!(item.name, "Gadget");
}
