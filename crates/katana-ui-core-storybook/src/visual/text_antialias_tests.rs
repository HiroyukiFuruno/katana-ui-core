use super::canvas::Canvas;
use super::text::TextRenderer;
use super::text_test_support::{
    BACKGROUND, CANVAS_HEIGHT, CANVAS_WIDTH, SMALL_CODE_TEXT_SIZE, TEXT, TEXT_X, TEXT_Y,
    WIDGET_LABEL_BOX_HEIGHT, WIDGET_LABEL_TEXT_SIZE, antialias_intensity_levels_count,
    antialias_pixel_count_for_colors, antialias_pixels_for_centered_draw,
    antialias_pixels_for_draw, average_alpha_for_antialias_pixels, scale_nearest,
};
use katana_ui_core::facade::UiCoreFacade;

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

    assert!(scaled.pixels() != scaled_from_logical.pixels());
    assert!(
        antialias_hidpi > 0 && antialias_nearest > 0,
        "both render paths should produce anti-aliased pixels"
    );
    assert!(
        antialias_intensity_levels_count(&scaled, HI_DPI_BACKGROUND, HI_DPI_TEXT)
            > antialias_intensity_levels_count(
                &scaled_from_logical,
                HI_DPI_BACKGROUND,
                HI_DPI_TEXT,
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
        super::text::TextVerticalBox::new(TEXT_Y, WIDGET_LABEL_BOX_HEIGHT),
        WIDGET_LABEL_TEXT_SIZE,
        TEXT,
    );
    let after_first = renderer.cache_stats();
    renderer.draw_centered(
        &mut canvas,
        "Foundation",
        TEXT_X + 80,
        super::text::TextVerticalBox::new(TEXT_Y, WIDGET_LABEL_BOX_HEIGHT),
        WIDGET_LABEL_TEXT_SIZE,
        TEXT,
    );
    let after_second = renderer.cache_stats();

    assert_eq!(1, after_first.entries);
    assert_eq!(1, after_first.raster_misses);
    assert_eq!(after_first, after_second);
}
