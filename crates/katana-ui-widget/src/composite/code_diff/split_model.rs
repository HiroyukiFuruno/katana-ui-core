use super::types::{CodeDiffAlignedRow, CodeDiffLineKind, CodeDiffModel};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CodeDiffVisibleRow {
    Row(CodeDiffAlignedRow),
    Omitted {
        block_index: usize,
        hidden_count: usize,
    },
}

pub(crate) struct CodeDiffVisibleRows;

impl CodeDiffVisibleRows {
    pub(crate) fn visible_rows(
        model: &CodeDiffModel,
        collapse_enabled: bool,
        context_lines: usize,
        expanded_blocks: &[usize],
    ) -> Vec<CodeDiffVisibleRow> {
        if !collapse_enabled {
            return model
                .rows
                .iter()
                .cloned()
                .map(CodeDiffVisibleRow::Row)
                .collect();
        }

        let mut rows = Vec::new();
        let mut block_index = 0;
        let mut cursor = 0;
        while cursor < model.rows.len() {
            if !is_equal(&model.rows[cursor]) {
                rows.push(CodeDiffVisibleRow::Row(model.rows[cursor].clone()));
                cursor += 1;
                continue;
            }

            let start = cursor;
            while cursor < model.rows.len() && is_equal(&model.rows[cursor]) {
                cursor += 1;
            }
            append_equal_block(
                &model.rows[start..cursor],
                block_index,
                context_lines,
                expanded_blocks,
                &mut rows,
            );
            block_index += 1;
        }
        rows
    }
}

fn append_equal_block(
    block: &[CodeDiffAlignedRow],
    block_index: usize,
    context_lines: usize,
    expanded_blocks: &[usize],
    rows: &mut Vec<CodeDiffVisibleRow>,
) {
    let threshold = context_lines.saturating_mul(2).saturating_add(1);
    let expanded = expanded_blocks.contains(&block_index);
    if expanded || block.len() <= threshold {
        rows.extend(block.iter().cloned().map(CodeDiffVisibleRow::Row));
        return;
    }

    let head = context_lines.min(block.len());
    let tail = context_lines.min(block.len().saturating_sub(head));
    rows.extend(
        block
            .iter()
            .take(head)
            .cloned()
            .map(CodeDiffVisibleRow::Row),
    );
    rows.push(CodeDiffVisibleRow::Omitted {
        block_index,
        hidden_count: block.len() - head - tail,
    });
    rows.extend(
        block
            .iter()
            .skip(block.len() - tail)
            .cloned()
            .map(CodeDiffVisibleRow::Row),
    );
}

fn is_equal(row: &CodeDiffAlignedRow) -> bool {
    row.before.kind == CodeDiffLineKind::Equal && row.after.kind == CodeDiffLineKind::Equal
}
