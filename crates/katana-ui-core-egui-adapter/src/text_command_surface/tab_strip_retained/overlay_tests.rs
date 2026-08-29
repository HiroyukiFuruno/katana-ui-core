use super::*;
use crate::text_command_surface::{
    TabStripCorrelation, TabStripGroupDescriptor, TabStripProjection, TabStripProjectionLease,
    TabStripTabDescriptor, TabStripTabTarget, TabStripText,
};
use std::sync::Arc;

fn build_state() -> TabStripRetainedState {
    let config = katana_ui_core_text_raster::PlatformTextRasterConfig::default();
    let catalog = Arc::new(katana_ui_core_text_raster::PlatformFontCatalog::new(
        config.catalog_policy(),
    ));
    TabStripRetainedState::from_lease(
        TabStripProjectionLease::new(TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"corr"),
        )),
        catalog,
        config,
    )
    .expect("tab strip retained state should be constructible")
}

#[test]
fn render_overlay_is_fail_closed_when_menu_path_disappears() {
    let mut state = build_state();
    state.overlay = TabStripOverlayState::TabMenu {
        path: "root-tab-0".to_string(),
        anchor: egui::pos2(10.0, 10.0),
        submenu_path: Vec::new(),
    };

    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .tab(TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"tab"),
            TabStripText::new("tab"),
        ));

    let context = egui::Context::default();
    let mut output = None;
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        output = state
            .render_overlay(ui, &projection)
            .expect("overlay render should be fail-closed")
            .map(|_| ());
    });
    platform_output.textures_delta.clear();
    assert!(output.is_none());
}

#[test]
fn render_overlay_is_fail_closed_when_group_popup_path_disappears() {
    let mut state = build_state();
    state.overlay = TabStripOverlayState::GroupPopup {
        path: "root-group-0".to_string(),
        anchor: egui::pos2(10.0, 10.0),
        submenu_path: Vec::new(),
        rename: None,
    };

    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(TabStripGroupDescriptor::new(
            crate::text_command_surface::TabStripGroupTarget::from_opaque_bytes(b"group"),
            TabStripText::new("group"),
        ));

    let context = egui::Context::default();
    let mut output = None;
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        output = state
            .render_overlay(ui, &projection)
            .expect("overlay render should be fail-closed")
            .map(|_| ());
    });
    platform_output.textures_delta.clear();
    assert!(output.is_none());
}

#[test]
fn render_overlay_is_fail_closed_when_group_popup_missing_swatches() {
    let mut state = build_state();
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(TabStripGroupDescriptor::new(
            crate::text_command_surface::TabStripGroupTarget::from_opaque_bytes(b"group-no-popup"),
            TabStripText::new("group"),
        ));
    state.overlay = TabStripOverlayState::TabMenu {
        path: "root-tab-0".to_string(),
        anchor: egui::pos2(10.0, 10.0),
        submenu_path: Vec::new(),
    };

    let context = egui::Context::default();
    let mut output = None;
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        output = state
            .render_overlay(ui, &projection)
            .expect("overlay render should still complete");
    });
    platform_output.textures_delta.clear();

    assert!(output.is_none());
}
