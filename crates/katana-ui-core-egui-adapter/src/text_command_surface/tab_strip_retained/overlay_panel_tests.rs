use super::*;
use crate::text_command_surface::{
    TabStripContextMenuPresentation, TabStripCorrelation, TabStripMenuEntry, TabStripMenuOperation,
    TabStripProjection, TabStripProjectionLease, TabStripProposalPort, TabStripProposalPortError,
    TabStripTabDescriptor, TabStripTabTarget, TabStripText,
};
use std::sync::Arc;

const OVERLAY_TEST_VIEWPORT_WIDTH_PX: f32 = 480.0;
const OVERLAY_TEST_VIEWPORT_HEIGHT_PX: f32 = 240.0;
const SUBMENU_TEST_VIEWPORT_WIDTH_PX: f32 = 200.0;
const SUBMENU_TEST_VIEWPORT_HEIGHT_PX: f32 = 120.0;

#[path = "overlay_panel_tests/failures.rs"]
mod failures;
#[path = "overlay_panel_tests/rows_and_submenus.rs"]
mod rows_and_submenus;
#[path = "overlay_panel_tests/tree_frames.rs"]
mod tree_frames;

fn build_state_from_lease(lease: TabStripProjectionLease) -> TabStripRetainedState {
    let config = katana_ui_core_text_raster::PlatformTextRasterConfig::default();
    let catalog = Arc::new(katana_ui_core_text_raster::PlatformFontCatalog::new(
        config.catalog_policy(),
    ));
    TabStripRetainedState::from_lease(lease, catalog, config)
        .expect("tab strip retained state should be constructible")
}

fn build_state() -> TabStripRetainedState {
    build_state_from_lease(TabStripProjectionLease::new(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    )))
}

struct NullPort;

impl TabStripProposalPort for NullPort {
    fn forward_proposal(
        &mut self,
        _proposal: crate::text_command_surface::tab_strip_proposal_port::TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        Ok(())
    }
}

fn overlay_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        events,
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(
                OVERLAY_TEST_VIEWPORT_WIDTH_PX,
                OVERLAY_TEST_VIEWPORT_HEIGHT_PX,
            ),
        )),
        ..Default::default()
    }
}

fn first_row_center(operations: &[TabStripPaintOperation]) -> egui::Pos2 {
    let Some(bounds) = operations
        .iter()
        .find_map(|operation| match operation.kind {
            TabStripPaintOperationKind::Fill { bounds, color_rgba }
                if color_rgba == OVERLAY_BACKGROUND_RGBA
                    && bounds.width < OVERLAY_WIDTH_PX.round() as u32 =>
            {
                Some(bounds)
            }
            _ => None,
        })
    else {
        panic!("overlay row fill operation should be present");
    };
    egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    )
}

#[test]
fn overlay_panel_uses_disabled_row_sense() {
    let mut state = build_state();
    let entries = vec![
        TabStripMenuEntry::action(
            TabStripText::new("disabled"),
            TabStripText::new("disabled"),
            TabStripMenuOperation::RequestClose,
        )
        .enabled(false),
    ];

    let context = egui::Context::default();
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        let mut operations = Vec::new();
        let panel = state
            .render_overlay_panel(
                ui,
                &entries,
                "root",
                egui::pos2(10.0, 10.0),
                0,
                &mut operations,
            )
            .expect("disabled row should render");
        assert!(!panel.closed);
        assert!(panel.open_submenu.is_none());
        assert!(!operations.is_empty());
    });
    platform_output.textures_delta.clear();
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
    let entries = vec![TabStripMenuEntry::action(
        TabStripText::new("outside click target"),
        TabStripText::new("outside click target"),
        TabStripMenuOperation::RequestClose,
    )];
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
                egui::vec2(
                    SUBMENU_TEST_VIEWPORT_WIDTH_PX,
                    SUBMENU_TEST_VIEWPORT_HEIGHT_PX,
                ),
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

#[test]
fn overlay_tree_is_fail_closed_when_click_is_outside_panel_and_protected_bounds() {
    let mut state = build_state();
    let entries = vec![TabStripMenuEntry::action(
        TabStripText::new("outside protected bounds"),
        TabStripText::new("outside protected bounds"),
        TabStripMenuOperation::RequestClose,
    )];
    let protected = egui::Rect::from_min_size(egui::pos2(20.0, 20.0), egui::vec2(20.0, 20.0));
    let context = egui::Context::default();
    let mut outcome = None;
    let mut platform_output = context.run_ui(
        overlay_input(vec![egui::Event::PointerButton {
            pos: egui::pos2(1.0, 1.0),
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }]),
        |ui| {
            outcome = Some(
                state
                    .render_overlay_tree(
                        ui,
                        &entries,
                        "root-with-protected-bounds",
                        egui::pos2(100.0, 100.0),
                        Vec::new(),
                        &[protected],
                    )
                    .expect("overlay traversal should be recoverable"),
            );
        },
    );
    platform_output.textures_delta.clear();

    assert!(outcome.expect("run_ui produced outcome").closed);
}
