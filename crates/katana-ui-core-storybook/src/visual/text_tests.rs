use super::canvas::Canvas;
use super::text::{TextRenderer, TextVerticalBox};
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::theme::FontFamily;

const BACKGROUND: u32 = 0x1e1e1e;
const TEXT: u32 = 0xd4d4d4;
const CANVAS_WIDTH: usize = 360;
const CANVAS_HEIGHT: usize = 80;
const TEXT_X: usize = 12;
const TEXT_Y: usize = 12;
const TEXT_SIZE: f32 = 18.0;
const ALIGN_BOX_HEIGHT: f32 = 32.0;
const MAX_CENTER_DELTA: f32 = 2.0;

#[test]
fn draws_japanese_and_emoji_text() {
    let facade = UiCoreFacade::default();
    let renderer = TextRenderer::load(&facade, "body");
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);

    renderer.draw(&mut canvas, "日本語 UI 🔷", TEXT_X, TEXT_Y, TEXT_SIZE, TEXT);

    assert!(canvas.non_background_pixels(BACKGROUND) > 500);
}

#[test]
fn resolves_default_and_code_font_roles_from_theme() {
    let facade = UiCoreFacade::default();
    let default_renderer = TextRenderer::load(&facade, facade.default_font_role());
    let code_renderer = TextRenderer::load(&facade, "code");
    let shortcut_renderer = TextRenderer::load(&facade, "shortcut");

    assert_eq!(FontFamily::Proportional, default_renderer.font_family());
    assert_eq!(FontFamily::Monospace, code_renderer.font_family());
    assert_eq!(FontFamily::Monospace, shortcut_renderer.font_family());
}

#[test]
fn mixed_japanese_english_and_emoji_are_vertically_centered() {
    let facade = UiCoreFacade::default();
    let renderer = TextRenderer::load(&facade, "body");
    let code_renderer = TextRenderer::load(&facade, "code");
    let samples = [
        ("English UI", &renderer),
        ("日本語 UI", &renderer),
        ("Text 日本語", &renderer),
        ("UI 🔷", &renderer),
        ("⌘ K", &code_renderer),
    ];

    for (sample, sample_renderer) in samples {
        let center_delta = centered_text_delta(sample_renderer, sample);
        assert!(
            center_delta <= MAX_CENTER_DELTA,
            "{sample} center delta was {center_delta}"
        );
    }
}

fn centered_text_delta(renderer: &TextRenderer, sample: &str) -> f32 {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);
    renderer.draw_centered(
        &mut canvas,
        sample,
        TEXT_X,
        TextVerticalBox::new(TEXT_Y, ALIGN_BOX_HEIGHT),
        TEXT_SIZE,
        TEXT,
    );
    let bounds = ink_vertical_bounds(&canvas);
    let ink_center = (bounds.top + bounds.bottom) as f32 / 2.0;
    let box_center = TEXT_Y as f32 + ALIGN_BOX_HEIGHT / 2.0;
    (ink_center - box_center).abs()
}

struct VerticalBounds {
    top: usize,
    bottom: usize,
}

fn ink_vertical_bounds(canvas: &Canvas) -> VerticalBounds {
    let mut top = canvas.height();
    let mut bottom = 0;
    for (index, pixel) in canvas.pixels().iter().enumerate() {
        if *pixel == BACKGROUND {
            continue;
        }
        let y = index / canvas.width();
        top = top.min(y);
        bottom = bottom.max(y);
    }
    VerticalBounds { top, bottom }
}
