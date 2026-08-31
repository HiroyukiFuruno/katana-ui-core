use super::super::{ICON_SIZE_PX, TabStripIcon};
use super::*;
use crate::text_command_surface::tab_strip_proposal_port::{
    TabStripProposalPort, TabStripProposalPortError,
};
use crate::text_command_surface::{
    TabStripCorrelation, TabStripGroupCapabilities, TabStripGroupDescriptor, TabStripGroupTarget,
    TabStripProjection, TabStripProjectionLease, TabStripTabCapabilities, TabStripTabDescriptor,
    TabStripTabTarget, TabStripText,
};
use katana_ui_core_svg_raster::{UiSvgRasterConfig, UiSvgRasterizer};
use std::sync::Arc;

const PAINT_TEST_VIEWPORT_HEIGHT_PX: f32 = 40.0;
const ICON_TEST_BOUNDS_WIDTH_PX: f32 = 180.0;

#[path = "paint_tests/drag.rs"]
mod drag;
#[path = "paint_tests/raster_failures.rs"]
mod raster_failures;

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

#[test]
fn render_group_renders_nested_children_when_not_collapsed() {
    let mut state = build_state();
    let group = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"group-parent"),
        TabStripText::new("parent"),
    )
    .capabilities(
        TabStripGroupCapabilities::new()
            .accepts_tab_drop(true)
            .collapsible(false),
    )
    .tab(TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"tab-child"),
        TabStripText::new("child"),
    ))
    .group(
        TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"group-child"),
            TabStripText::new("nested"),
        )
        .capabilities(
            TabStripGroupCapabilities::new()
                .accepts_tab_drop(true)
                .collapsible(false),
        )
        .tab(TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"tab-grand"),
            TabStripText::new("grand"),
        )),
    );

    let context = egui::Context::default();
    let mut operations = Vec::new();
    let mut next_x = 0.0;
    let mut active_reveal_pending = false;
    let mut candidate_count = 0;
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        let bounds = ui.available_rect_before_wrap();
        state
            .render_group(
                ui,
                &group,
                "root-group-0".to_string(),
                &mut next_x,
                bounds,
                &mut operations,
                &mut active_reveal_pending,
            )
            .expect("expanded group should render nested items");
        candidate_count = state.drag_candidates.len();
    });
    platform_output.textures_delta.clear();

    assert!(operations.len() > 1);
    assert!(next_x > 0.0);
    assert!(
        candidate_count >= 2,
        "expanded group should collect child candidates"
    );
}

#[test]
fn render_group_skips_nested_children_when_collapsed() {
    let mut state = build_state();
    let group = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"group-parent-collapsed"),
        TabStripText::new("parent"),
    )
    .capabilities(
        TabStripGroupCapabilities::new()
            .collapsed(true)
            .accepts_tab_drop(true)
            .collapsible(false),
    )
    .tab(TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"tab-child-collapsed"),
        TabStripText::new("child"),
    ));

    let context = egui::Context::default();
    let mut operations = Vec::new();
    let mut next_x = 0.0;
    let mut active_reveal_pending = false;
    let mut candidates = 0;
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        let bounds = ui.available_rect_before_wrap();
        state
            .render_group(
                ui,
                &group,
                "root-group-0".to_string(),
                &mut next_x,
                bounds,
                &mut operations,
                &mut active_reveal_pending,
            )
            .expect("collapsed group should still render header");
        candidates = state.drag_candidates.len();
    });
    platform_output.textures_delta.clear();

    assert!(operations.len() > 0);
    assert_eq!(
        candidates, 1,
        "collapsed group should not recurse into child descriptors"
    );
}

#[test]
fn render_icon_control_records_disabled_and_enabled_background_branches() {
    let mut state = build_state();
    let context = egui::Context::default();
    let mut operations = Vec::new();
    let mut x = 0.0;
    let bounds = egui::Rect::from_min_size(
        egui::Pos2::ZERO,
        egui::vec2(ICON_TEST_BOUNDS_WIDTH_PX, PAINT_TEST_VIEWPORT_HEIGHT_PX),
    );

    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        let enabled_presentation = crate::text_command_surface::TabStripControlPresentation::new(
            TabStripText::new("next"),
            TabStripText::new("next"),
        );
        let disabled_presentation = crate::text_command_surface::TabStripControlPresentation::new(
            TabStripText::new("next"),
            TabStripText::new("next"),
        );
        let enabled = TabStripIconControl {
            icon: TabStripIcon::Next,
            presentation: &enabled_presentation,
            enabled: true,
            path: "icon-enabled",
        };
        let disabled = TabStripIconControl {
            icon: TabStripIcon::Next,
            presentation: &disabled_presentation,
            enabled: false,
            path: "icon-disabled",
        };
        state
            .render_icon_control(ui, enabled, &mut x, bounds, &mut operations)
            .expect("enabled icon control should render");
        state
            .render_icon_control(ui, disabled, &mut x, bounds, &mut operations)
            .expect("disabled icon control should render");
    });
    platform_output.textures_delta.clear();

    assert!(
        operations
            .iter()
            .any(|operation| matches!(operation.kind, TabStripPaintOperationKind::Fill { .. }))
    );
    assert!(
        operations
            .iter()
            .any(|operation| matches!(operation.kind, TabStripPaintOperationKind::Texture { .. }))
    );
    assert!(x > 0.0);
}

#[test]
fn render_tab_renders_trailing_control_when_present() {
    let mut state = build_state();
    let mut operations = Vec::new();
    let mut active_reveal_pending = false;
    let mut x = 0.0;
    let tab = TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"tab-control"),
        TabStripText::new("tab"),
    )
    .capabilities(TabStripTabCapabilities::new().closeable(true))
    .trailing_control(
        crate::text_command_surface::TabStripControlPresentation::new(
            TabStripText::new("close"),
            TabStripText::new("close"),
        ),
    );

    let context = egui::Context::default();
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .render_tab(
                ui,
                &tab,
                "root-tab-0".to_string(),
                &mut x,
                ui.available_rect_before_wrap(),
                &mut operations,
                &mut active_reveal_pending,
            )
            .expect("tab with trailing control should render");
    });
    platform_output.textures_delta.clear();

    assert!(matches!(tab.capabilities.closeable, true));
    assert!(x > 0.0);
    assert!(
        operations
            .iter()
            .any(|operation| matches!(operation.kind, TabStripPaintOperationKind::Texture { .. }))
    );

    let tab_without_trailing = TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"tab-control"),
        TabStripText::new("tab"),
    )
    .capabilities(TabStripTabCapabilities::new().closeable(false))
    .trailing_control(
        crate::text_command_surface::TabStripControlPresentation::new(
            TabStripText::new("close"),
            TabStripText::new("close"),
        ),
    );
    let mut x_without_trailing = 0.0;
    let mut operations_without_trailing = Vec::new();
    let mut active_reveal_pending = false;
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .render_tab(
                ui,
                &tab_without_trailing,
                "root-tab-1".to_string(),
                &mut x_without_trailing,
                ui.available_rect_before_wrap(),
                &mut operations_without_trailing,
                &mut active_reveal_pending,
            )
            .expect("tab without trailing control should render");
    });
    platform_output.textures_delta.clear();

    assert!(x > x_without_trailing);
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
