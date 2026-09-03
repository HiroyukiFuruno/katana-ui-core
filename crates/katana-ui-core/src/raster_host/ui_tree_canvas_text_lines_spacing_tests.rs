use super::*;

#[test]
fn split_text_spans_preserve_visible_space_gap() -> Result<(), String> {
    let facade = UiCoreFacade::new(ThemeSnapshot::light());
    let renderer = TextRenderer::load(&facade, "body");
    let node = split_space_node();
    let metrics = UiTreeTextMetrics::for_node(&node);
    let mut canvas = Canvas::new(360, 96, 0xffffff);
    let area = UiTreeRenderArea {
        x: 24,
        y: 0,
        width: 300,
        height: 96,
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
                &ThemeSnapshot::light(),
            ),
            metrics,
        },
        area.x,
        20,
    );

    let gap = widest_empty_column_run_between_ink(&canvas, 0xffffff)
        .ok_or_else(|| "split span text should render ink".to_string())?;
    assert!(
        gap >= 3,
        "space between split spans must produce a visible gap, got {gap}px"
    );
    Ok(())
}

#[test]
fn single_text_span_preserves_document_space_gap() -> Result<(), String> {
    let facade = UiCoreFacade::new(ThemeSnapshot::light());
    let renderer = TextRenderer::load(&facade, "body");
    let node = single_space_node();
    let metrics = UiTreeTextMetrics::for_node(&node);
    let mut canvas = Canvas::new(360, 96, 0xffffff);
    let area = UiTreeRenderArea {
        x: 24,
        y: 0,
        width: 300,
        height: 96,
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
                &ThemeSnapshot::light(),
            ),
            metrics,
        },
        area.x,
        20,
    );

    let gap = widest_empty_column_run_between_ink(&canvas, 0xffffff)
        .ok_or_else(|| "single span text should render ink".to_string())?;
    assert!(
        gap >= whitespace_width(metrics, false),
        "space inside one span must use document whitespace width, got {gap}px"
    );
    Ok(())
}

#[test]
fn emoji_span_draws_os_color_pixels() {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let renderer = TextRenderer::load(&facade, "body");
    let node = emoji_node();
    let metrics = UiTreeTextMetrics::for_node(&node);
    let mut canvas = Canvas::new(160, 96, TEST_BACKGROUND);
    let area = UiTreeRenderArea {
        x: 24,
        y: 0,
        width: 112,
        height: 96,
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

    assert!(chromatic_pixel_count(&canvas) > 32);
}

#[test]
fn compact_heading_2_dark_preview_keeps_readable_ink_origin() -> Result<(), String> {
    let mut theme = ThemeSnapshot::dark();
    theme.fonts.push(katana_ui_core::theme::FontToken {
        name: "document-body".to_string(),
        family: katana_ui_core::theme::FontFamily::Proportional,
        size: 14.0,
        weight: 400,
    });
    let facade = UiCoreFacade::new(theme.clone());
    let renderer = TextRenderer::load(&facade, "body");
    let node: UiNode = Text::new("5.5 Short + Long + Short Columns")
        .text_role("heading-2")
        .into();
    let typography =
        crate::raster_host::ui_tree_canvas_text_metrics::UiTreeDocumentTypography::from_theme(
            &theme,
        );
    let metrics = UiTreeTextMetrics::for_node_with_typography(&node, typography);
    let mut canvas = Canvas::new(640, 96, TEST_BACKGROUND);
    let area = UiTreeRenderArea {
        x: 24,
        y: 0,
        width: 592,
        height: 96,
        scroll_y: 0.0,
    };

    UiTreeTextLines::draw_plain(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette: crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
                &theme,
            ),
            metrics,
        },
        area.x,
        area.x,
        20,
    );

    let (top, _) = vertical_non_background_bounds(&canvas, TEST_BACKGROUND)
        .ok_or_else(|| "heading should draw text ink".to_string())?;
    assert!(
        top >= 26,
        "dark preview heading ink must not be shifted upward into a crushed origin: top={top}"
    );
    Ok(())
}

#[test]
fn compact_heading_2_light_preview_keeps_default_ink_origin() -> Result<(), String> {
    let mut theme = ThemeSnapshot::light();
    theme.fonts.push(katana_ui_core::theme::FontToken {
        name: "document-body".to_string(),
        family: katana_ui_core::theme::FontFamily::Proportional,
        size: 14.0,
        weight: 400,
    });
    let facade = UiCoreFacade::new(theme.clone());
    let renderer = TextRenderer::load(&facade, "body");
    let node: UiNode = Text::new("5.5 Short + Long + Short Columns")
        .text_role("heading-2")
        .into();
    let typography =
        crate::raster_host::ui_tree_canvas_text_metrics::UiTreeDocumentTypography::from_theme(
            &theme,
        );
    let metrics = UiTreeTextMetrics::for_node_with_typography(&node, typography);
    let mut canvas = Canvas::new(640, 96, 0xffffff);
    let area = UiTreeRenderArea {
        x: 24,
        y: 0,
        width: 592,
        height: 96,
        scroll_y: 0.0,
    };

    UiTreeTextLines::draw_plain(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette: crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
                &theme,
            ),
            metrics,
        },
        area.x,
        area.x,
        20,
    );

    let (top, _) = vertical_non_background_bounds(&canvas, 0xffffff)
        .ok_or_else(|| "heading should draw text ink".to_string())?;
    assert!(
        top >= 26,
        "light preview heading ink must keep the existing KatanA reference origin: top={top}"
    );
    Ok(())
}
