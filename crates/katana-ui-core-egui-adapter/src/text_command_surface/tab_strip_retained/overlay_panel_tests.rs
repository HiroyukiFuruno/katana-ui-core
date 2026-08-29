use super::*;
use crate::text_command_surface::{
    TabStripCorrelation, TabStripMenuEntry, TabStripMenuOperation, TabStripProjection,
    TabStripProjectionLease, TabStripText,
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
fn overlay_tree_truncates_submenu_path_when_deeper_node_is_missing() {
    let mut state = build_state();
    let entries = vec![
        TabStripMenuEntry::submenu(TabStripText::new("submenu"), TabStripText::new("submenu"))
            .child(TabStripMenuEntry::action(
                TabStripText::new("leaf"),
                TabStripText::new("leaf"),
                TabStripMenuOperation::RequestClose,
            )),
    ];

    let context = egui::Context::default();
    let mut outcome = None;
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        let result = state
            .render_overlay_tree(
                ui,
                &entries,
                "root",
                egui::pos2(10.0, 10.0),
                vec![0, 7],
                &[],
            )
            .expect("overlay traversal should be recoverable");
        outcome = Some(result);
    });
    platform_output.textures_delta.clear();

    assert_eq!(
        outcome.expect("run_ui produced outcome").submenu_path,
        vec![0]
    );
}

#[test]
fn overlay_tree_is_fail_closed_by_external_click_when_no_protected_bounds() {
    let mut state = build_state();
    let entries: Vec<TabStripMenuEntry> = Vec::new();
    let context = egui::Context::default();
    let mut outcome = None;
    let mut platform_output = context.run_ui(
        egui::RawInput {
            events: vec![egui::Event::PointerButton {
                pos: egui::pos2(1.0, 1.0),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            }],
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(200.0, 120.0),
            )),
            ..Default::default()
        },
        |ui| {
            let result = state
                .render_overlay_tree(
                    ui,
                    &entries,
                    "root",
                    egui::pos2(100.0, 100.0),
                    Vec::new(),
                    &[],
                )
                .expect("overlay traversal should be recoverable");
            outcome = Some(result);
        },
    );
    platform_output.textures_delta.clear();

    assert!(outcome.expect("run_ui produced outcome").closed);
}
