use super::*;

#[test]
fn context_menu_parent_submenu_debug_is_opaque() {
    let projection = SanitizedContextMenuProjectionBuilder::new()
        .item(
            SanitizedContextMenuItem::new(
                SanitizedContextMenuTarget::from_opaque_bytes(b"parent-secret"),
                0,
                "親 日本語 ⭐️",
            )
            .submenu_item(SanitizedContextMenuItem::new(
                SanitizedContextMenuTarget::from_opaque_bytes(b"child-secret"),
                0,
                "子 日本語 ⭐️",
            )),
        )
        .build();
    let debug = format!("{projection:?}");
    assert!(!debug.contains("親 日本語"));
    assert!(!debug.contains("parent-secret"));
}
