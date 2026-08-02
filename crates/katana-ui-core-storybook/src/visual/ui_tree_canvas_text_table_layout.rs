use crate::visual::text::TextRenderer;
use crate::visual::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
use crate::visual::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::render_model::UiNode;

use super::ui_tree_canvas_text_table_types::{TableTextAlignment, UiTreeCanvasTableLayout};
use super::ui_tree_canvas_text_table_wrap::{build_row_layout, remaining_width};

const KATANA_TABLE_CHAR_WIDTH_MUL: usize = 12;
const KATANA_TABLE_BASE_WIDTH_OFFSET: usize = 32;
const KATANA_TABLE_GUARANTEED_MIN_WIDTH: usize = 40;

pub(super) fn build_layout(
    _renderer: &TextRenderer,
    node: &UiNode,
    x: usize,
    area: UiTreeRenderArea,
    metrics: UiTreeTextMetrics,
) -> UiTreeCanvasTableLayout {
    let table = ParsedTable::new(&node.props().label);
    if table.rows.is_empty() {
        return UiTreeCanvasTableLayout {
            rows: Vec::new(),
            alignments: Vec::new(),
            column_count: 0,
            table_width: 0,
            column_widths: Vec::new(),
        };
    }

    let table_width = explicit_width(node).unwrap_or_else(|| remaining_width(area, x));
    let column_count = table.rows.iter().map(Vec::len).max().unwrap_or(1).max(1);
    let column_widths = katana_column_widths(&table.rows, column_count, table_width);
    UiTreeCanvasTableLayout {
        rows: table
            .rows
            .into_iter()
            .map(|row| build_row_layout(&row, &column_widths, metrics))
            .collect(),
        alignments: table.alignments,
        column_count,
        table_width,
        column_widths,
    }
}

fn explicit_width(node: &UiNode) -> Option<usize> {
    match node.props().common.width {
        katana_ui_core::render_model::UiDimension::Px(value) if value > 0 => {
            Some(usize::from(value))
        }
        _ => None,
    }
}

struct ParsedTable {
    rows: Vec<Vec<String>>,
    alignments: Vec<TableTextAlignment>,
}

impl ParsedTable {
    fn new(text: &str) -> Self {
        let mut rows = Vec::new();
        let mut alignments = Vec::new();
        for line in text.lines() {
            let Some(cells) = split_table_row(line) else {
                continue;
            };
            if cells.iter().all(|cell| is_separator_cell(cell)) {
                alignments = cells.iter().map(|cell| separator_alignment(cell)).collect();
                continue;
            }
            rows.push(cells.into_iter().map(String::from).collect());
        }
        Self { rows, alignments }
    }
}

fn split_table_row(line: &str) -> Option<Vec<&str>> {
    let mut cells = line.split('|').collect::<Vec<_>>();
    if cells.first().is_some_and(|cell| cell.trim().is_empty()) {
        cells.remove(0);
    }
    if cells.last().is_some_and(|cell| cell.trim().is_empty()) {
        cells.pop();
    }
    if cells.is_empty() {
        return None;
    }
    let cells = cells.iter().map(|cell| cell.trim()).collect::<Vec<_>>();
    Some(cells)
}

fn is_separator_cell(cell: &str) -> bool {
    !cell.is_empty() && cell.chars().all(|character| matches!(character, '-' | ':'))
}

fn separator_alignment(cell: &str) -> TableTextAlignment {
    let trimmed = cell.trim();
    if trimmed.starts_with(':') && trimmed.ends_with(':') {
        return TableTextAlignment::Center;
    }
    if trimmed.ends_with(':') {
        return TableTextAlignment::Right;
    }
    TableTextAlignment::Left
}

fn katana_column_widths(
    rows: &[Vec<String>],
    column_count: usize,
    table_width: usize,
) -> Vec<usize> {
    if column_count == 0 {
        return Vec::new();
    }
    let max_chars = column_max_chars(rows, column_count);
    let mut ideal = max_chars
        .iter()
        .enumerate()
        .map(|(index, chars)| {
            (
                chars
                    .saturating_mul(KATANA_TABLE_CHAR_WIDTH_MUL)
                    .saturating_add(KATANA_TABLE_BASE_WIDTH_OFFSET),
                index,
            )
        })
        .collect::<Vec<_>>();
    ideal.sort_by_key(|(width, _)| *width);
    allocate_katana_column_widths(column_count, table_width, &ideal)
}

fn column_max_chars(rows: &[Vec<String>], column_count: usize) -> Vec<usize> {
    let mut values = vec![0; column_count];
    for row in rows {
        for (index, cell) in row.iter().enumerate().take(column_count) {
            values[index] = values[index].max(cell.chars().count());
        }
    }
    values
}

