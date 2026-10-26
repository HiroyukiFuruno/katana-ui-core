#![cfg(feature = "egui")]
use std::fs;
use std::path::Path;

use katana_ui_core::egui::text_command_surface::{
    TabStripContextMenuPresentation, TabStripControlPresentation, TabStripCorrelation,
    TabStripGroupCapabilities, TabStripGroupDescriptor, TabStripGroupPopupPresentation,
    TabStripGroupTarget, TabStripMenuEntry, TabStripMenuOperation, TabStripNavigationPresentation,
    TabStripProjection, TabStripProjectionLease, TabStripProposal, TabStripProposalPort,
    TabStripProposalPortError, TabStripScrollPresentation, TabStripSurfaceCapabilities,
    TabStripSwatchDescriptor, TabStripSwatchTarget, TabStripTabCapabilities, TabStripTabDescriptor,
    TabStripTabTarget, TabStripText,
};
use katana_ui_core::molecule::RgbaColor;

struct AcceptingProposalPort;

#[test]
fn host_can_read_typed_rename_text_without_inspecting_opaque_targets() {
    let name = TabStripText::new("日本語 ⭐️");
    assert_eq!(name.as_str(), "日本語 ⭐️");
}

impl TabStripProposalPort for AcceptingProposalPort {
    fn forward_proposal(
        &mut self,
        _proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        Ok(())
    }
}

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
        .join("src/egui/text_command_surface/tab_strip_projection_lease.rs");
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
fn opaque_targets_can_be_retained_and_compared_without_exposing_payloads() {
    let tab = TabStripTabTarget::from_opaque_bytes(b"private-tab");
    let group = TabStripGroupTarget::from_opaque_bytes(b"private-group");
    let swatch = TabStripSwatchTarget::from_opaque_bytes(b"private-swatch");
    assert_eq!(tab, tab.clone());
    assert_eq!(group, group.clone());
    assert_eq!(swatch, swatch.clone());
    assert_ne!(tab, TabStripTabTarget::from_opaque_bytes(b"other-tab"));
    assert_ne!(
        group,
        TabStripGroupTarget::from_opaque_bytes(b"other-group")
    );
    assert_ne!(
        swatch,
        TabStripSwatchTarget::from_opaque_bytes(b"other-swatch")
    );
    assert!(!format!("{tab:?} {group:?} {swatch:?}").contains("private-"));
}

#[test]
fn public_boundary_exercises_every_optional_tab_group_and_navigation_builder() {
    let tab_menu = TabStripContextMenuPresentation::new()
        .entry(TabStripMenuEntry::separator())
        .entry(
            TabStripMenuEntry::submenu(TabStripText::new("Move"), TabStripText::new("Move tab"))
                .child(
                    TabStripMenuEntry::action(
                        TabStripText::new("Close"),
                        TabStripText::new("Close tab"),
                        TabStripMenuOperation::RequestClose,
                    )
                    .enabled(false)
                    .checked(true),
                ),
        );
    assert_eq!(
        format!("{tab_menu:?}"),
        "TabStripContextMenuPresentation(..)"
    );

    let swatch = TabStripSwatchDescriptor::new(
        TabStripSwatchTarget::from_opaque_bytes(b"swatch-secret"),
        RgbaColor::new(1, 2, 3, 255),
    )
    .selected(true)
    .accessibility_label(TabStripText::new("Blue"));
    assert_eq!(format!("{swatch:?}"), "TabStripSwatchDescriptor(..)");

    let tab = TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"tab-secret"),
        TabStripText::new("Document"),
    )
    .tooltip(TabStripText::new("Document tooltip"))
    .accessibility_label(TabStripText::new("Document tab"))
    .capabilities(
        TabStripTabCapabilities::new()
            .active(true)
            .dirty(true)
            .pinned(true)
            .selectable(true)
            .closeable(true)
            .draggable(true)
            .accepts_tab_drop(true)
            .groupable(true)
            .virtual_tab(true),
    )
    .trailing_control(TabStripControlPresentation::new(
        TabStripText::new("Close"),
        TabStripText::new("Close document"),
    ))
    .context_menu(tab_menu);
    let tab_debug = format!("{tab:?}");
    assert!(tab_debug.contains("TabStripTabDescriptor"));
    assert!(!tab_debug.contains("tab-secret"));

    let popup = TabStripGroupPopupPresentation::default()
        .rename_placeholder(TabStripText::new("Group name"))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Ungroup"),
            TabStripText::new("Ungroup tabs"),
            TabStripMenuOperation::Ungroup,
        ));
    assert_eq!(format!("{popup:?}"), "TabStripGroupPopupPresentation(..)");

    let nested = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"nested-secret"),
        TabStripText::new("Nested"),
    );
    let group = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"group-secret"),
        TabStripText::new("Group"),
    )
    .accessibility_label(TabStripText::new("Document group"))
    .capabilities(
        TabStripGroupCapabilities::new()
            .collapsed(true)
            .collapsible(true)
            .menu_available(true)
            .renamable(true)
            .recolorable(true)
            .closeable(true)
            .ungroupable(true)
            .draggable(true)
            .accepts_tab_drop(true),
    )
    .swatch(swatch)
    .tab(tab)
    .group(nested)
    .popup(popup);
    let group_debug = format!("{group:?}");
    assert!(group_debug.contains("TabStripGroupDescriptor"));
    assert!(!group_debug.contains("group-secret"));

    let navigation = TabStripNavigationPresentation::new(
        TabStripControlPresentation::new(
            TabStripText::new("Previous"),
            TabStripText::new("Previous tab"),
        ),
        TabStripControlPresentation::new(TabStripText::new("Next"), TabStripText::new("Next tab")),
    )
    .overflow(TabStripControlPresentation::new(
        TabStripText::new("More"),
        TabStripText::new("More tabs"),
    ));
    assert_eq!(
        format!("{navigation:?}"),
        "TabStripNavigationPresentation(..)"
    );

    let projection = TabStripProjection::new(
        9,
        TabStripCorrelation::from_opaque_bytes(b"correlation-secret"),
    )
    .group(group)
    .capabilities(
        TabStripSurfaceCapabilities::new()
            .previous_available(true)
            .next_available(true)
            .overflow_available(true)
            .restore_available(true)
            .create_group_available(true)
            .tab_drop_at_end_available(true),
    )
    .navigation(navigation)
    .scroll_presentation(TabStripScrollPresentation::new().request_active_reveal(true));
    let lease = TabStripProjectionLease::new(projection).with_proposal_port(AcceptingProposalPort);
    assert_eq!(format!("{lease:?}"), "TabStripProjectionLease(..)");

    for operation in [
        TabStripMenuOperation::CloseAll,
        TabStripMenuOperation::RestoreClosed,
        TabStripMenuOperation::CreateGroup,
    ] {
        assert_eq!(format!("{operation:?}"), "TabStripMenuOperation(..)");
    }
}
