use super::*;

#[test]
fn plain_text_fractional_logical_origin_quantizes_at_the_scaled_paint_boundary() {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let renderer = TextRenderer::load(&facade, "body");
    let node: UiNode = Text::new("Plain").text_role("body").into();
    let metrics = UiTreeTextMetrics::for_node(&node);
    let palette = crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
        &ThemeSnapshot::dark(),
    );
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: 160,
        height: 120,
        scroll_y: 0.0,
    };
    let mut integral_canvas = Canvas::new_scaled(160, 120, 2.0, TEST_BACKGROUND);
    let mut fractional_canvas = Canvas::new_scaled(160, 120, 2.0, TEST_BACKGROUND);

    UiTreeTextLines::draw_plain_at_logical_y(
        &mut integral_canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette,
            metrics,
        },
        0,
        0,
        31.0,
    );
    UiTreeTextLines::draw_plain_at_logical_y(
        &mut fractional_canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette,
            metrics,
        },
        0,
        0,
        31.5,
    );

    let first_painted_row = |canvas: &Canvas| {
        canvas
            .pixels()
            .chunks(canvas.width())
            .position(|row| row.iter().any(|pixel| *pixel != TEST_BACKGROUND))
    };
    let integral_top = first_painted_row(&integral_canvas).expect("plain text must paint");
    let fractional_top = first_painted_row(&fractional_canvas).expect("plain text must paint");
    assert_eq!(
        integral_top + 1,
        fractional_top,
        "31.5 logical px must retain its half-pixel origin until scale-2 paint"
    );
}
