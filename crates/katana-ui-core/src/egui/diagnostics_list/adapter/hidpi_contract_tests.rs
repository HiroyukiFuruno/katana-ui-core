use super::super::paint::{DiagnosticsPaint, diagnostics_text_request};
use super::super::types::{
    DiagnosticsListPaintOperationKind, DiagnosticsListPaintPlan, DiagnosticsListStyle,
};
use super::EguiDiagnosticsListAdapter;
use crate::egui::raster_extent::LogicalRasterExtent;

const TEXT: &str = "日本語の診断 ⭐️ を幅制約で描画します";
const WIDTH: f32 = 180.0;
const HEIGHT: f32 = 80.0;

#[test]
fn diagnostics_reservation_paint_wrap_and_clip_share_logical_extents_at_hidpi_scales() {
    let context = egui::Context::default();
    let style = DiagnosticsListStyle::standard();
    for scale in [1.0, 1.5, 2.0] {
        context.set_pixels_per_point(scale);
        let mut adapter = EguiDiagnosticsListAdapter::new(format!("diagnostics-hidpi-{scale}"))
            .expect("diagnostics adapter should initialize");
        let mut observed = None;
        let mut output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(WIDTH, HEIGHT),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let bounds = ui.available_rect_before_wrap();
                    let reserved_width = adapter
                        .text_width(TEXT, &style, scale)
                        .expect("reservation text should rasterize");
                    let raster = adapter
                        .text_rasterizer
                        .rasterize(&diagnostics_text_request(
                            TEXT,
                            &style,
                            Some(bounds.width()),
                            scale,
                        ))
                        .expect("paint text should rasterize");
                    let logical_size = LogicalRasterExtent::size(&raster, scale);
                    let mut plan = DiagnosticsListPaintPlan {
                        surface_bounds: DiagnosticsPaint::ui_rect(bounds),
                        operations: Vec::new(),
                    };
                    adapter
                        .paint_text(&mut plan, bounds, TEXT, &style, scale)
                        .expect("paint text should build a texture operation");
                    let (clip_bounds, texture_bounds) = plan
                        .operations
                        .iter()
                        .find_map(|operation| match &operation.kind {
                            DiagnosticsListPaintOperationKind::Texture { bounds, .. } => {
                                Some((operation.clip_bounds, *bounds))
                            }
                            DiagnosticsListPaintOperationKind::Fill { .. } => None,
                        })
                        .expect("text texture operation");
                    observed = Some((
                        bounds,
                        reserved_width,
                        logical_size,
                        clip_bounds,
                        texture_bounds,
                    ));
                });
            },
        );
        output.textures_delta.clear();
        let (bounds, reserved_width, (width, height), clip_bounds, texture_bounds) =
            observed.expect("diagnostics frame should produce bounds");
        assert_eq!(
            (texture_bounds.width, texture_bounds.height),
            (width, height)
        );
        assert!(reserved_width >= width as f32);
        assert!(texture_bounds.x >= bounds.left().round() as i32);
        assert!(texture_bounds.y >= bounds.top().round() as i32);
        assert_eq!(clip_bounds, DiagnosticsPaint::ui_rect(bounds));
        let first_control = egui::Rect::from_min_size(
            bounds.left_top(),
            egui::vec2(reserved_width + 20.0, bounds.height()),
        );
        let second_control = egui::Rect::from_min_size(
            egui::pos2(first_control.right() + 4.0, first_control.top()),
            egui::vec2(reserved_width + 20.0, first_control.height()),
        );
        assert!(
            !first_control.intersects(second_control),
            "adjacent diagnostic reservations must not intersect at scale {scale}"
        );
        let requested_physical_width = (WIDTH * scale).ceil() as usize;
        assert!(raster_width_at_most(
            &style,
            scale,
            requested_physical_width
        ));
    }
}

fn raster_width_at_most(style: &DiagnosticsListStyle, scale: f32, requested_width: usize) -> bool {
    let mut adapter = EguiDiagnosticsListAdapter::new(format!("diagnostics-wrap-{scale}"))
        .expect("diagnostics adapter should initialize");
    let raster = adapter
        .text_rasterizer
        .rasterize(&diagnostics_text_request(TEXT, style, Some(WIDTH), scale))
        .expect("wrap raster should initialize");
    raster.width <= requested_width
}