fn allocate_katana_column_widths(
    column_count: usize,
    available_width: usize,
    ideal_widths: &[(usize, usize)],
) -> Vec<usize> {
    let mut widths = vec![0; column_count];
    if column_count == 0 {
        return widths;
    }
    let fair_width = available_width / column_count;
    if ideal_widths.iter().all(|(width, _)| *width <= fair_width) {
        widths.fill(fair_width);
        add_remainder_to_last_column(&mut widths, available_width);
        return widths;
    }

    let mut remaining_width = available_width;
    let mut remaining_columns = column_count;
    for &(ideal_width, column_index) in ideal_widths {
        let fair_share = remaining_width / remaining_columns.max(1);
        let width = if ideal_width < fair_share {
            ideal_width
        } else {
            let reserved = KATANA_TABLE_GUARANTEED_MIN_WIDTH
                .saturating_mul(remaining_columns.saturating_sub(1));
            let max_current = remaining_width
                .saturating_sub(reserved)
                .max(KATANA_TABLE_GUARANTEED_MIN_WIDTH);
            fair_share.min(max_current)
        };
        widths[column_index] = width;
        remaining_width = remaining_width.saturating_sub(width);
        remaining_columns = remaining_columns.saturating_sub(1);
    }
    add_remainder_to_last_column(&mut widths, available_width);
    widths
}

fn add_remainder_to_last_column(widths: &mut [usize], available_width: usize) {
    let used = widths.iter().sum::<usize>();
    if let Some(last) = widths.last_mut() {
        *last = last.saturating_add(available_width.saturating_sub(used));
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ParsedTable, allocate_katana_column_widths, build_layout, katana_column_widths,
        split_table_row,
    };
    use crate::visual::text::TextRenderer;
    use crate::visual::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
    use crate::visual::ui_tree_canvas_types::UiTreeRenderArea;
    use katana_ui_core::atom::Text;
    use katana_ui_core::facade::UiCoreFacade;
    use katana_ui_core::render_model::UiDimension;

    #[test]
    fn table_layout_uses_explicit_viewer_width() {
        let facade = UiCoreFacade::default();
        let renderer = TextRenderer::load(&facade, "body");
        let node = Text::new("Header\nTable after list")
            .text_role("table")
            .width(UiDimension::Px(1168))
            .into();
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 1280,
            height: 200,
            scroll_y: 0.0,
        };

        let layout = build_layout(
            &renderer,
            &node,
            56,
            area,
            UiTreeTextMetrics::for_node(&node),
        );

        assert_eq!(1168, layout.table_width);
    }

    #[test]
    fn table_layout_uses_katana_short_long_short_column_allocation() {
        let facade = UiCoreFacade::default();
        let renderer = TextRenderer::load(&facade, "body");
        let node = Text::new(
            "Short | Long Column Test | Short\nID | This text is a very long line to verify horizontal scrolling and word wrapping are working correctly. | Notes",
        )
        .text_role("table")
        .width(UiDimension::Px(500))
        .into();
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 640,
            height: 200,
            scroll_y: 0.0,
        };

        let layout = build_layout(
            &renderer,
            &node,
            0,
            area,
            UiTreeTextMetrics::for_node(&node),
        );

        assert!(
            layout.column_width(0) < layout.column_width(1),
            "short first column must not get the same width as the dominant long column"
        );
        assert!(
            layout.column_width(2) < layout.column_width(1),
            "short last column must not get the same width as the dominant long column"
        );
        assert!(
            layout.column_width(0) >= 92,
            "short text must keep enough width for KUC raster glyphs plus cell padding"
        );
        assert_eq!(500, layout.column_widths.iter().sum::<usize>());
    }

    #[test]
    fn table_layout_uses_full_area_width_when_node_has_no_explicit_width() {
        let facade = UiCoreFacade::default();
        let renderer = TextRenderer::load(&facade, "body");
        let node = Text::new("Header | Value\nShort | Body")
            .text_role("table")
            .into();
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 960,
            height: 200,
            scroll_y: 0.0,
        };

        let layout = build_layout(
            &renderer,
            &node,
            40,
            area,
            UiTreeTextMetrics::for_node(&node),
        );

        assert_eq!(
            920, layout.table_width,
            "table must consume the remaining full Markdown row width, not a stale fixed content width"
        );
    }

    #[test]
    fn table_layout_handles_empty_separator_and_zero_column_boundaries() {
        let facade = UiCoreFacade::default();
        let renderer = TextRenderer::load(&facade, "body");
        let empty_node = Text::new("").text_role("table").into();
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 320,
            height: 120,
            scroll_y: 0.0,
        };

        let empty = build_layout(
            &renderer,
            &empty_node,
            0,
            area,
            UiTreeTextMetrics::for_node(&empty_node),
        );
        assert!(empty.rows.is_empty());
        assert!(empty.alignments.is_empty());
        assert_eq!(0, empty.column_count);
        assert_eq!(0, empty.table_width);
        assert!(empty.column_widths.is_empty());

        let parsed = ParsedTable::new("| --- | :---: | ---: |\n| A | B | C |");
        assert_eq!(1, parsed.rows.len());
        assert_eq!(3, parsed.alignments.len());
        assert_eq!(1, ParsedTable::new("|\n| A |").rows.len());
        assert_eq!(None, split_table_row("|"));
        assert!(katana_column_widths(&[], 0, 100).is_empty());
        assert!(allocate_katana_column_widths(0, 100, &[]).is_empty());
        assert_eq!(
            vec![50, 50],
            allocate_katana_column_widths(2, 100, &[(20, 0), (30, 1)])
        );
    }
}
