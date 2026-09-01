use super::{
    SanitizedContextMenuItem, SanitizedContextMenuProjectionBuilder, SanitizedContextMenuTarget,
};
use crate::render_model::{UiIconProps, UiSvgPaintPolicy};

#[test]
fn consuming_builder_constructs_a_localized_checked_submenu_tree() {
    let child = SanitizedContextMenuItem::new(
        SanitizedContextMenuTarget::from_opaque_bytes([2, 3]),
        2,
        "子項目",
    )
    .accessibility_label_text("子項目を実行")
    .enabled_state(false);
    let item = SanitizedContextMenuItem::new(
        SanitizedContextMenuTarget::from_opaque_bytes([1]),
        1,
        "表示",
    )
    .accessibility_label_text("表示設定")
    .with_icon(UiIconProps::new("<svg/>"))
    .checked_state(true)
    .submenu_item(child);
    let projection = SanitizedContextMenuProjectionBuilder::new()
        .item(item)
        .build();

    let item = &projection.items()[0];
    assert_eq!(item.order(), 1);
    assert_eq!(item.label(), "表示");
    assert_eq!(item.accessibility_label(), Some("表示設定"));
    assert!(item.icon().is_some());
    assert!(item.enabled());
    assert!(item.checked());
    assert_eq!(item.submenu().len(), 1);
    assert!(!item.submenu()[0].enabled());
    assert_eq!(item.target().opaque(), [1]);
}

#[test]
fn opaque_target_debug_does_not_reveal_bytes() {
    let target = SanitizedContextMenuTarget::from_opaque_bytes([0xde, 0xad, 0xbe, 0xef]);

    assert_eq!(format!("{target:?}"), "SanitizedContextMenuTarget(..)");
}

#[test]
fn public_surface_has_only_generic_construction_methods() {
    let source = [
        include_str!("sanitized_context_projection.rs"),
        include_str!("sanitized_context_projection/target.rs"),
        include_str!("sanitized_context_projection/projection.rs"),
        include_str!("sanitized_context_projection/builder.rs"),
        include_str!("sanitized_context_projection/item.rs"),
    ]
    .join("\n");
    let api_source = source.split("#[cfg(test)]").next().unwrap_or(&source);
    let forbidden = [
        "semantic",
        "command",
        "markdown",
        "kle",
        "geometry",
        "anchor",
        "range",
        "query",
        "content",
        "serialize",
        "clone",
        "payload",
        "hash",
    ];
    let api_source = api_source.to_ascii_lowercase();
    for term in forbidden {
        assert!(
            !api_source.contains(term),
            "forbidden context projection term: {term}"
        );
    }

    let public_methods = api_source.lines().filter_map(|line| {
        let declaration = line.trim_start();
        (declaration.starts_with("pub fn ") || declaration.starts_with("pub const fn "))
            .then_some(declaration)
    });
    let allowed = [
        "from_opaque_bytes",
        "with_unit_capability",
        "new",
        "item",
        "build",
        "accessibility_label_text",
        "with_icon",
        "enabled_state",
        "checked_state",
        "submenu_item",
    ];
    for declaration in public_methods {
        assert!(
            allowed
                .iter()
                .any(|name| declaration.contains(&format!("fn {name}"))),
            "unexpected public method: {declaration}"
        );
    }

    assert!(!api_source.contains("pub fn bytes"));
    assert!(!api_source.contains("pub fn opaque"));
    assert!(!api_source.contains("pub fn stable_fingerprint"));
    assert!(!api_source.contains("pub fn target(&"));
    assert!(!api_source.contains("pub fn label(&"));
    assert!(!api_source.contains("pub fn enabled(&"));
    assert!(!api_source.contains("pub fn checked(&"));
    assert!(!api_source.contains("pub fn submenu(&"));
    assert!(!api_source.contains("derive(Debug, Clone"));
    assert!(!api_source.contains("Serialize"));
}

#[test]
fn context_item_fingerprint_tracks_optional_fields_and_icon_policies() {
    let target = || SanitizedContextMenuTarget::from_opaque_bytes([1, 2, 3]);
    let base = SanitizedContextMenuProjectionBuilder::new()
        .item(
            SanitizedContextMenuItem::new(target(), 1, "項目")
                .accessibility_label_text("項目")
                .with_icon(UiIconProps::new("<svg/>"))
                .enabled_state(false)
                .checked_state(true)
                .submenu_item(SanitizedContextMenuItem::new(target(), 2, "子")),
        )
        .build();
    let changed = SanitizedContextMenuProjectionBuilder::new()
        .item(
            SanitizedContextMenuItem::new(target(), 1, "項目")
                .with_icon(UiIconProps::new("<svg-alt/>"))
                .submenu_item(
                    SanitizedContextMenuItem::new(target(), 2, "子").enabled_state(false),
                ),
        )
        .build();

    assert_ne!(base.stable_fingerprint(), changed.stable_fingerprint());
    assert!(!base.same_as(&changed));
    assert!(format!("{base:?}").contains("item_count"));
}

#[test]
fn context_item_debug_masks_target_and_localized_label() {
    let item = SanitizedContextMenuItem::new(
        SanitizedContextMenuTarget::from_opaque_bytes([91, 92, 93]),
        7,
        "秘密の項目 ⭐️",
    )
    .accessibility_label_text("秘密の説明 ⭐️")
    .with_icon(UiIconProps::new("<svg data-secret='yes'/>"))
    .checked_state(true);

    let debug = format!("{item:?}");
    assert!(debug.contains("order: 7"));
    assert!(debug.contains("checked: true"));
    assert!(!debug.contains("秘密"));
    assert!(!debug.contains("⭐️"));
    assert!(!debug.contains("91"));
    assert!(!debug.contains("data-secret"));
}

#[test]
fn context_fingerprint_distinguishes_every_svg_paint_policy() {
    let fingerprints = [
        UiSvgPaintPolicy::CurrentColor,
        UiSvgPaintPolicy::StrokeOnly,
        UiSvgPaintPolicy::FillOnly,
        UiSvgPaintPolicy::StrokeAndFill,
    ]
    .map(|policy| {
        SanitizedContextMenuProjectionBuilder::new()
            .item(
                SanitizedContextMenuItem::new(
                    SanitizedContextMenuTarget::from_opaque_bytes([1]),
                    0,
                    "項目",
                )
                .with_icon(UiIconProps::new("<svg/>").paint_policy(policy)),
            )
            .build()
            .stable_fingerprint()
    });

    for (index, fingerprint) in fingerprints.iter().enumerate() {
        assert!(!fingerprints[..index].contains(fingerprint));
    }
}
