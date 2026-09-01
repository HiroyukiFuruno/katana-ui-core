use super::super::tab_strip_proposal_port::{
    TabStripProposalOperationClass, TabStripProposalPort, TabStripProposalPortError,
};
use super::*;
use crate::egui::text_command_surface::{
    TabStripCorrelation, TabStripGroupDescriptor, TabStripGroupPopupPresentation,
    TabStripGroupTarget, TabStripMenuEntry, TabStripMenuOperation, TabStripProjection,
    TabStripProjectionLease, TabStripSwatchDescriptor, TabStripSwatchTarget, TabStripTabDescriptor,
    TabStripTabTarget, TabStripText,
};
use crate::molecule::RgbaColor;
use std::rc::Rc;
use std::sync::Arc;

const OVERLAY_TEST_VIEWPORT_WIDTH_PX: f32 = 320.0;
const OVERLAY_TEST_VIEWPORT_HEIGHT_PX: f32 = 180.0;

#[path = "overlay_tests/group_popup.rs"]
mod group_popup;

struct RecordingPort {
    operations: Rc<std::cell::RefCell<Vec<TabStripProposalOperationClass>>>,
}

impl RecordingPort {
    fn new(operations: Rc<std::cell::RefCell<Vec<TabStripProposalOperationClass>>>) -> Self {
        Self { operations }
    }
}

impl TabStripProposalPort for RecordingPort {
    fn forward_proposal(
        &mut self,
        proposal: crate::egui::text_command_surface::tab_strip_proposal_port::TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        self.operations
            .borrow_mut()
            .push(proposal.operation_class_for_test());
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

fn first_overlay_row_center(plan: &TabStripPaintPlan) -> egui::Pos2 {
    let Some(bounds) = plan
        .operations
        .iter()
        .find_map(|operation| match operation.kind {
            TabStripPaintOperationKind::Fill { bounds, .. }
                if bounds.width < OVERLAY_WIDTH_PX.round() as u32 =>
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

fn build_state_from_lease(lease: TabStripProjectionLease) -> TabStripRetainedState {
    let config = crate::text_raster::PlatformTextRasterConfig::default();
    let catalog = Arc::new(crate::text_raster::PlatformFontCatalog::new(
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
            crate::egui::text_command_surface::TabStripGroupTarget::from_opaque_bytes(b"group"),
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
            crate::egui::text_command_surface::TabStripGroupTarget::from_opaque_bytes(
                b"group-no-popup",
            ),
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

#[test]
fn render_overlay_is_opened_for_group_popup_with_minimal_prefix() {
    let mut state = build_state();
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"group-popup"),
                TabStripText::new("group"),
            )
            .popup(TabStripGroupPopupPresentation::new().entry(
                TabStripMenuEntry::action(
                    TabStripText::new("open"),
                    TabStripText::new("open"),
                    TabStripMenuOperation::RequestClose,
                ),
            )),
        );
    state.overlay = TabStripOverlayState::GroupPopup {
        path: "root-group-0".to_string(),
        anchor: egui::pos2(10.0, 10.0),
        submenu_path: Vec::new(),
        rename: None,
    };

    let context = egui::Context::default();
    let mut paint_plan = None;
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        paint_plan = state
            .render_overlay(ui, &projection)
            .expect("overlay should render");
    });
    platform_output.textures_delta.clear();

    let plan = paint_plan.expect("overlay should produce a paint plan");
    assert!(plan.surface_bounds.width > 0 || plan.surface_bounds.height > 0);
}

#[test]
fn render_overlay_group_popup_cancel_keeps_no_rename_submission() {
    let mut state = build_state();
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"group-rename"),
                TabStripText::new("group"),
            )
            .popup(
                TabStripGroupPopupPresentation::new()
                    .rename_placeholder(TabStripText::new("グループ名")),
            ),
        );
    state.overlay = TabStripOverlayState::GroupPopup {
        path: "root-group-0".to_owned(),
        anchor: egui::pos2(10.0, 10.0),
        submenu_path: Vec::new(),
        rename: Some(Box::new(TabStripRenameDraft::new("group", "グループ名"))),
    };

    let context = egui::Context::default();
    let mut paint_plan = None;
    let mut output = context.run_ui(
        egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::Escape,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..egui::RawInput::default()
        },
        |ui| {
            paint_plan = state
                .render_overlay(ui, &projection)
                .expect("rename popup should render");
        },
    );
    output.textures_delta.clear();

    assert!(paint_plan.is_none());
    assert!(matches!(state.overlay, TabStripOverlayState::Closed));
}
