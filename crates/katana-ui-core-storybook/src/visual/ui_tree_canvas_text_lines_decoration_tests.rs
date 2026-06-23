use super::super::underline_y_offset;
use super::*;
use crate::test_assert::KucTestExpect;

#[test]
fn strikethrough_span_draws_line_even_for_whitespace() {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let renderer = TextRenderer::load(&facade, "code");
    let node = whitespace_strike_node();
    let metrics = UiTreeTextMetrics::for_node(&node);
    let mut canvas = Canvas::new(360, 80, TEST_BACKGROUND);
    let area = UiTreeRenderArea {
        x: 24,
        y: 0,
        width: 300,
        height: 80,
        scroll_y: 0.0,
    };

    UiTreeTextLines::draw_spans(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette: crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
                &ThemeSnapshot::dark(),
            ),
            metrics,
        },
        area.x,
        20,
    );

    let strike_y = 20 + metrics.strikethrough_offset;
    assert!(row_color_count(&canvas, strike_y, STRIKE_COLOR) > 80);
    assert_eq!(0, row_color_count(&canvas, strike_y + 1, STRIKE_COLOR));
}

#[test]
fn underline_span_draws_line_below_document_text() {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let renderer = TextRenderer::load(&facade, "body");
    let node = underline_node();
    let metrics = UiTreeTextMetrics::for_node(&node);
    let mut canvas = Canvas::new(360, 96, TEST_BACKGROUND);
    let area = UiTreeRenderArea {
        x: 24,
        y: 0,
        width: 300,
        height: 96,
        scroll_y: 0.0,
    };

    let palette = crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
        &ThemeSnapshot::dark(),
    );

    UiTreeTextLines::draw_spans(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette,
            metrics,
        },
        area.x,
        20,
    );

    let underline_y = 20 + underline_y_offset(metrics, &node);
    assert!(row_color_count(&canvas, underline_y, palette.link) > 40);
    assert!(
        row_color_count(&canvas, underline_y + 1, palette.link) <= 1,
        "underline must stay on one row with at most platform antialias residue"
    );
    assert!(metrics.strikethrough_offset < metrics.underline_offset);
    assert_eq!(
        metrics.underline_offset,
        underline_y_offset(metrics, &node),
        "KatanA-style underline must follow the text metric underline offset, not the row bottom"
    );
}

#[test]
fn link_underline_uses_visible_text_bounds() {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let renderer = TextRenderer::load(&facade, "body");
    let node = padded_link_node();
    let metrics = UiTreeTextMetrics::for_node(&node);
    let mut canvas = Canvas::new(360, 96, TEST_BACKGROUND);
    let area = UiTreeRenderArea {
        x: 24,
        y: 0,
        width: 300,
        height: 96,
        scroll_y: 0.0,
    };

    let palette = crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
        &ThemeSnapshot::dark(),
    );

    UiTreeTextLines::draw_spans(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette,
            metrics,
        },
        area.x,
        20,
    );

    let underline_y = 20 + underline_y_offset(metrics, &node);
    let (min_x, max_x) = row_color_bounds(&canvas, underline_y, palette.link).kuc_unwrap();
    let renderers = SpanTextRenderers::new(&renderer, &renderer);
    let spans = &node.props().text.spans;
    let first_span_width = span_part_width(renderers, &spans[0], metrics, false);
    let link_span_width = span_part_width(renderers, &spans[1], metrics, false);
    let whitespace_width = whitespace_width(metrics, false);
    assert_eq!(area.x + first_span_width + whitespace_width, min_x);
    assert!(max_x < area.x + first_span_width + link_span_width - whitespace_width);
}

