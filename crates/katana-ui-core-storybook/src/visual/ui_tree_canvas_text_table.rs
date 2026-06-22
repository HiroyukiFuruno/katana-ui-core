use crate::visual::canvas::Canvas;
use crate::visual::text::TextRenderer;
use crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette;
use crate::visual::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
use crate::visual::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::render_model::UiNode;

const TABLE_BORDER_WIDTH: usize = 1;
const TABLE_CELL_PADDING: usize = 16;
const ASCII_CELL_CHAR_WIDTH: usize = 12;
const WIDE_CELL_CHAR_WIDTH: usize = 22;

#[path = "ui_tree_canvas_text_table_layout.rs"]
mod ui_tree_canvas_text_table_layout;
#[path = "ui_tree_canvas_text_table_types.rs"]
mod ui_tree_canvas_text_table_types;
#[path = "ui_tree_canvas_text_table_wrap.rs"]
mod ui_tree_canvas_text_table_wrap;
use ui_tree_canvas_text_table_layout::build_layout;
use ui_tree_canvas_text_table_types::{TableTextAlignment, UiTreeCanvasTableLayout};

pub(super) struct UiTreeTextTable;

#[derive(Clone, Copy)]
pub(super) struct UiTreeTextTableContext<'a> {
    pub(super) renderer: &'a TextRenderer,
    pub(super) node: &'a UiNode,
    pub(super) area: UiTreeRenderArea,
    pub(super) palette: UiTreeCanvasPalette,
    pub(super) metrics: UiTreeTextMetrics,
}

impl UiTreeTextTable {
    pub(super) fn content_height(
        renderer: &TextRenderer,
        node: &UiNode,
        x: usize,
        area: UiTreeRenderArea,
        metrics: UiTreeTextMetrics,
    ) -> usize {
        build_layout(renderer, node, x, area, metrics)
            .total_height()
            .max(metrics.line_height)
    }

    pub(super) fn draw(
        canvas: &mut Canvas,
        context: UiTreeTextTableContext<'_>,
        x: usize,
        y: usize,
    ) {
        let layout = build_layout(
            context.renderer,
            context.node,
            x,
            context.area,
            context.metrics,
        );
        if layout.rows.is_empty() {
            return;
        }
        let mut row_y = y;
        for (row_index, row) in layout.rows.iter().enumerate() {
            draw_row_background(canvas, x, row_y, &layout, row_index, context.palette);
            let line_count = row.lines.iter().map(|value| value.len()).max().unwrap_or(1);
            let offset = text_start_offset(context.metrics.line_height, row.height, line_count);
            for column_index in 0..layout.column_count {
                let Some(lines) = row.lines.get(column_index) else {
                    continue;
                };
                let mut text_y = row_y
                    .saturating_add(offset)
                    .saturating_add(context.metrics.top_margin);
                for line in lines {
                    context.renderer.draw(
                        canvas,
                        line,
                        cell_text_x(line, x, &layout, column_index),
                        text_y,
                        context.metrics.font_size,
                        context.palette.text,
                    );
                    text_y = text_y.saturating_add(context.metrics.line_height);
                }
            }
            row_y = row_y.saturating_add(row.height);
        }
        draw_grid_lines(
            canvas,
            x,
            y,
            &layout,
            row_y.saturating_sub(y),
            context.palette.muted_border,
        );
    }
}

fn draw_row_background(
    canvas: &mut Canvas,
    x: usize,
    y: usize,
    layout: &UiTreeCanvasTableLayout,
    row_index: usize,
    palette: UiTreeCanvasPalette,
) {
    let row = &layout.rows[row_index];
    let fill = if row_index == 0 {
        palette.table_header_background
    } else if row_index.is_multiple_of(2) {
        palette.table_even_row_background
    } else {
        palette.table_background
    };
    canvas.fill_rect(x, y, layout.table_width, row.height, fill);
}

fn draw_grid_lines(
    canvas: &mut Canvas,
    x: usize,
    y: usize,
    layout: &UiTreeCanvasTableLayout,
    total_height: usize,
    border_color: u32,
) {
    if layout.table_width == 0 || total_height == 0 || layout.column_count == 0 {
        return;
    }

    let table_right_x = x.saturating_add(layout.table_width).saturating_sub(1);
    for column in 0..=layout.column_count {
        let line_x = x.saturating_add(layout.column_x_offset(column));
        canvas.fill_rect(
            line_x.min(table_right_x),
            y,
            TABLE_BORDER_WIDTH,
            total_height,
            border_color,
        );
    }

    let mut boundary_y = y;
    for row in &layout.rows {
        boundary_y = boundary_y.saturating_add(row.height);
        canvas.fill_rect(
            x,
            boundary_y.saturating_sub(TABLE_BORDER_WIDTH),
            layout.table_width,
            TABLE_BORDER_WIDTH,
            border_color,
        );
    }
    canvas.fill_rect(x, y, layout.table_width, TABLE_BORDER_WIDTH, border_color);
}

fn cell_text_x(
    line: &str,
    table_x: usize,
    layout: &UiTreeCanvasTableLayout,
    column_index: usize,
) -> usize {
    let column_width = layout.column_width(column_index);
    let column_x = table_x.saturating_add(layout.column_x_offset(column_index));
    let content_width = column_width.saturating_sub(TABLE_CELL_PADDING.saturating_mul(2));
    let text_width = estimated_cell_text_width(line).min(content_width);
    let left = column_x.saturating_add(TABLE_CELL_PADDING);
    match layout.alignment(column_index) {
        TableTextAlignment::Center => {
            left.saturating_add(content_width.saturating_sub(text_width) / 2)
        }
        TableTextAlignment::Right => left.saturating_add(content_width.saturating_sub(text_width)),
        TableTextAlignment::Left => left,
    }
}

fn estimated_cell_text_width(text: &str) -> usize {
    text.chars()
        .map(|character| {
            if character.is_ascii() {
                ASCII_CELL_CHAR_WIDTH
            } else {
                WIDE_CELL_CHAR_WIDTH
            }
        })
        .sum()
}

fn text_start_offset(line_height: usize, row_height: usize, lines: usize) -> usize {
    let line_height = line_height.saturating_mul(lines.max(1));
    row_height.saturating_sub(line_height) / 2
}
