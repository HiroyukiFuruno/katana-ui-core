use super::*;
use crate::egui::text_command_surface::TabStripTabTarget;
use crate::egui::text_command_surface::tab_strip_retained::TabStripRenameDraft;
use crate::egui::text_command_surface::{
    TabStripGroupCapabilities, TabStripGroupPopupPresentation, TabStripGroupTarget,
};

#[test]
fn synchronize_lease_rehomes_rename_overlay_by_opaque_target() {
    let target = TabStripGroupTarget::from_opaque_bytes(b"rename-group");
    let initial = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(
            TabStripGroupDescriptor::new(target.clone(), TabStripText::new("G"))
                .capabilities(TabStripGroupCapabilities::new().renamable(true))
                .popup(
                    TabStripGroupPopupPresentation::new()
                        .rename_placeholder(TabStripText::new("Rename")),
                ),
        );
    let mut state = build_state_with_projection(initial);
    state.overlay = TabStripOverlayState::GroupPopup {
        path: "root-group-0".to_owned(),
        anchor: egui::pos2(10.0, 20.0),
        submenu_path: Vec::new(),
        rename: Some(Box::new(TabStripRenameDraft::new("G", "Rename"))),
    };
    let next = TabStripProjection::new(2, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"other"),
            TabStripText::new("Other"),
        ))
        .group(
            TabStripGroupDescriptor::new(target, TabStripText::new("G"))
                .capabilities(TabStripGroupCapabilities::new().renamable(true))
                .popup(
                    TabStripGroupPopupPresentation::new()
                        .rename_placeholder(TabStripText::new("Rename")),
                ),
        );
    state
        .synchronize_lease(TabStripProjectionLease::new(next))
        .expect("lease sync");
    assert!(
        matches!(&state.overlay, TabStripOverlayState::GroupPopup { path, rename: Some(_), .. } if path == "root-group-1")
    );
}
#[test]
fn synchronize_lease_rehomes_tab_menu_by_opaque_target_and_closes_removed_target() {
    let target = TabStripTabTarget::from_opaque_bytes(b"same-label-target");
    let initial = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .tab(TabStripTabDescriptor::new(
            target.clone(),
            TabStripText::new("same label"),
        ))
        .tab(TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"other-target"),
            TabStripText::new("same label"),
        ));
    let mut state = build_state_with_projection(initial);
    state.overlay = TabStripOverlayState::TabMenu {
        path: "root-tab-0".to_owned(),
        anchor: egui::Pos2::ZERO,
        submenu_path: Vec::new(),
    };

    let reordered = TabStripProjection::new(2, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .tab(TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"other-target"),
            TabStripText::new("same label"),
        ))
        .tab(TabStripTabDescriptor::new(
            target,
            TabStripText::new("same label"),
        ));
    state
        .synchronize_lease(TabStripProjectionLease::new(reordered))
        .expect("reordered lease sync");
    assert!(matches!(
        &state.overlay,
        TabStripOverlayState::TabMenu { path, .. } if path == "root-tab-1"
    ));

    state
        .synchronize_lease(TabStripProjectionLease::new(
            TabStripProjection::new(3, TabStripCorrelation::from_opaque_bytes(b"corr")).tab(
                TabStripTabDescriptor::new(
                    TabStripTabTarget::from_opaque_bytes(b"other-target"),
                    TabStripText::new("same label"),
                ),
            ),
        ))
        .expect("removed target lease sync");
    assert!(matches!(state.overlay, TabStripOverlayState::Closed));
}
#[test]
fn synchronize_lease_rehomes_nested_rename_and_closes_removed_target() {
    let target = TabStripGroupTarget::from_opaque_bytes(b"nested");
    let initial = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"parent"),
                TabStripText::new("P"),
            )
            .group(
                TabStripGroupDescriptor::new(target.clone(), TabStripText::new("N"))
                    .capabilities(TabStripGroupCapabilities::new().renamable(true))
                    .popup(
                        TabStripGroupPopupPresentation::new()
                            .rename_placeholder(TabStripText::new("Rename")),
                    ),
            ),
        );
    let mut state = build_state_with_projection(initial);
    state.overlay = TabStripOverlayState::GroupPopup {
        path: "root-group-0-group-0".to_owned(),
        anchor: egui::pos2(10.0, 20.0),
        submenu_path: Vec::new(),
        rename: Some(Box::new(TabStripRenameDraft::new("N", "Rename"))),
    };
    let next = TabStripProjection::new(2, TabStripCorrelation::from_opaque_bytes(b"corr")).group(
        TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"other"),
            TabStripText::new("O"),
        )
        .group(
            TabStripGroupDescriptor::new(target, TabStripText::new("N"))
                .capabilities(TabStripGroupCapabilities::new().renamable(true))
                .popup(
                    TabStripGroupPopupPresentation::new()
                        .rename_placeholder(TabStripText::new("Rename")),
                ),
        ),
    );
    state
        .synchronize_lease(TabStripProjectionLease::new(next))
        .expect("nested lease sync");
    assert!(
        matches!(&state.overlay, TabStripOverlayState::GroupPopup { path, .. } if path == "root-group-0-group-0")
    );
    state
        .synchronize_lease(TabStripProjectionLease::new(TabStripProjection::new(
            3,
            TabStripCorrelation::from_opaque_bytes(b"corr"),
        )))
        .expect("removed lease sync");
    assert!(matches!(state.overlay, TabStripOverlayState::Closed));
}
