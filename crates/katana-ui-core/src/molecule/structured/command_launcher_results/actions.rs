use super::{CommandPalette, CommandResultRow, HighlightMove};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandKeyboardInput {
    ArrowUp,
    ArrowDown,
    Home,
    End,
    Enter,
    Escape,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandLauncherAction {
    SetQuery(String),
    MoveHighlight(HighlightMove),
    Keyboard(CommandKeyboardInput),
    SelectHighlighted,
    Execute(String),
    Close,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandLauncherEvent {
    QueryChanged(String),
    ResultHighlighted {
        index: Option<usize>,
        id: Option<String>,
    },
    ResultExecuted {
        id: String,
    },
    Closed,
}

pub(super) fn apply(
    palette: &mut CommandPalette,
    action: CommandLauncherAction,
) -> Vec<CommandLauncherEvent> {
    match action {
        CommandLauncherAction::SetQuery(value) => set_query(palette, value),
        CommandLauncherAction::MoveHighlight(value) => move_highlight(palette, value),
        CommandLauncherAction::Keyboard(value) => keyboard(palette, value),
        CommandLauncherAction::SelectHighlighted => execute_highlighted(palette),
        CommandLauncherAction::Execute(id) => execute_id(palette, &id),
        CommandLauncherAction::Close => vec![CommandLauncherEvent::Closed],
    }
}

fn set_query(palette: &mut CommandPalette, value: String) -> Vec<CommandLauncherEvent> {
    palette.model.query = value.clone();
    palette.state.value = value.clone();
    let highlighted = first_enabled_index(&palette.model.command_result_rows, &value);
    set_highlight(palette, highlighted);
    vec![
        CommandLauncherEvent::QueryChanged(value),
        highlighted_event(palette),
    ]
}

fn keyboard(
    palette: &mut CommandPalette,
    input: CommandKeyboardInput,
) -> Vec<CommandLauncherEvent> {
    match input {
        CommandKeyboardInput::ArrowUp => move_highlight(palette, HighlightMove::Previous),
        CommandKeyboardInput::ArrowDown => move_highlight(palette, HighlightMove::Next),
        CommandKeyboardInput::Home => move_highlight(palette, HighlightMove::First),
        CommandKeyboardInput::End => move_highlight(palette, HighlightMove::Last),
        CommandKeyboardInput::Enter => execute_highlighted(palette),
        CommandKeyboardInput::Escape => vec![CommandLauncherEvent::Closed],
    }
}

fn move_highlight(palette: &mut CommandPalette, value: HighlightMove) -> Vec<CommandLauncherEvent> {
    let next = moved_index(
        palette.model.command_highlighted_index,
        palette.model.command_result_rows.len(),
        value,
    );
    set_highlight(palette, next);
    vec![highlighted_event(palette)]
}

fn execute_highlighted(palette: &CommandPalette) -> Vec<CommandLauncherEvent> {
    match highlighted_row(palette) {
        Some(row) if !row.disabled => {
            vec![CommandLauncherEvent::ResultExecuted { id: row.id.clone() }]
        }
        Some(_) | None => Vec::new(),
    }
}

fn execute_id(palette: &CommandPalette, id: &str) -> Vec<CommandLauncherEvent> {
    match palette
        .model
        .command_result_rows
        .iter()
        .find(|row| row.id == id)
    {
        Some(row) if !row.disabled => {
            vec![CommandLauncherEvent::ResultExecuted { id: row.id.clone() }]
        }
        Some(_) | None => Vec::new(),
    }
}

fn first_enabled_index(rows: &[CommandResultRow], query: &str) -> Option<usize> {
    let normalized = query.to_ascii_lowercase();
    rows.iter()
        .position(|row| {
            !row.disabled
                && !normalized.is_empty()
                && (row.id.to_ascii_lowercase().contains(&normalized)
                    || row.label.to_ascii_lowercase().contains(&normalized))
        })
        .or_else(|| rows.iter().position(|row| !row.disabled))
}

fn moved_index(current: Option<usize>, len: usize, value: HighlightMove) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let current = current.unwrap_or_default().min(len - 1);
    match value {
        HighlightMove::Previous => Some(current.saturating_sub(1)),
        HighlightMove::Next => Some(current.saturating_add(1).min(len - 1)),
        HighlightMove::First => Some(0),
        HighlightMove::Last => Some(len - 1),
    }
}

fn set_highlight(palette: &mut CommandPalette, value: Option<usize>) {
    palette.model.command_highlighted_index = value;
    palette.state.has_selection = value.is_some();
    palette.state.selected_index = value.unwrap_or_default();
    if let Some(config) = &mut palette.model.command_virtualization {
        config.focused_index = value;
        config.total_count = palette.model.command_result_rows.len();
        config.keep_focused_in_window = true;
    }
}

fn highlighted_event(palette: &CommandPalette) -> CommandLauncherEvent {
    CommandLauncherEvent::ResultHighlighted {
        index: palette.model.command_highlighted_index,
        id: highlighted_row(palette).map(|row| row.id.clone()),
    }
}

fn highlighted_row(palette: &CommandPalette) -> Option<&CommandResultRow> {
    let index = palette.model.command_highlighted_index?;
    palette.model.command_result_rows.get(index)
}
