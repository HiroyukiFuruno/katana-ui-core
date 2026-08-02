use super::canvas::Canvas;
use super::text::{RichTextStyle, TextRenderer, TextVerticalBox};
use super::text_test_support::{
    ALIGN_BOX_HEIGHT, BACKGROUND, CANVAS_HEIGHT, CANVAS_WIDTH, MAX_CENTER_DELTA,
    MAX_CODE_GLYPH_CENTER_DELTA, SMALL_CODE_BOX_HEIGHT, SMALL_CODE_TEXT_SIZE, TEXT, TEXT_SIZE,
    TEXT_X, TEXT_Y, WIDGET_LABEL_BOX_HEIGHT, WIDGET_LABEL_TEXT_SIZE, centered_text_delta,
    centered_text_delta_with_size,
};
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::theme::FontFamily;

#[test]
fn draws_japanese_and_emoji_text() {
    let facade = UiCoreFacade::default();
    let renderer = TextRenderer::load(&facade, "body");
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);

    renderer.draw(&mut canvas, "日本語 UI 🔷", TEXT_X, TEXT_Y, TEXT_SIZE, TEXT);

    assert!(canvas.non_background_pixels(BACKGROUND) > 500);
}

#[test]
fn measured_width_matches_drawn_ink_right_edge() -> Result<(), String> {
    let facade = UiCoreFacade::default();
    let renderer = TextRenderer::load(&facade, "body");
    let samples = ["abcdefb", "typed 日本語", "emoji 🔷"];

    for sample in samples {
        let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);
        renderer.draw(&mut canvas, sample, TEXT_X, TEXT_Y, TEXT_SIZE, TEXT);
        let measured_width = renderer.measure_width(sample, TEXT_SIZE);
        let right =
            ink_right_edge(&canvas).ok_or_else(|| "text should render pixels".to_string())?;

        assert_eq!(
            TEXT_X + measured_width - 1,
            right,
            "{sample} measured width should match ink edge"
        );
    }
    Ok(())
}

#[test]
fn text_space_advances_latin_words() {
    let facade = UiCoreFacade::default();
    let renderer = TextRenderer::load(&facade, "body");

    let collapsed = renderer.measure_width("H1Heading", TEXT_SIZE);
    let spaced = renderer.measure_width("H1 Heading", TEXT_SIZE);

    assert!(
        spaced > collapsed,
        "space must advance text width: collapsed={collapsed} spaced={spaced}"
    );
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

fn ink_right_edge(canvas: &Canvas) -> Option<usize> {
    canvas
        .pixels()
        .iter()
        .enumerate()
        .filter_map(|(index, pixel)| {
            if *pixel == BACKGROUND {
                return None;
            }
            Some(index % canvas.width())
        })
        .max()
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
        SMALL_CODE_TEXT_SIZE,
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
fn emoji_entrypoints_draw_and_measure_the_same_content() {
    let facade = UiCoreFacade::default();
    let renderer = TextRenderer::load(&facade, "body");
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);

    renderer.draw_emoji(&mut canvas, "🔷", TEXT_X, TEXT_Y, TEXT_SIZE, TEXT);

    assert!(renderer.measure_emoji_width("🔷", TEXT_SIZE) > 0);
    assert!(canvas.non_background_pixels(BACKGROUND) > 0);
}

#[test]
fn italic_rich_line_reuses_the_raster_cache() {
    let facade = UiCoreFacade::default();
    let renderer = TextRenderer::load(&facade, "body");
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);
    let spans = [renderer.rich_line_span(
        "italic cache",
        RichTextStyle::new(TEXT_SIZE, TEXT).italic(true),
    )];

    renderer.draw_rich_line_signed(&mut canvas, &spans, 0, TEXT_Y);
    let first = renderer.cache_stats();
    renderer.draw_rich_line_signed(&mut canvas, &spans, 0, TEXT_Y);
    let second = renderer.cache_stats();

    assert_eq!(first.raster_misses, second.raster_misses);
    assert!(canvas.non_background_pixels(BACKGROUND) > 0);
}

#[test]
fn signed_text_recording_rejects_negative_origins_and_empty_runs() {
    let facade = UiCoreFacade::default();
    let renderer = TextRenderer::load(&facade, "body");
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);

    renderer.draw_signed_styled(
        &mut canvas,
        "outside",
        -1,
        TEXT_Y,
        RichTextStyle::new(TEXT_SIZE, TEXT),
    );
    renderer.draw_rich_line_signed(&mut canvas, &[], 0, TEXT_Y);

    assert!(canvas.text_runs().is_empty());
}
