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
    let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(180.0, 40.0));

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

#[test]
fn render_tab_records_drag_release_pending_on_drag_stopped() {
    let draggable_tab = || {
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"drag-stop-tab"),
            TabStripText::new("drag-stop"),
        )
        .capabilities(
            TabStripTabCapabilities::new()
                .draggable(true)
                .selectable(false),
        )
    };
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .tab(draggable_tab());
    let mut state = build_state_from_lease(
        TabStripProjectionLease::new(projection).with_proposal_port(NullPort),
    );
    let tab = draggable_tab();

    let context = egui::Context::default();
    let mut operations = Vec::new();
    let pointer = egui::pos2(12.0, 12.0);
    let drag_pointer = egui::pos2(40.0, 12.0);
    let frames = [
        vec![egui::Event::PointerMoved(pointer)],
        vec![
            egui::Event::PointerMoved(pointer),
            egui::Event::PointerButton {
                pos: pointer,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
        vec![egui::Event::PointerMoved(drag_pointer)],
        vec![egui::Event::PointerButton {
            pos: drag_pointer,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    ];
    for (index, events) in frames.into_iter().enumerate() {
        let mut active_reveal_pending = false;
        let mut x = 0.0;
        operations.clear();
        let mut platform_output = context.run_ui(
            egui::RawInput {
                events,
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(200.0, 40.0),
                )),
                ..Default::default()
            },
            |ui| {
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
                    .expect("drag interaction frame should render");
            },
        );
        platform_output.textures_delta.clear();
        if index == 2 {
            assert!(
                state.drag.is_some(),
                "pointer movement should start the drag"
            );
        }
    }

    assert!(state.drag_release_pending);
    assert!(
        operations.len() > 0,
        "rendering should emit operations even when drag state updates"
    );
    assert_eq!(
        state.drag.as_ref().map(|drag| drag.label.value.as_str()),
        Some("drag-stop")
    );
}

#[test]
fn render_group_propagates_real_raster_failures_from_header_tab_and_nested_group() {
    let mut state = build_state();
    let context = egui::Context::default();
    let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(480.0, 40.0));

    let valid_group = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"valid-group"),
        TabStripText::new("valid"),
    );
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .render_group(
                ui,
                &valid_group,
                "valid-group".to_owned(),
                &mut 0.0,
                bounds,
                &mut Vec::new(),
                &mut false,
            )
            .expect("valid group should warm the real label raster route");
    });
    output.textures_delta.clear();

    let invalid_header = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"invalid-header"),
        TabStripText::new(""),
    );
    let mut header_result = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        header_result = Some(state.render_group(
            ui,
            &invalid_header,
            "invalid-header".to_owned(),
            &mut 0.0,
            bounds,
            &mut Vec::new(),
            &mut false,
        ));
    });
    output.textures_delta.clear();
    assert!(matches!(
        header_result.expect("header frame should execute"),
        Err(TabStripRetainedError::Raster(_))
    ));

    let invalid_tab = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"invalid-tab-parent"),
        TabStripText::new("parent"),
    )
    .tab(TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"invalid-child-tab"),
        TabStripText::new(""),
    ));
    let mut tab_result = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        tab_result = Some(state.render_group(
            ui,
            &invalid_tab,
            "invalid-tab-parent".to_owned(),
            &mut 0.0,
            bounds,
            &mut Vec::new(),
            &mut false,
        ));
    });
    output.textures_delta.clear();
    assert!(matches!(
        tab_result.expect("child tab frame should execute"),
        Err(TabStripRetainedError::Raster(_))
    ));

    let invalid_nested_group = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"invalid-group-parent"),
        TabStripText::new("parent"),
    )
    .group(TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"invalid-child-group"),
        TabStripText::new(""),
    ));
    let mut nested_result = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        nested_result = Some(state.render_group(
            ui,
            &invalid_nested_group,
            "invalid-group-parent".to_owned(),
            &mut 0.0,
            bounds,
            &mut Vec::new(),
            &mut false,
        ));
    });
    output.textures_delta.clear();
    assert!(matches!(
        nested_result.expect("nested group frame should execute"),
        Err(TabStripRetainedError::Raster(_))
    ));
}

#[test]
fn icon_control_propagates_real_svg_configuration_failure_after_valid_frame() {
    let mut state = build_state();
    let context = egui::Context::default();
    let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(180.0, 40.0));
    let presentation = crate::text_command_surface::TabStripControlPresentation::new(
        TabStripText::new("next"),
        TabStripText::new("next"),
    );
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .render_icon_control(
                ui,
                TabStripIconControl {
                    icon: TabStripIcon::Next,
                    presentation: &presentation,
                    enabled: true,
                    path: "valid-icon",
                },
                &mut 0.0,
                bounds,
                &mut Vec::new(),
            )
            .expect("default SVG configuration should render the warm frame");
    });
    output.textures_delta.clear();

    state.svg_rasterizer = UiSvgRasterizer::new(UiSvgRasterConfig {
        cache_capacity: 1,
        max_dimension_px: ICON_SIZE_PX - 1,
    });
    let mut observed = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        observed = Some(state.render_icon_control(
            ui,
            TabStripIconControl {
                icon: TabStripIcon::Next,
                presentation: &presentation,
                enabled: true,
                path: "invalid-icon",
            },
            &mut 0.0,
            bounds,
            &mut Vec::new(),
        ));
    });
    output.textures_delta.clear();

    assert!(matches!(
        observed.expect("invalid SVG frame should execute the icon control"),
        Err(TabStripRetainedError::Svg(
            katana_ui_core_svg_raster::UiSvgRasterError::DimensionsExceedMaximum { .. }
        ))
    ));
}

#[test]
fn render_tab_propagates_real_trailing_svg_failure_after_valid_frame() {
    let mut state = build_state();
    let tab = TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"trailing-svg"),
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
    let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(220.0, 40.0));
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .render_tab(
                ui,
                &tab,
                "valid-trailing-tab".to_owned(),
                &mut 0.0,
                bounds,
                &mut Vec::new(),
                &mut false,
            )
            .expect("default SVG configuration should render the warm frame");
    });
    output.textures_delta.clear();

    state.svg_rasterizer = UiSvgRasterizer::new(UiSvgRasterConfig {
        cache_capacity: 1,
        max_dimension_px: ICON_SIZE_PX - 1,
    });
    let mut observed = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        observed = Some(state.render_tab(
            ui,
            &tab,
            "invalid-trailing-tab".to_owned(),
            &mut 0.0,
            bounds,
            &mut Vec::new(),
            &mut false,
        ));
    });
    output.textures_delta.clear();

    assert!(matches!(
        observed.expect("invalid SVG frame should execute the tab"),
        Err(TabStripRetainedError::Svg(
            katana_ui_core_svg_raster::UiSvgRasterError::DimensionsExceedMaximum { .. }
        ))
    ));
}
