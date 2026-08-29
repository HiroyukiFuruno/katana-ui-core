use std::fs;
use std::path::Path;

use katana_ui_core::molecule::RgbaColor;
use katana_ui_core_egui_adapter::text_command_surface::{
    TabStripControlPresentation, TabStripCorrelation, TabStripGroupCapabilities,
    TabStripGroupDescriptor, TabStripGroupTarget, TabStripNavigationPresentation,
    TabStripProjection, TabStripProjectionLease, TabStripSurfaceCapabilities,
    TabStripSwatchDescriptor, TabStripSwatchTarget, TabStripTabCapabilities, TabStripTabDescriptor,
    TabStripTabTarget, TabStripText,
};

#[test]
fn public_boundary_constructs_nested_generic_projection_without_legacy_types() {
    let nested = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"nested-group"),
        TabStripText::new("子グループ"),
    )
    .tab(
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"nested-tab"),
            TabStripText::new("⭐️"),
        )
        .capabilities(
            TabStripTabCapabilities::new()
                .active(true)
                .groupable(true)
                .virtual_tab(false),
        ),
    );
    let projection = TabStripProjection::new(
        3,
        TabStripCorrelation::from_opaque_bytes(b"opaque-correlation"),
    )
    .group(
        TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"root-group"),
            TabStripText::new("ルート"),
        )
        .capabilities(
            TabStripGroupCapabilities::new()
                .collapsed(false)
                .collapsible(true)
                .menu_available(true),
        )
        .swatch(TabStripSwatchDescriptor::new(
            TabStripSwatchTarget::from_opaque_bytes(b"swatch"),
            RgbaColor::new(74, 144, 217, 255),
        ))
        .group(nested),
    )
    .capabilities(
        TabStripSurfaceCapabilities::new()
            .previous_available(true)
            .next_available(true)
            .overflow_available(true)
            .restore_available(true)
            .create_group_available(true),
    )
    .navigation(TabStripNavigationPresentation::new(
        TabStripControlPresentation::new(TabStripText::new("前へ"), TabStripText::new("前のタブ")),
        TabStripControlPresentation::new(TabStripText::new("次へ"), TabStripText::new("次のタブ")),
    ));
    let lease = TabStripProjectionLease::new(projection);
    assert_eq!(format!("{lease:?}"), "TabStripProjectionLease(..)");
    let _ = TabStripSwatchTarget::from_opaque_bytes(b"opaque-swatch");
}

#[test]
fn source_guard_rejects_legacy_aliases_and_wire_payload_shapes() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/text_command_surface/tab_strip_projection_lease.rs");
    let source = fs::read_to_string(path).expect("tab-strip boundary source");
    let production = source
        .split("#[cfg(test)]")
        .next()
        .expect("production source");
    for forbidden in [
        "Workspace",
        "CloseableTab",
        "SanitizedTab",
        "Serialize",
        "Deserialize",
        "PathBuf",
        "#derive(Clone)",
        "pub fn payload",
        "pub fn target",
        "pub fn label",
        "pub fn order",
    ] {
        assert!(
            !production.contains(forbidden),
            "forbidden boundary symbol/payload: {forbidden}"
        );
    }
}

#[test]
fn opaque_values_do_not_implement_clone_or_serialize_by_source_contract() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/text_command_surface/tab_strip_projection_lease/types.rs");
    let source = fs::read_to_string(path).expect("tab-strip boundary source");
    for type_name in [
        "TabStripTabTarget",
        "TabStripGroupTarget",
        "TabStripSwatchTarget",
        "TabStripProjectionLease",
    ] {
        let start = source
            .find(&format!("struct {type_name}"))
            .or_else(|| source.find(&format!("pub struct {type_name}")))
            .unwrap_or_else(|| panic!("missing {type_name}"));
        let tail = &source[start..source.len().min(start + 500)];
        assert!(!tail.contains("derive(Clone)"), "{type_name} is Clone");
        assert!(!tail.contains("Serialize"), "{type_name} is serializable");
    }
}
