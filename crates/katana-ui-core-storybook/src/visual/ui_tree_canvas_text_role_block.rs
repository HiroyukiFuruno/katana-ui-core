use super::{
    ALERT_STRIPE_WIDTH, CODE_BLOCK_VERTICAL_MARGIN, Canvas, QUOTE_INDENT, QUOTE_STRIPE_WIDTH,
    UiNode, UiTreeCanvasPalette, UiTreeRenderArea, UiTreeTextMetrics, draw_filled_bullet,
    quote_depth, quote_text_padding_left, remaining_width,
};

pub(super) fn draw_heading(
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    y: usize,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
    metrics: UiTreeTextMetrics,
) {
    if !node.props().common.border.visible {
        return;
    }
    canvas.fill_rect(
        x,
        y.saturating_add(metrics.background_height.saturating_sub(2)),
        remaining_width(area, x),
        2,
        palette.selection,
    );
}

pub(super) fn draw_code(
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    y: usize,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
    metrics: UiTreeTextMetrics,
) {
    let quote_depth = quote_depth(node);
    if quote_depth > 0 {
        draw_quote_bars(
            canvas,
            x,
            y,
            quote_depth,
            metrics.background_height,
            palette,
        );
    }
    let code_x = x.saturating_add(quote_depth.saturating_mul(QUOTE_INDENT));
    let code_box = node.props().common.border.visible;
    let inset_code_box = code_box && quote_depth == 0;
    let code_y = if inset_code_box {
        y.saturating_add(CODE_BLOCK_VERTICAL_MARGIN)
    } else {
        y
    };
    let code_height = if inset_code_box
        && metrics.background_height > CODE_BLOCK_VERTICAL_MARGIN.saturating_mul(2)
    {
        metrics
            .background_height
            .saturating_sub(CODE_BLOCK_VERTICAL_MARGIN.saturating_mul(2))
    } else {
        metrics.background_height
    };
    let width = remaining_width(area, code_x);
    canvas.fill_rect(code_x, code_y, width, code_height, palette.code_background);
    if !code_box {
        return;
    }
    canvas.fill_rect(code_x, code_y, width, 1, palette.muted_border);
    canvas.fill_rect(
        code_x,
        code_y.saturating_add(code_height.saturating_sub(1)),
        width,
        1,
        palette.muted_border,
    );
    canvas.fill_rect(code_x, code_y, 1, code_height, palette.muted_border);
    canvas.fill_rect(
        code_x.saturating_add(width.saturating_sub(1)),
        code_y,
        1,
        code_height,
        palette.muted_border,
    );
}

pub(super) fn draw_table(
    _canvas: &mut Canvas,
    _x: usize,
    _y: usize,
    _area: UiTreeRenderArea,
    _palette: UiTreeCanvasPalette,
    _metrics: UiTreeTextMetrics,
) {
}

pub(super) fn draw_quote(
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    y: usize,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
    metrics: UiTreeTextMetrics,
) {
    let depth = quote_depth(node).max(1);
    if node.props().interaction.hovered {
        canvas.fill_rect(
            x,
            y,
            remaining_width(area, x),
            metrics.background_height,
            palette.hover_background,
        );
    }
    draw_quote_bars(canvas, x, y, depth, metrics.background_height, palette);
    if quote_text_padding_left(node) > 0 {
        draw_filled_bullet(
            canvas,
            x.saturating_add(depth.saturating_mul(QUOTE_INDENT)),
            y,
            palette.text,
        );
    }
}

pub(super) fn draw_quote_bars(
    canvas: &mut Canvas,
    x: usize,
    y: usize,
    depth: usize,
    height: usize,
    palette: UiTreeCanvasPalette,
) {
    for index in 0..depth {
        canvas.fill_rect(
            x.saturating_add(index.saturating_mul(QUOTE_INDENT)),
            y,
            QUOTE_STRIPE_WIDTH,
            height,
            palette.muted_border,
        );
    }
}

pub(super) fn draw_media_error(
    canvas: &mut Canvas,
    x: usize,
    y: usize,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
    metrics: UiTreeTextMetrics,
) {
    canvas.fill_rect(
        x,
        y,
        remaining_width(area, x),
        metrics.background_height,
        palette.alert_background,
    );
    canvas.fill_rect(
        x,
        y,
        ALERT_STRIPE_WIDTH,
        metrics.background_height,
        palette.danger_accent,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::render_model::{UiInteractionState, UiNodeKind};
    use katana_ui_core::theme::ThemeSnapshot;

    const WIDTH: usize = 120;
    const HEIGHT: usize = 80;

    #[test]
    fn table_placeholder_is_a_rendering_noop() {
        let mut canvas = Canvas::new(WIDTH, HEIGHT, 0x101010);
        let before = canvas.pixels().to_vec();
        let node = UiNode::new(UiNodeKind::Text, "table");

        draw_table(
            &mut canvas,
            8,
            8,
            area(),
            palette(),
            UiTreeTextMetrics::for_node(&node),
        );

        assert_eq!(before, canvas.pixels());
    }

    #[test]
    fn hovered_quote_and_media_error_use_semantic_backgrounds() {
        let palette = palette();
        let node = UiNode::new(UiNodeKind::Text, "quote").interaction(UiInteractionState {
            hovered: true,
            ..UiInteractionState::default()
        });
        let metrics = UiTreeTextMetrics::for_node(&node);
        let mut quote = Canvas::new(WIDTH, HEIGHT, palette.background);
        let mut error = Canvas::new(WIDTH, HEIGHT, palette.background);

        draw_quote(&mut quote, &node, 8, 8, area(), palette, metrics);
        draw_media_error(&mut error, 8, 8, area(), palette, metrics);

        assert_eq!(palette.hover_background, quote.pixels()[8 * WIDTH + 12]);
        assert_eq!(palette.danger_accent, error.pixels()[8 * WIDTH + 8]);
        assert_eq!(palette.alert_background, error.pixels()[8 * WIDTH + 20]);
    }

    fn palette() -> UiTreeCanvasPalette {
        UiTreeCanvasPalette::from_theme(&ThemeSnapshot::dark())
    }

    fn area() -> UiTreeRenderArea {
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: WIDTH,
            height: HEIGHT,
            scroll_y: 0.0,
        }
    }
}
