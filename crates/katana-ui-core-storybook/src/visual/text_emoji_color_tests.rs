use super::canvas::Canvas;
use super::text::TextRenderer;
use katana_ui_core::facade::UiCoreFacade;
use std::error::Error;

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

#[test]
fn runtime_glyph_bounds_keep_star_variation_selector_as_one_selection_unit()
-> Result<(), Box<dyn Error>> {
    let facade = UiCoreFacade::default();
    let renderer = TextRenderer::load(&facade, "body");
    let mut canvas = Canvas::new(240, 96, BACKGROUND);

    renderer.draw(&mut canvas, "A⭐️B", 16, 12, 32.0, TEXT);

    let run = canvas
        .text_runs()
        .first()
        .ok_or_else(|| std::io::Error::other("runtime text run missing"))?;
    let y = run.y() as i32 + 1;
    let star_start = (run.x()..=run.right())
        .find(|x| run.model().point_to_caret(*x as i32, y) == 1)
        .ok_or_else(|| std::io::Error::other("runtime glyph bounds must expose the star start"))?;
    let star_end = (star_start..=run.right())
        .find(|x| run.model().point_to_caret(*x as i32, y) == 2)
        .ok_or_else(|| std::io::Error::other("runtime glyph bounds must expose the star end"))?;

    assert_eq!(
        "⭐️",
        run.model().selected_text(
            run.model()
                .drag_range((star_start as i32, y), (star_end as i32, y))
        )
    );
    Ok(())
}

fn is_chromatic(pixel: u32) -> bool {
    let red = (pixel >> RED_SHIFT) & CHANNEL_MASK;
    let green = (pixel >> GREEN_SHIFT) & CHANNEL_MASK;
    let blue = pixel & CHANNEL_MASK;
    red != green || green != blue
}
