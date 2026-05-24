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
fn text_renderer_uses_antialiased_edges_for_common_draw_paths() {
    let facade = UiCoreFacade::default();
    let body_renderer = TextRenderer::load(&facade, "body");
    let code_renderer = TextRenderer::load(&facade, "code");

    assert!(
        antialias_pixels_for_draw(&body_renderer, "Body text", 16.0) > 0,
        "body draw path should contain blended glyph edge pixels"
    );
    assert!(
        antialias_pixels_for_draw(&code_renderer, "count 0", SMALL_CODE_TEXT_SIZE) > 0,
        "code draw path should contain blended glyph edge pixels"
    );
    assert!(
        antialias_pixels_for_centered_draw(&body_renderer, "Centered UI", WIDGET_LABEL_TEXT_SIZE)
            > 0,
        "centered draw path should contain blended glyph edge pixels"
    );
}

#[test]
fn text_on_hidpi_canvas_has_more_anti_aliased_pixels_than_nearest_scaled_1x_canvas() {
    const HI_DPI_BACKGROUND: u32 = 0x000000;
    const HI_DPI_TEXT: u32 = 0xffffff;
    let facade = UiCoreFacade::default();
    let renderer = TextRenderer::load(&facade, "body");
    let mut logical = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, HI_DPI_BACKGROUND);
    let mut scaled = Canvas::new_scaled(CANVAS_WIDTH, CANVAS_HEIGHT, 2.0, HI_DPI_BACKGROUND);

    renderer.draw(
        &mut logical,
        "retina",
        TEXT_X,
        TEXT_Y,
        WIDGET_LABEL_TEXT_SIZE,
        HI_DPI_TEXT,
    );
    renderer.draw(
        &mut scaled,
        "retina",
        TEXT_X,
        TEXT_Y,
        WIDGET_LABEL_TEXT_SIZE,
        HI_DPI_TEXT,
    );

    let scaled_from_logical = scale_nearest(&logical, 2);
    let antialias_hidpi = antialias_pixel_count_for_colors(&scaled, HI_DPI_BACKGROUND, HI_DPI_TEXT);
    let antialias_nearest =
        antialias_pixel_count_for_colors(&scaled_from_logical, HI_DPI_BACKGROUND, HI_DPI_TEXT);
    let avg_alpha_hidpi =
        average_alpha_for_antialias_pixels(&scaled, HI_DPI_BACKGROUND, HI_DPI_TEXT);
    let avg_alpha_nearest =
        average_alpha_for_antialias_pixels(&scaled_from_logical, HI_DPI_BACKGROUND, HI_DPI_TEXT);
    let differing_pixels = scaled
        .pixels()
        .iter()
        .zip(scaled_from_logical.pixels())
        .filter(|(lhs, rhs)| lhs != rhs)
        .count();

    assert_eq!(false, scaled.pixels() == scaled_from_logical.pixels());
    assert!(
        antialias_hidpi > 0 && antialias_nearest > 0,
        "both render paths should produce anti-aliased pixels"
    );
    assert!(
        antialias_intensity_levels_count(&scaled, HI_DPI_BACKGROUND, HI_DPI_TEXT)
            > antialias_intensity_levels_count(
                &scaled_from_logical,
                HI_DPI_BACKGROUND,
                HI_DPI_TEXT
            ),
        "hidpi rendering should expose a finer anti-aliased intensity spread than nearest scaling"
    );
    assert_ne!(
        avg_alpha_hidpi, avg_alpha_nearest,
        "hidpi and nearest-scaled rendering should differ in antialias intensity"
    );
    assert!(
        differing_pixels > 0,
        "hidpi rendering should produce different raster than nearest-scaling 1x"
    );
}

#[test]
fn text_raster_cache_keeps_scale_separated_entries_for_hidpi_and_standard_layout() {
    let facade = UiCoreFacade::default();
    let renderer = TextRenderer::load(&facade, "body");
    let mut logical = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);
    let mut scaled = Canvas::new_scaled(CANVAS_WIDTH, CANVAS_HEIGHT, 2.0, BACKGROUND);

    renderer.draw(
        &mut logical,
        "cache check",
        TEXT_X,
        TEXT_Y,
        WIDGET_LABEL_TEXT_SIZE,
        TEXT,
    );
    renderer.draw(
        &mut scaled,
        "cache check",
        TEXT_X,
        TEXT_Y,
        WIDGET_LABEL_TEXT_SIZE,
        TEXT,
    );

    assert_eq!(
        2,
        renderer.cache_stats().entries,
        "scale factor should split raster cache entries"
    );
    assert_eq!(2, renderer.cache_stats().raster_misses);
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

fn antialias_pixels_for_draw(renderer: &TextRenderer, sample: &str, size: f32) -> usize {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);
    renderer.draw(&mut canvas, sample, TEXT_X, TEXT_Y, size, TEXT);
    antialias_pixel_count(&canvas)
}

fn antialias_pixels_for_centered_draw(renderer: &TextRenderer, sample: &str, size: f32) -> usize {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);
    renderer.draw_centered(
        &mut canvas,
        sample,
        TEXT_X,
        TextVerticalBox::new(TEXT_Y, WIDGET_LABEL_BOX_HEIGHT),
        size,
        TEXT,
    );
    antialias_pixel_count(&canvas)
}

fn antialias_pixel_count_for_colors(canvas: &Canvas, background: u32, text: u32) -> usize {
    canvas
        .pixels()
        .iter()
        .filter(|&&pixel| pixel != background && pixel != text)
        .count()
}

fn average_alpha_for_antialias_pixels(canvas: &Canvas, background: u32, text: u32) -> f32 {
    let mut alpha_sum = 0u32;
    let mut count = 0u32;
    for &pixel in canvas.pixels() {
        if pixel == background || pixel == text {
            continue;
        }
        let red = (pixel >> 16) & 0xff;
        let green = (pixel >> 8) & 0xff;
        let blue = pixel & 0xff;
        alpha_sum += (red + green + blue) / 3;
        count += 1;
    }
    if count == 0 {
        return 0.0;
    }
    alpha_sum as f32 / count as f32
}

fn antialias_intensity_levels_count(canvas: &Canvas, background: u32, text: u32) -> usize {
    let mut levels = std::collections::HashSet::<u32>::new();
    for &pixel in canvas.pixels() {
        if pixel == background || pixel == text {
            continue;
        }
        let red = (pixel >> 16) & 0xff;
        let green = (pixel >> 8) & 0xff;
        let blue = pixel & 0xff;
        let intensity = (red + green + blue) / 3;
        levels.insert(intensity);
    }
    levels.len()
}

fn antialias_pixel_count(canvas: &Canvas) -> usize {
    canvas
        .pixels()
        .iter()
        .filter(|&&pixel| pixel != BACKGROUND && pixel != TEXT)
        .count()
}

fn scale_nearest(canvas: &Canvas, scale: usize) -> Canvas {
    let mut output = Canvas::new(canvas.width() * scale, canvas.height() * scale, BACKGROUND);
    for y in 0..canvas.height() {
        for x in 0..canvas.width() {
            let color = canvas.pixels()[y * canvas.width() + x];
            for offset_y in 0..scale {
                for offset_x in 0..scale {
                    output.set(x * scale + offset_x, y * scale + offset_y, color);
                }
            }
        }
    }
    output
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
