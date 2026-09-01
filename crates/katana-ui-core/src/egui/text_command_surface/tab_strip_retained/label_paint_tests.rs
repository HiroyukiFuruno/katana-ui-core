use super::super::{TabStripIcon, TabStripLabelInteraction};
use super::*;
use crate::egui::text_command_surface::{
    TabStripControlPresentation, TabStripCorrelation, TabStripProjection, TabStripProjectionLease,
    TabStripTabCapabilities, TabStripTabDescriptor, TabStripTabTarget, TabStripText,
};
use crate::svg_raster::{UiSvgRasterConfig, UiSvgRasterizer};
use std::sync::Arc;

fn build_state() -> TabStripRetainedState {
    let config = crate::text_raster::PlatformTextRasterConfig::default();
    let catalog = Arc::new(crate::text_raster::PlatformFontCatalog::new(
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
fn render_label_uses_active_background_when_tab_is_active() {
    let mut state = build_state();
    let context = egui::Context::default();
    let mut operations = Vec::new();
    let mut active_reveal_pending = false;

    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        let request = TabStripLabelRenderRequest {
            text: &TabStripText::new("active"),
            path: "active-label".to_string(),
            x: 0.0,
            bounds: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(200.0, 36.0)),
            active: true,
            active_reveal_pending: &mut active_reveal_pending,
            interaction: TabStripLabelInteraction { route_path: None },
            draggable: false,
        };
        state
            .render_label(ui, &mut operations, request)
            .expect("active label should render");
    });
    platform_output.textures_delta.clear();

    assert!(!operations.is_empty());
    assert!(matches!(
        operations[0].kind,
        TabStripPaintOperationKind::Fill { .. }
    ));
}

#[test]
fn render_label_replaces_retained_texture_when_same_path_is_renamed() {
    let mut state = build_state();
    let context = egui::Context::default();
    let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(240.0, 36.0));
    let mut pending = false;
    let mut first_operations = Vec::new();
    let mut first = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .render_label(
                ui,
                &mut first_operations,
                TabStripLabelRenderRequest {
                    text: &TabStripText::new("Before"),
                    path: "stable-tab-path".to_owned(),
                    x: 0.0,
                    bounds,
                    active: false,
                    active_reveal_pending: &mut pending,
                    interaction: TabStripLabelInteraction { route_path: None },
                    draggable: false,
                },
            )
            .expect("initial label should render");
    });
    let first_identity = texture_identity(&first_operations);
    first.textures_delta.clear();

    let mut second_operations = Vec::new();
    let mut second = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .render_label(
                ui,
                &mut second_operations,
                TabStripLabelRenderRequest {
                    text: &TabStripText::new("Renamed tab"),
                    path: "stable-tab-path".to_owned(),
                    x: 0.0,
                    bounds,
                    active: false,
                    active_reveal_pending: &mut pending,
                    interaction: TabStripLabelInteraction { route_path: None },
                    draggable: false,
                },
            )
            .expect("renamed label should render");
    });
    let second_identity = texture_identity(&second_operations);

    assert_ne!(first_identity, second_identity);
    assert!(!second.textures_delta.set.is_empty());
    second.textures_delta.clear();
}

fn texture_identity(operations: &[TabStripPaintOperation]) -> &str {
    operations
        .iter()
        .find_map(|operation| match &operation.kind {
            TabStripPaintOperationKind::Texture { texture, .. } => Some(texture.identity.as_str()),
            TabStripPaintOperationKind::Fill { .. } => None,
        })
        .expect("label frame should contain a texture")
}

