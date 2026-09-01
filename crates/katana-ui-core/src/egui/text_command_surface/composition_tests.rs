use super::*;
use crate::atom::TextArea;
use crate::text_surface::{TextSurface, TextSurfaceProps, TextSurfaceViewport};

#[test]
fn ui_rect_rounds_position_and_size_while_clamping_negative_size() {
    let rect = egui::Rect::from_min_max(egui::pos2(1.6, 2.4), egui::pos2(4.4, 1.9));
    let converted = ui_rect(rect);
    assert_eq!(converted.x, 2);
    assert_eq!(converted.y, 2);
    assert_eq!(converted.width, 3);
    assert_eq!(converted.height, 0);
}

#[test]
fn metrics_frame_rejects_a_non_positive_platform_scale_before_rendering() {
    let adapter = EguiTextCommandSurfaceAdapter::with_text_raster_config(
        crate::text_raster::PlatformTextRasterConfig::default(),
    )
    .expect("default text raster configuration");

    let error = begin_metrics_frame(&adapter, 0.0)
        .expect_err("zero platform scale must fail closed before child rendering");

    assert!(matches!(
        error,
        EguiTextCommandSurfaceError::Text(EguiTextSurfaceError::Raster(
            crate::text_raster::PlatformTextRasterError::NonFiniteLayoutExtent
        ))
    ));
}

#[test]
fn root_composition_propagates_a_real_text_surface_raster_failure() {
    let mut adapter = EguiTextCommandSurfaceAdapter::with_text_raster_config(
        crate::text_raster::PlatformTextRasterConfig::default(),
    )
    .expect("default text raster configuration");
    let mut props = TextSurfaceProps::new(
        TextArea::new("missing-font-root").value("本文"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 320, 180),
    );
    props.accessibility_label = "missing font root".to_owned();
    let mut surface = EguiTextCommandSurface::new(TextSurface::new(props));
    let mut style = TextCommandSurfaceStyle::standard().expect("standard root style");
    style.text_raster.font.size = f32::NAN;
    let context = egui::Context::default();
    let mut result = None;

    crate::egui::run_ui_discard(
        &context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(320.0, 180.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| result = Some(adapter.show(ui, &mut surface, &style)),
    );

    let error = result
        .expect("root composition result")
        .expect_err("non-finite text style must fail closed");
    assert!(
        matches!(&error, EguiTextCommandSurfaceError::Text(_)),
        "unexpected root composition error: {error:?}"
    );
}
