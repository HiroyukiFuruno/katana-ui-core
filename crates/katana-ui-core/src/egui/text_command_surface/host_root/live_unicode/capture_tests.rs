use super::{KucBounds, UiRect, crop_rgba};

#[test]
fn crop_rgba_rejects_overflow_visibility_and_missing_pixels() {
    let canvas = UiRect::new(0, 0, 2, 2);
    let bounds = KucBounds::new(0, 0, 1, 1);
    let pixels = vec![255; 2 * 2 * crate::render_model::RGBA_CHANNEL_COUNT];
    assert_eq!(
        crop_rgba(&pixels, canvas, bounds)
            .expect("one pixel")
            .pixels
            .len(),
        1
    );
    assert!(crop_rgba(&pixels, canvas, KucBounds::new(2, 0, 1, 1)).is_err());
    assert!(crop_rgba(&pixels, canvas, KucBounds::new(u32::MAX, 0, 1, 1)).is_err());
    assert!(crop_rgba(&pixels[..3], canvas, bounds).is_err());
}

#[test]
fn crop_rgba_rejects_coordinate_addition_overflow() {
    let canvas = UiRect::new(0, 0, u32::MAX, u32::MAX);
    let bounds = KucBounds::new(u32::MAX, u32::MAX, 1, 1);
    assert!(crop_rgba(&[], canvas, bounds).is_err());
    assert!(crop_rgba(&[], canvas, KucBounds::new(0, u32::MAX, 1, 1)).is_err());
    assert!(crop_rgba(&[], canvas, KucBounds::new(0, u32::MAX - 2, 1, 1)).is_err());
}

const FRAME_WIDTH: u32 = 320;
const FRAME_HEIGHT: u32 = 200;

fn captured_frame() -> crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame {
    use crate::egui::text_command_surface::{
        EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfacePresentation,
        EguiTextCommandSurfaceRootFactory, TextCommandSurfaceStyle,
    };
    use crate::text_surface::{
        TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
    };
    let surface = TextSurface::new(TextSurfaceProps::new(
        crate::atom::TextArea::new("capture-boundary").value("ab"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, FRAME_WIDTH, FRAME_HEIGHT),
    ));
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"capture-boundary".to_vec(),
        EguiTextCommandSurfacePresentation {
            text_state_id: None,
            text: TextSurfacePresentation::from_props(surface.props()),
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
        },
        TextCommandSurfaceStyle::standard().expect("style"),
    )
    .expect("token");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .expect("root");
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut captured = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(FRAME_WIDTH as f32, FRAME_HEIGHT as f32),
            )),
            ..Default::default()
        },
        |ui| captured = Some(root.show(ui).expect("show")),
    );
    output.textures_delta.clear();
    captured.expect("captured frame")
}

#[test]
fn observation_rejects_missing_and_inconsistent_glyph_geometry() {
    let mut frame = captured_frame();
    assert!(frame.observe_unicode((0, 1)).is_ok());
    frame
        .output
        .evidence_text
        .raster
        .grapheme_bounds
        .retain(|glyph| glyph.byte_start != 0);
    assert!(frame.observe_unicode((0, 1)).is_err());

    let mut frame = captured_frame();
    frame.output.evidence_text.raster.grapheme_bounds[0].height = -1.0;
    assert!(frame.observe_unicode((0, 1)).is_err());

    let mut frame = captured_frame();
    let first = frame.output.evidence_text.raster.grapheme_bounds[0];
    let second = &mut frame.output.evidence_text.raster.grapheme_bounds[1];
    second.x = first.x;
    second.y = first.y;
    second.width = first.width;
    second.height = first.height;
    assert!(frame.observe_unicode((1, 2)).is_err());
}

#[test]
fn observation_rejects_a_crop_before_either_canvas_origin() {
    let mut frame = captured_frame();
    frame.output.evidence_text.record.texture_bounds.x = -(FRAME_WIDTH as i32);
    assert!(frame.observe_unicode((0, 1)).is_err());
    let mut frame = captured_frame();
    frame.output.evidence_text.record.texture_bounds.y = -(FRAME_HEIGHT as i32);
    assert!(frame.observe_unicode((0, 1)).is_err());
}
