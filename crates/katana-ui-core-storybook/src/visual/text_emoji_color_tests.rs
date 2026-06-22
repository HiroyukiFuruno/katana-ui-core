use super::canvas::Canvas;
use super::text::TextRenderer;
use katana_ui_core::facade::UiCoreFacade;

const BACKGROUND: u32 = 0x101010;
const TEXT: u32 = 0xf5f5f5;
const EMOJI_SIZE: f32 = 64.0;
const EMOJI_SAMPLE_X: usize = 16;
const EMOJI_SAMPLE_Y: usize = 12;
const MIN_CHROMATIC_EMOJI_PIXELS: usize = 32;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const CHANNEL_MASK: u32 = 0xff;

#[test]
fn renderer_preserves_color_pixels_for_os_emoji() {
    let facade = UiCoreFacade::default();
    let renderer = TextRenderer::load(&facade, "body");
    let mut canvas = Canvas::new(120, 96, BACKGROUND);

    renderer.draw(
        &mut canvas,
        "🔥",
        EMOJI_SAMPLE_X,
        EMOJI_SAMPLE_Y,
        EMOJI_SIZE,
        TEXT,
    );

    let chromatic_pixels = canvas
        .pixels()
        .iter()
        .filter(|pixel| **pixel != BACKGROUND)
        .filter(|pixel| is_chromatic(**pixel))
        .count();

    assert!(
        chromatic_pixels > MIN_CHROMATIC_EMOJI_PIXELS,
        "OS emoji must keep color pixels instead of being recolored as monochrome text"
    );
}

fn is_chromatic(pixel: u32) -> bool {
    let red = (pixel >> RED_SHIFT) & CHANNEL_MASK;
    let green = (pixel >> GREEN_SHIFT) & CHANNEL_MASK;
    let blue = pixel & CHANNEL_MASK;
    red != green || green != blue
}