#[test]
fn html_link_underline_uses_aligned_text_bounds() {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let renderer = TextRenderer::load(&facade, "body");
    let node = html_right_link_node();
    let metrics = UiTreeTextMetrics::for_node(&node);
    let mut canvas = Canvas::new(360, 96, TEST_BACKGROUND);
    let area = UiTreeRenderArea {
        x: 24,
        y: 0,
        width: 300,
        height: 96,
        scroll_y: 0.0,
    };

    let palette = crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
        &ThemeSnapshot::dark(),
    );

    UiTreeTextLines::draw_spans(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette,
            metrics,
        },
        area.x,
        20,
    );

    let underline_y = 20 + underline_y_offset(metrics, &node);
    let (min_x, max_x) = row_color_bounds(&canvas, underline_y, palette.link).kuc_unwrap();
    let renderers = SpanTextRenderers::new(&renderer, &renderer);
    let span_width = span_part_width(renderers, &node.props().text.spans[0], metrics, false);
    assert!(
        row_color_count(&canvas, underline_y + 1, palette.link) <= 1,
        "HTML link underline must stay on one row with at most platform antialias residue"
    );
    let expected_min_x = area.x + area.width - span_width;
    assert!(
        min_x >= expected_min_x && min_x <= expected_min_x + 6,
        "HTML link underline must start near the rendered right-aligned text bounds: expected={expected_min_x} actual={min_x}"
    );
    let painted_width = max_x - min_x + 1;
    assert!(
        painted_width <= span_width + 8,
        "HTML link underline must not extend to the whole label row: painted={painted_width} span={span_width}"
    );
    assert!(
        painted_width < area.width.saturating_sub(40),
        "HTML link underline must stay close to text bounds, not row bounds: painted={painted_width} row={}",
        area.width
    );
    assert_eq!(
        metrics.underline_offset,
        underline_y_offset(metrics, &node),
        "HTML link underline must follow the text metric underline offset, not the row bottom"
    );
}

#[test]
fn html_link_underline_uses_visible_text_bounds() {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let renderer = TextRenderer::load(&facade, "body");
    let node = html_padded_link_node();
    let metrics = UiTreeTextMetrics::for_node(&node);
    let mut canvas = Canvas::new(420, 96, TEST_BACKGROUND);
    let area = UiTreeRenderArea {
        x: 24,
        y: 0,
        width: 360,
        height: 96,
        scroll_y: 0.0,
    };

    let palette = crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
        &ThemeSnapshot::dark(),
    );

    UiTreeTextLines::draw_spans(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette,
            metrics,
        },
        area.x,
        20,
    );

    let underline_y = 20 + underline_y_offset(metrics, &node);
    let (min_x, max_x) = row_color_bounds(&canvas, underline_y, palette.link).kuc_unwrap();
    let renderers = SpanTextRenderers::new(&renderer, &renderer);
    let span = &node.props().text.spans[0];
    let (visible_x, visible_width) = span_visible_part_bounds(renderers, span, metrics, false);
    let painted_width = max_x - min_x + 1;

    assert_eq!(
        area.x + visible_x,
        min_x,
        "HTML link underline should start at the visible text origin"
    );
    assert!(
        painted_width <= visible_width + 2,
        "HTML link underline must trim trailing whitespace like KatanA RichText links: painted={painted_width} visible={visible_width}"
    );
}

#[test]
fn underline_without_link_uses_document_text_color() {
    let palette = crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
        &ThemeSnapshot::dark(),
    );
    let span = UiTextSpan {
        text: "Underline".to_string(),
        style: UiTextSpanStyle {
            underline: true,
            ..UiTextSpanStyle::default()
        },
        link_target: String::new(),
    };

    assert_eq!(palette.text, span_color(&span, palette));
}

#[test]
fn link_span_uses_theme_link_token() {
    let mut theme = ThemeSnapshot::dark();
    theme.colors.push(ColorToken {
        name: "link".to_string(),
        rgba: [88, 166, 255, 255],
    });
    let palette = crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(&theme);
    let span = UiTextSpan {
        text: "Link".to_string(),
        style: UiTextSpanStyle::default(),
        link_target: "https://example.com".to_string(),
    };

    assert_eq!(0x58a6ff, span_color(&span, palette));
}
