use super::*;

const RASTER_FAILURE_VIEWPORT_WIDTH_PX: f32 = 480.0;
const RASTER_FAILURE_VIEWPORT_HEIGHT_PX: f32 = 40.0;

#[test]
fn render_group_propagates_real_raster_failures_from_header_tab_and_nested_group() {
    let mut state = build_state();
    let context = egui::Context::default();
    let bounds = egui::Rect::from_min_size(
        egui::Pos2::ZERO,
        egui::vec2(
            RASTER_FAILURE_VIEWPORT_WIDTH_PX,
            RASTER_FAILURE_VIEWPORT_HEIGHT_PX,
        ),
    );

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
    let bounds = egui::Rect::from_min_size(
        egui::Pos2::ZERO,
        egui::vec2(ICON_TEST_BOUNDS_WIDTH_PX, RASTER_FAILURE_VIEWPORT_HEIGHT_PX),
    );
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
    let bounds = egui::Rect::from_min_size(
        egui::Pos2::ZERO,
        egui::vec2(220.0, RASTER_FAILURE_VIEWPORT_HEIGHT_PX),
    );
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