#[test]
fn render_tab_trailing_control_uses_pinned_and_close_icons() {
    let mut state = build_state();
    let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(160.0, 36.0));

    let mut unpinned_operations = Vec::new();
    let mut pinned_operations = Vec::new();
    let context = egui::Context::default();
    let mut pinned_x = 0.0;
    let mut unpinned_x = 0.0;

    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        let close_tab = TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"tab"),
            TabStripText::new("tab"),
        )
        .capabilities(
            TabStripTabCapabilities::new()
                .closeable(true)
                .selectable(false),
        );
        let presentation = TabStripControlPresentation::new(
            TabStripText::new("close"),
            TabStripText::new("close"),
        );
        let control = TabStripTrailingControl {
            tab: &close_tab,
            presentation: &presentation,
            path: "tab-close".to_string(),
        };
        state
            .render_tab_trailing_control(
                ui,
                control,
                &mut unpinned_x,
                bounds,
                &mut unpinned_operations,
            )
            .expect("unpinned trailing control should render");
    });
    platform_output.textures_delta.clear();

    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        let pinned_tab = TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"tab"),
            TabStripText::new("tab"),
        )
        .capabilities(TabStripTabCapabilities::new().pinned(true).closeable(false));
        let presentation =
            TabStripControlPresentation::new(TabStripText::new("pin"), TabStripText::new("pin"));
        let pinned_control = TabStripTrailingControl {
            tab: &pinned_tab,
            presentation: &presentation,
            path: "tab-pin".to_string(),
        };
        state
            .render_tab_trailing_control(
                ui,
                pinned_control,
                &mut pinned_x,
                bounds,
                &mut pinned_operations,
            )
            .expect("pinned trailing control should render");
    });
    platform_output.textures_delta.clear();

    let pinned_identity = pinned_operations
        .iter()
        .find_map(|operation| match &operation.kind {
            TabStripPaintOperationKind::Texture { texture, .. } => Some(&texture.identity),
            _ => None,
        });
    let close_identity = unpinned_operations
        .iter()
        .find_map(|operation| match &operation.kind {
            TabStripPaintOperationKind::Texture { texture, .. } => Some(&texture.identity),
            _ => None,
        });

    assert!(matches!(pinned_identity, Some(_)));
    assert!(matches!(close_identity, Some(_)));
    assert_ne!(pinned_identity, close_identity);
}

#[test]
fn raster_icon_distinguishes_pin_and_close() {
    let mut state = build_state();
    let pin = state
        .raster_icon(
            TabStripIcon::Pin,
            RgbaColor::new(
                PRIMARY_TEXT_RGBA[0],
                PRIMARY_TEXT_RGBA[1],
                PRIMARY_TEXT_RGBA[2],
                PRIMARY_TEXT_RGBA[RGBA_ALPHA_INDEX],
            ),
        )
        .expect("pin icon raster should be available");
    let close = state
        .raster_icon(
            TabStripIcon::Close,
            RgbaColor::new(
                PRIMARY_TEXT_RGBA[0],
                PRIMARY_TEXT_RGBA[1],
                PRIMARY_TEXT_RGBA[2],
                PRIMARY_TEXT_RGBA[RGBA_ALPHA_INDEX],
            ),
        )
        .expect("close icon raster should be available");

    assert_ne!(pin.identity, close.identity);
    assert_eq!(pin.width, ICON_SIZE_PX);
    assert_eq!(close.width, ICON_SIZE_PX);
}

#[test]
fn trailing_control_propagates_real_svg_configuration_failure_after_valid_frame() {
    let mut state = build_state();
    let tab = TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"svg-transition"),
        TabStripText::new("tab"),
    )
    .capabilities(TabStripTabCapabilities::new().closeable(true));
    let presentation =
        TabStripControlPresentation::new(TabStripText::new("close"), TabStripText::new("close"));
    let context = egui::Context::default();
    let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(160.0, 36.0));
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .render_tab_trailing_control(
                ui,
                TabStripTrailingControl {
                    tab: &tab,
                    presentation: &presentation,
                    path: "valid-trailing".to_owned(),
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
        observed = Some(state.render_tab_trailing_control(
            ui,
            TabStripTrailingControl {
                tab: &tab,
                presentation: &presentation,
                path: "invalid-trailing".to_owned(),
            },
            &mut 0.0,
            bounds,
            &mut Vec::new(),
        ));
    });
    output.textures_delta.clear();

    assert!(matches!(
        observed.expect("invalid SVG frame should execute the trailing control"),
        Err(TabStripRetainedError::Svg(
            crate::svg_raster::UiSvgRasterError::DimensionsExceedMaximum { .. }
        ))
    ));
}
