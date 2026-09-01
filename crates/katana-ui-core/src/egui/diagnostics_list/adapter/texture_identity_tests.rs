use super::super::paint::DiagnosticsPaint;
use super::super::types::{
    DiagnosticsListPaintOperationKind, DiagnosticsListPaintPlan, DiagnosticsListStyle,
};
use super::EguiDiagnosticsListAdapter;

const TEST_TEXT: &str =
    "同じ診断テキストを長い幅制約で rasterize し、保持 texture の更新を検証します ⭐️";
const TEST_HEIGHT: f32 = 180.0;

#[test]
fn retained_diagnostics_texture_uses_raster_pixels_for_style_and_scale_changes() {
    let context = egui::Context::default();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-raster-identity")
        .expect("diagnostics adapter should initialize");
    let standard = DiagnosticsListStyle::standard();
    let mut resized_font = standard.clone();
    resized_font.font.size = 19.0;

    let (initial, initial_identity) =
        paint_text_frame(&context, &mut adapter, &standard, 420.0, 1.0);
    assert!(initial);

    let (reused, reused_identity) = paint_text_frame(&context, &mut adapter, &standard, 420.0, 1.0);
    assert_eq!(initial_identity, reused_identity);
    assert!(!reused);

    let (font_changed, font_identity) =
        paint_text_frame(&context, &mut adapter, &resized_font, 420.0, 1.0);
    assert_ne!(initial_identity, font_identity);
    assert!(font_changed);

    let (scale_changed, scale_identity) =
        paint_text_frame(&context, &mut adapter, &standard, 420.0, 2.0);
    assert_ne!(initial_identity, scale_identity);
    assert!(scale_changed);
}

fn paint_text_frame(
    context: &egui::Context,
    adapter: &mut EguiDiagnosticsListAdapter,
    style: &DiagnosticsListStyle,
    width: f32,
    scale: f32,
) -> (bool, String) {
    context.set_pixels_per_point(scale);
    let mut rendered = None;
    let mut identity = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width, TEST_HEIGHT),
            )),
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let bounds = ui.available_rect_before_wrap();
                let mut plan = DiagnosticsListPaintPlan {
                    surface_bounds: DiagnosticsPaint::ui_rect(bounds),
                    operations: Vec::new(),
                };
                rendered = Some(adapter.paint_text(&mut plan, bounds, TEST_TEXT, style, scale));
                adapter.paint_plan(ui, &plan);
                identity = plan
                    .operations
                    .iter()
                    .find_map(|operation| match &operation.kind {
                        DiagnosticsListPaintOperationKind::Texture { texture, .. } => {
                            Some(texture.identity.clone())
                        }
                        DiagnosticsListPaintOperationKind::Fill { .. } => None,
                    });
            });
        },
    );
    rendered
        .expect("diagnostics text frame should run")
        .expect("diagnostics text should rasterize");
    let emitted_texture = !output.textures_delta.set.is_empty();
    output.textures_delta.clear();
    (
        emitted_texture,
        identity.expect("diagnostics text frame should produce a texture identity"),
    )
}
