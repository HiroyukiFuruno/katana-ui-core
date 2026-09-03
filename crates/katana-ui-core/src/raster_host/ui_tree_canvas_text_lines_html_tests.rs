use super::*;

#[test]
fn whitespace_span_width_uses_document_font_metrics() {
    let facade = UiCoreFacade::new(ThemeSnapshot::light());
    let renderer = TextRenderer::load(&facade, "code");
    let node: UiNode = Text::new("code").font_role("code").text_role("code").into();
    let metrics = UiTreeTextMetrics::for_node(&node);
    let span = UiTextSpan::plain("    ");

    let width = span_part_width(
        SpanTextRenderers::new(&renderer, &renderer),
        &span,
        metrics,
        true,
    );

    assert_eq!(whitespace_width(metrics, true) * 4, width);
    assert!(width > 4);
}

#[test]
fn code_font_without_code_role_does_not_preserve_wrap_spaces() {
    let node: UiNode = Text::new("inline")
        .font_role("code")
        .text_role("body")
        .into();

    assert!(!preserves_whitespace(&node));
}

#[test]
fn html_centered_span_uses_measured_text_width() -> Result<(), String> {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let renderer = TextRenderer::load(&facade, "body");
    let label = "KatanA HTML Fixture";
    let node = html_span_node(label, "html-centered");
    let metrics = UiTreeTextMetrics::for_node(&node);
    let mut canvas = Canvas::new(900, 120, TEST_BACKGROUND);
    let area = UiTreeRenderArea {
        x: 56,
        y: 0,
        width: 788,
        height: 120,
        scroll_y: 0.0,
    };

    UiTreeTextLines::draw_spans(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette: crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
                &ThemeSnapshot::dark(),
            ),
            metrics,
        },
        area.x,
        20,
    );

    let bounds = horizontal_ink_bounds(&canvas).ok_or_else(|| "text ink".to_string())?;
    let expected_center = area.x + area.width / 2;
    let actual_center = (bounds.0 + bounds.1) / 2;
    assert!(
        actual_center.abs_diff(expected_center) <= 8,
        "centered span bounds were {bounds:?}, expected center {expected_center}"
    );
    Ok(())
}

#[test]
fn html_centered_overwide_span_clips_from_signed_origin() -> Result<(), String> {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let renderer = TextRenderer::load(&facade, "body");
    let label = "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM";
    let node = html_span_node(label, "html-centered");
    let metrics = UiTreeTextMetrics::for_node(&node);
    let mut canvas = Canvas::new(240, 120, TEST_BACKGROUND);
    let area = UiTreeRenderArea {
        x: 80,
        y: 0,
        width: 120,
        height: 120,
        scroll_y: 0.0,
    };

    assert!(renderer.measure_width(label, metrics.font_size) > area.width + area.x * 2);

    UiTreeTextLines::draw_spans(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette: crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
                &ThemeSnapshot::dark(),
            ),
            metrics,
        },
        area.x,
        20,
    );

    let bounds = horizontal_ink_bounds(&canvas).ok_or_else(|| "text ink".to_string())?;
    assert!(
        bounds.0 <= 2,
        "overwide centered text must be clipped from a negative origin, got bounds {bounds:?}"
    );
    Ok(())
}

#[test]
fn html_right_span_uses_measured_text_width() -> Result<(), String> {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let renderer = TextRenderer::load(&facade, "body");
    let label = "Right aligned link";
    let node = html_span_node(label, "html-right");
    let metrics = UiTreeTextMetrics::for_node(&node);
    let mut canvas = Canvas::new(900, 120, TEST_BACKGROUND);
    let area = UiTreeRenderArea {
        x: 56,
        y: 0,
        width: 788,
        height: 120,
        scroll_y: 0.0,
    };

    UiTreeTextLines::draw_spans(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette: crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
                &ThemeSnapshot::dark(),
            ),
            metrics,
        },
        area.x,
        20,
    );

    let bounds = horizontal_ink_bounds(&canvas).ok_or_else(|| "text ink".to_string())?;
    let expected_right = area.x + area.width;
    assert!(
        bounds.1.abs_diff(expected_right) <= 8,
        "right span bounds were {bounds:?}, expected right {expected_right}"
    );
    Ok(())
}

#[test]
fn html_right_span_uses_explicit_row_width_when_area_is_narrow() -> Result<(), String> {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let renderer = TextRenderer::load(&facade, "body");
    let label = "Right aligned link";
    let node = html_span_node(label, "html-right").width(UiDimension::Px(788));
    let metrics = UiTreeTextMetrics::for_node(&node);
    let mut canvas = Canvas::new(900, 120, TEST_BACKGROUND);
    let area = UiTreeRenderArea {
        x: 56,
        y: 0,
        width: 420,
        height: 120,
        scroll_y: 0.0,
    };

    UiTreeTextLines::draw_spans(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette: crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
                &ThemeSnapshot::dark(),
            ),
            metrics,
        },
        area.x,
        20,
    );

    let bounds = horizontal_ink_bounds(&canvas).ok_or_else(|| "text ink".to_string())?;
    let expected_right = area.x + 788;
    assert!(
        bounds.1.abs_diff(expected_right) <= 8,
        "right span bounds were {bounds:?}, expected explicit right {expected_right}"
    );
    Ok(())
}
