use super::CommandResultRow;
use crate::atom::Text;
use crate::interaction::VirtualRange;
use crate::molecule::structured::items::CommandItem;
use crate::molecule::structured::model::CommandPalette;
use crate::molecule::virtualization::MoleculeVirtualization;
use crate::render_model::{
    UiCommandResultProps, UiCursor, UiInteractionState, UiNode, UiNodeKind, UiVisualRole,
};

pub(crate) struct CommandPaletteRenderer;

impl CommandPaletteRenderer {
    pub(crate) fn render(value: CommandPalette) -> UiNode {
        let range = value.virtual_range_model();
        let rows = MoleculeVirtualization::slice_by_range(indexed_rows_for(&value), range.as_ref());
        let label = value.label.clone();
        let mut node = value
            .state
            .node(UiNodeKind::CommandPalette, label)
            .interaction(interaction(
                &value,
                interaction_item_count(&value, rows.len()),
                range.as_ref(),
            ));

        for child in value.children {
            node = node.child(child);
        }
        for (index, row) in rows {
            node = node.child(row_node(row, index, range.as_ref()));
        }
        node
    }
}

fn indexed_rows_for(value: &CommandPalette) -> Vec<(usize, CommandResultRow)> {
    rows_for(value).into_iter().enumerate().collect()
}

fn rows_for(value: &CommandPalette) -> Vec<CommandResultRow> {
    if value.model.command_result_rows.is_empty() {
        return value.items.iter().cloned().map(row_from_legacy).collect();
    }
    value.model.command_result_rows.clone()
}

fn row_from_legacy(value: CommandItem) -> CommandResultRow {
    let mut row = CommandResultRow::new(value.id, value.title);
    if !value.shortcut.is_empty() {
        row.secondary_label = Some(value.shortcut);
    }
    row.disabled = value.disabled;
    row
}

fn interaction(
    value: &CommandPalette,
    item_count: usize,
    range: Option<&VirtualRange>,
) -> UiInteractionState {
    let base = UiInteractionState {
        open: value.state.open,
        has_selection: value.model.command_highlighted_index.is_some() || value.state.has_selection,
        selected_index: value
            .model
            .command_highlighted_index
            .unwrap_or(value.state.selected_index),
        item_count,
        value: interaction_value(value),
        ..UiInteractionState::default()
    };
    MoleculeVirtualization::interaction(base, range)
}

fn interaction_value(value: &CommandPalette) -> String {
    if value.model.query.is_empty() {
        return value.state.value.clone();
    }
    value.model.query.clone()
}

fn interaction_item_count(value: &CommandPalette, row_count: usize) -> usize {
    if row_count == 0 {
        return value.state.item_count;
    }
    row_count
}

fn row_node(row: CommandResultRow, index: usize, range: Option<&VirtualRange>) -> UiNode {
    let set_size = range.map_or(1, |it| it.aria_set_size);
    let props = UiCommandResultProps {
        id: row.id.clone(),
        secondary_label: row.secondary_label.clone().unwrap_or_default(),
        icon: row.icon.clone().unwrap_or_default(),
        shortcut: row
            .shortcut
            .as_ref()
            .map(|it| it.visual_text(crate::atom::RuntimePlatform::MacOS))
            .unwrap_or_default(),
        provider_id: row.provider_id.clone().unwrap_or_default(),
        group_id: row.group_id.clone().unwrap_or_default(),
        disabled_reason: row.disabled_reason.clone().unwrap_or_default(),
        aria_pos_in_set: index + 1,
        aria_set_size: set_size,
    };
    let label = row.label.clone();
    let mut node = UiNode::new(UiNodeKind::CommandResultRow, label)
        .command_result(props)
        .disabled(row.disabled)
        .focusable(true)
        .cursor(UiCursor::Pointer)
        .visual_role(UiVisualRole::Control)
        .accessibility_label(accessibility_label(&row));

    if let Some(icon) = row.icon {
        node = node.child(UiNode::new(UiNodeKind::Icon, icon));
    }
    if let Some(secondary_label) = row.secondary_label {
        node = node.child(Text::new(secondary_label));
    }
    if let Some(shortcut) = row.shortcut {
        node = node.child(shortcut);
    }
    if let Some(reason) = row.disabled_reason {
        node = node.child(Text::new(reason));
    }
    node
}

fn accessibility_label(row: &CommandResultRow) -> String {
    match &row.disabled_reason {
        Some(reason) if row.disabled => format!("{} disabled: {reason}", row.label),
        Some(_) | None => row.label.clone(),
    }
}
