use super::{
    CIRCLE_SPAN_MID, CIRCLE_SPAN_MID_DY, CIRCLE_SPAN_NARROW, CIRCLE_SPAN_WIDE,
    CIRCLE_SPAN_WIDE_MAX_DY, Canvas, LIST_BULLET_RADIUS, LIST_BULLET_X_OFFSET, LIST_SQUARE_SIZE,
    SQUARE_BULLET_X_INSET, UiNode, UiTreeCanvasPalette, UiTreeTextMetrics, list_depth,
};

pub(super) fn draw_list_marker(
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    y: usize,
    palette: UiTreeCanvasPalette,
    metrics: UiTreeTextMetrics,
) {
    if node.props().label.trim().is_empty() {
        let center_y = list_marker_center_y(y, metrics);
        match list_depth(node) {
            0 => draw_filled_bullet(canvas, x, center_y, palette.text),
            1 => draw_hollow_bullet(canvas, x, center_y, palette.text),
            _ => draw_square_bullet(canvas, x, center_y, palette.text),
        }
    }
}

pub(super) fn list_marker_center_y(y: usize, metrics: UiTreeTextMetrics) -> usize {
    y.saturating_add(metrics.top_margin)
        .saturating_add(metrics.highlight_height / 2)
}

pub(super) fn draw_filled_bullet(canvas: &mut Canvas, x: usize, center_y: usize, color: u32) {
    let center_x = x.saturating_add(LIST_BULLET_X_OFFSET) as isize;
    let center_y = center_y as isize;
    for dy in -LIST_BULLET_RADIUS..=LIST_BULLET_RADIUS {
        let span = circle_span(dy);
        let row_y = center_y + dy;
        if row_y < 0 {
            continue;
        }
        let start_x = center_x - span;
        if start_x < 0 {
            continue;
        }
        canvas.fill_rect(
            start_x as usize,
            row_y as usize,
            (span * 2 + 1) as usize,
            1,
            color,
        );
    }
}

pub(super) fn draw_hollow_bullet(canvas: &mut Canvas, x: usize, center_y: usize, color: u32) {
    let center_x = x.saturating_add(LIST_BULLET_X_OFFSET) as isize;
    let center_y = center_y as isize;
    let radius = LIST_BULLET_RADIUS;
    let inner = radius.saturating_sub(2);
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let distance_squared = dx * dx + dy * dy;
            if distance_squared < inner * inner || distance_squared > radius * radius {
                continue;
            }
            let point_x = center_x + dx;
            let point_y = center_y + dy;
            if point_x < 0 || point_y < 0 {
                continue;
            }
            canvas.set(point_x as usize, point_y as usize, color);
        }
    }
}

pub(super) fn draw_square_bullet(canvas: &mut Canvas, x: usize, center_y: usize, color: u32) {
    canvas.fill_rect(
        x.saturating_add(LIST_BULLET_X_OFFSET.saturating_sub(SQUARE_BULLET_X_INSET)),
        center_y.saturating_sub(LIST_SQUARE_SIZE / 2),
        LIST_SQUARE_SIZE,
        LIST_SQUARE_SIZE,
        color,
    );
}

pub(super) fn circle_span(dy: isize) -> isize {
    match dy.abs() {
        0..=CIRCLE_SPAN_WIDE_MAX_DY => CIRCLE_SPAN_WIDE,
        CIRCLE_SPAN_MID_DY => CIRCLE_SPAN_MID,
        _ => CIRCLE_SPAN_NARROW,
    }
}
