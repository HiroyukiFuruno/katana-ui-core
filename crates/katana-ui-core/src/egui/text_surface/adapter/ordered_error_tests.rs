use super::super::events::OrderedKeyboardInput;
use super::super::model::{EguiTextSurfaceAdapter, EguiTextSurfaceInputPolicy};
use crate::atom::{TextArea, TextAreaWrapPolicy};
use crate::egui::text_command_surface::TextCommandSurfaceStyle;
use crate::text_surface::{TextSurface, TextSurfaceEvent, TextSurfaceProps, TextSurfaceViewport};

const OVERSIZED_QUERY_LEN: usize = 16_384;
const INPUT_WIDTH: u32 = 400;
const INPUT_HEIGHT: u32 = 34;

struct ReplaceQuery {
    enabled: bool,
}

impl OrderedKeyboardInput for ReplaceQuery {
    fn before(&mut self, surface: &mut TextSurface, event: &egui::Event) -> bool {
        if self.enabled && matches!(event, egui::Event::Text(_)) {
            surface.synchronize_value("x".repeat(OVERSIZED_QUERY_LEN));
            return true;
        }
        false
    }

    fn after(&mut self, _: &TextSurface, _: &[TextSurfaceEvent]) {}
}

fn surface() -> TextSurface {
    TextSurface::new(TextSurfaceProps::new(
        TextArea::new("query")
            .value("seed")
            .wrap_policy(TextAreaWrapPolicy::None),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, INPUT_WIDTH, INPUT_HEIGHT),
    ))
}

#[test]
fn ordered_input_propagates_initial_raster_error() {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let mut surface = surface();
    let mut style = TextCommandSurfaceStyle::standard().expect("style");
    style.text_raster.font.size = f32::NAN;
    let mut handler = ReplaceQuery { enabled: false };
    let mut result = None;
    let mut frame = context.run_ui(egui::RawInput::default(), |ui| {
        result = Some(adapter.show_with_ordered_input(
            ui,
            &mut surface,
            &style.text_raster,
            &style.text_paint,
            &EguiTextSurfaceInputPolicy::default(),
            &mut handler,
        ));
    });
    frame.textures_delta.clear();
    let error = result.expect("frame").expect_err("invalid style rejected");
    assert!(error.to_string().contains("raster"));
    assert_eq!(surface.state().text_area.value, "seed");
}

#[test]
fn ordered_input_propagates_refresh_error_after_text_changes() {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let mut surface = surface();
    let style = TextCommandSurfaceStyle::standard().expect("style");
    let mut handler = ReplaceQuery { enabled: false };
    adapter.request_focus_for_next_frame(true);
    for _ in 0..3 {
        let mut result = None;
        let mut frame = context.run_ui(egui::RawInput::default(), |ui| {
            result = Some(adapter.show_with_ordered_input(
                ui,
                &mut surface,
                &style.text_raster,
                &style.text_paint,
                &EguiTextSurfaceInputPolicy::default(),
                &mut handler,
            ));
        });
        frame.textures_delta.clear();
        assert!(result.expect("frame").is_ok());
    }
    assert!(surface.state().text_area.focused);
    handler.enabled = true;
    let mut result = None;
    let mut frame = context.run_ui(
        egui::RawInput {
            events: vec![egui::Event::Text("input".into())],
            ..Default::default()
        },
        |ui| {
            result = Some(adapter.show_with_ordered_input(
                ui,
                &mut surface,
                &style.text_raster,
                &style.text_paint,
                &EguiTextSurfaceInputPolicy::default(),
                &mut handler,
            ));
        },
    );
    frame.textures_delta.clear();
    let error = result
        .expect("frame")
        .expect_err("oversized query must fail");
    assert!(error.to_string().contains("pixel limit"));
    assert_eq!(surface.state().text_area.value.len(), OVERSIZED_QUERY_LEN);
}
