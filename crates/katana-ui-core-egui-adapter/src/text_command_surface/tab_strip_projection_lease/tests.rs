use super::*;
use katana_ui_core::molecule::RgbaColor;

#[test]
fn nested_descriptors_and_opaque_choices_are_constructible() {
    let child = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"child-group"),
        TabStripText::new("子グループ"),
    )
    .tab(
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"child-tab"),
            TabStripText::new("⭐️"),
        )
        .capabilities(TabStripTabCapabilities::new().selectable(true)),
    );
    let projection =
        TabStripProjection::new(7, TabStripCorrelation::from_opaque_bytes(b"correlation")).group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"root-group"),
                TabStripText::new("ルート"),
            )
            .group(child),
        );
    let lease = TabStripProjectionLease::new(projection);
    let debug = format!("{lease:?}");
    assert_eq!(debug, "TabStripProjectionLease(..)");
    let _swatch = TabStripSwatchTarget::from_opaque_bytes(b"swatch-token");
}

#[test]
fn opaque_debug_does_not_reveal_payload_or_presentation() {
    let projection = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"secret-correlation"),
    )
    .tab(TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"secret-target"),
        TabStripText::new("秘密のタブ"),
    ));
    let debug = format!("{projection:?}");
    for forbidden in ["secret-correlation", "secret-target", "秘密のタブ"] {
        assert!(!debug.contains(forbidden), "debug leaked {forbidden}");
    }
}

#[test]
fn overlay_projection_keeps_menu_popup_and_swatch_routes_non_wire() {
    let swatch = TabStripSwatchDescriptor::new(
        TabStripSwatchTarget::from_opaque_bytes(b"secret-swatch"),
        RgbaColor::new(74, 144, 217, 255),
    )
    .selected(true)
    .accessibility_label(TabStripText::new("青 ⭐️"));
    let menu = TabStripContextMenuPresentation::new().entry(
        TabStripMenuEntry::submenu(
            TabStripText::new("グループ"),
            TabStripText::new("グループ操作"),
        )
        .child(TabStripMenuEntry::action(
            TabStripText::new("追加"),
            TabStripText::new("グループへ追加"),
            TabStripMenuOperation::MoveToGroup(TabStripGroupTarget::from_opaque_bytes(
                b"secret-group",
            )),
        )),
    );
    let popup = TabStripGroupPopupPresentation::new()
        .rename_placeholder(TabStripText::new("グループ名"))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("解除"),
            TabStripText::new("グループを解除"),
            TabStripMenuOperation::Ungroup,
        ));
    let projection = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"secret-correlation"),
    )
    .tab(
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"secret-tab"),
            TabStripText::new("秘密のタブ"),
        )
        .context_menu(menu),
    )
    .group(
        TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"secret-popup-group"),
            TabStripText::new("秘密のグループ"),
        )
        .swatch(swatch)
        .popup(popup),
    );

    let debug = format!("{projection:?}");
    for forbidden in [
        "secret-correlation",
        "secret-tab",
        "secret-group",
        "secret-swatch",
        "秘密のタブ",
        "秘密のグループ",
        "グループ操作",
        "青 ⭐️",
    ] {
        assert!(!debug.contains(forbidden), "debug leaked {forbidden}");
    }
}

#[test]
fn production_source_has_no_wire_or_legacy_boundary_symbols() {
    let source = include_str!("../tab_strip_projection_lease.rs");
    let production = source
        .split("#[cfg(test)]")
        .next()
        .expect("test module marker");
    for forbidden in [
        "Workspace",
        "CloseableTab",
        "SanitizedTab",
        "Serialize",
        "Deserialize",
        "derive(Clone)",
        "payload(",
        "fn target",
        "fn label",
        "fn order",
        "PathBuf",
    ] {
        assert!(
            !production.contains(forbidden),
            "legacy/wire symbol: {forbidden}"
        );
    }
}
