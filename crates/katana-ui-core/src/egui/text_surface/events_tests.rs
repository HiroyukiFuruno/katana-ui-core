use super::{TextSurfaceInteraction, secondary_pointer_hit};
use crate::atom::TextArea;
use crate::egui::text_surface::model::EguiTextSurfaceInputPolicy;
use crate::render_model::UiRect;
use crate::text_surface::{
    TextSurface, TextSurfaceAction, TextSurfaceEvent, TextSurfaceGraphemeBox, TextSurfaceGutter,
    TextSurfaceGutterRow, TextSurfaceLayout, TextSurfaceProps, TextSurfaceViewport,
};

const TEST_SURFACE_HEIGHT: u32 = 20;

#[path = "events_tests/context_target.rs"]
mod context_target;
#[path = "events_tests/focus_and_gutter.rs"]
mod focus_and_gutter;
#[path = "events_tests/scroll_and_drag.rs"]
mod scroll_and_drag;

fn run_pointer_frame(
    context: &egui::Context,
    surface: &mut TextSurface,
    layout: &TextSurfaceLayout,
    frame: &crate::text_surface::TextSurfaceFrameRecord,
    events: Vec<egui::Event>,
) -> Vec<crate::text_surface::TextSurfaceEvent> {
    let mut captured = Vec::new();
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(100.0, TEST_SURFACE_HEIGHT as f32),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            let (_, response) = ui.allocate_exact_size(
                egui::vec2(100.0, TEST_SURFACE_HEIGHT as f32),
                egui::Sense::click_and_drag(),
            );
            captured = TextSurfaceInteraction::apply_interactions(
                ui,
                &response,
                surface,
                layout,
                frame,
                &EguiTextSurfaceInputPolicy::default(),
                None,
                &[],
            );
        },
    );
    output.textures_delta.clear();
    captured
}
