#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum TableTextAlignment {
    Left,
    Center,
    Right,
}

pub(super) struct TableLayoutRow {
    pub(super) lines: Vec<Vec<String>>,
    pub(super) height: usize,
}

pub(super) struct UiTreeCanvasTableLayout {
    pub(super) rows: Vec<TableLayoutRow>,
    pub(super) alignments: Vec<TableTextAlignment>,
    pub(super) column_count: usize,
    pub(super) table_width: usize,
    pub(super) column_widths: Vec<usize>,
}

impl UiTreeCanvasTableLayout {
    pub(super) fn total_height(&self) -> usize {
        self.rows.iter().map(|row| row.height).sum()
    }

    pub(super) fn alignment(&self, index: usize) -> TableTextAlignment {
        self.alignments
            .get(index)
            .copied()
            .unwrap_or(TableTextAlignment::Left)
    }

    pub(super) fn column_width(&self, index: usize) -> usize {
        self.column_widths.get(index).copied().unwrap_or(0)
    }

    pub(super) fn column_x_offset(&self, index: usize) -> usize {
        self.column_widths.iter().take(index).sum()
    }
}
