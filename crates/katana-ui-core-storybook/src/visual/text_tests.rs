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
const SMALL_CODE_TEXT_SIZE: f32 = 10.0;
const SMALL_CODE_BOX_HEIGHT: f32 = 24.0;
const MAX_CODE_GLYPH_CENTER_DELTA: f32 = 1.5;
const WIDGET_LABEL_TEXT_SIZE: f32 = 14.0;
const WIDGET_LABEL_BOX_HEIGHT: f32 = 28.0;

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

#[test]
fn latin_lowercase_glyphs_keep_the_same_vertical_center() {
    let facade = UiCoreFacade::default();
    let renderer = TextRenderer::load(&facade, "body");
    let samples = ["e", "a", "c", "o", "x", "m", "n", "s"];

    for sample in samples {
        let center_delta = centered_text_delta(&renderer, sample);
        assert!(
            center_delta <= MAX_CENTER_DELTA,
            "{sample} center delta was {center_delta}"
        );
    }
}

#[test]
fn code_role_draws_mixed_japanese_status_text() {
    let facade = UiCoreFacade::default();
    let code_renderer = TextRenderer::load(&facade, "code");
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);

    code_renderer.draw_centered(
        &mut canvas,
        "preset 編集器右クリック",
        TEXT_X,
        TextVerticalBox::new(TEXT_Y, ALIGN_BOX_HEIGHT),
        10.0,
        TEXT,
    );

    assert!(canvas.non_background_pixels(BACKGROUND) > 200);
}

#[test]
fn code_role_digits_and_lowercase_glyphs_share_vertical_center_at_small_size() {
    let facade = UiCoreFacade::default();
    let code_renderer = TextRenderer::load(&facade, "code");
    let samples = ["e", "0", "preset horizontal", "count 0", "state idle"];

    for sample in samples {
        let center_delta = centered_text_delta_with_size(
            &code_renderer,
            sample,
            SMALL_CODE_TEXT_SIZE,
            SMALL_CODE_BOX_HEIGHT,
        );
        assert!(
            center_delta <= MAX_CODE_GLYPH_CENTER_DELTA,
            "{sample} center delta was {center_delta}"
        );
    }
}

#[test]
fn completed_widget_preview_text_boxes_keep_vertical_alignment() {
    let facade = UiCoreFacade::default();
    let body_renderer = TextRenderer::load(&facade, "body");
    let code_renderer = TextRenderer::load(&facade, "code");
    let body_samples = ["Button", "Theme tokens", "保存する", "日本語 UI", "UI 🔷"];
    let code_samples = [
        "preset modern",
        "state idle",
        "setting layout=basic",
        "count 0",
    ];

    for sample in body_samples {
        let center_delta = centered_text_delta_with_size(
            &body_renderer,
            sample,
            WIDGET_LABEL_TEXT_SIZE,
            WIDGET_LABEL_BOX_HEIGHT,
        );
        assert!(
            center_delta <= MAX_CENTER_DELTA,
            "{sample} body center delta was {center_delta}"
        );
    }
    for sample in code_samples {
        let center_delta = centered_text_delta_with_size(
            &code_renderer,
            sample,
            SMALL_CODE_TEXT_SIZE,
            SMALL_CODE_BOX_HEIGHT,
        );
        assert!(
            center_delta <= MAX_CODE_GLYPH_CENTER_DELTA,
            "{sample} code center delta was {center_delta}"
        );
    }
}

#[test]
fn text_renderer_reuses_raster_cache_for_repeated_draws() {
    let facade = UiCoreFacade::default();
    let renderer = TextRenderer::load(&facade, "body");
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);

    renderer.draw_centered(
        &mut canvas,
        "Foundation",
        TEXT_X,
        TextVerticalBox::new(TEXT_Y, WIDGET_LABEL_BOX_HEIGHT),
        WIDGET_LABEL_TEXT_SIZE,
        TEXT,
    );
    let after_first = renderer.cache_stats();
    renderer.draw_centered(
        &mut canvas,
        "Foundation",
        TEXT_X + 80,
        TextVerticalBox::new(TEXT_Y, WIDGET_LABEL_BOX_HEIGHT),
        WIDGET_LABEL_TEXT_SIZE,
        TEXT,
    );
    let after_second = renderer.cache_stats();

    assert_eq!(1, after_first.entries);
    assert_eq!(1, after_first.raster_misses);
    assert_eq!(after_first, after_second);
}

fn centered_text_delta(renderer: &TextRenderer, sample: &str) -> f32 {
    centered_text_delta_with_size(renderer, sample, TEXT_SIZE, ALIGN_BOX_HEIGHT)
}

fn centered_text_delta_with_size(
    renderer: &TextRenderer,
    sample: &str,
    size: f32,
    box_height: f32,
) -> f32 {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);
    renderer.draw_centered(
        &mut canvas,
        sample,
        TEXT_X,
        TextVerticalBox::new(TEXT_Y, box_height),
        size,
        TEXT,
    );
    let bounds = ink_vertical_bounds(&canvas);
    let ink_center = (bounds.top + bounds.bottom) as f32 / 2.0;
    let box_center = TEXT_Y as f32 + box_height / 2.0;
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
