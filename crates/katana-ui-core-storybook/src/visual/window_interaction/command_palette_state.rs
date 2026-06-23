use katana_ui_core::molecule::{
    CommandKeyboardInput, CommandLauncherAction, CommandLauncherEvent, CommandPalette,
    CommandResultRow, HighlightMove,
};
use katana_ui_core::{
    component::ComponentAction,
    interaction::{UiAction, UiActionResult},
};

const DEFAULT_ROW_COUNT: usize = 5;
const EXPANDED_ROW_COUNT: usize = 50;
const THEME_QUERY: &str = "theme";
const THEME_ROW_INDEX: usize = 2;
const HIGHLIGHT_MOVE_COUNT: usize = THEME_ROW_INDEX;

#[derive(Debug, Clone, PartialEq)]
pub(in crate::visual) struct CommandPaletteScreenState {
    palette: CommandPalette,
    option_state: CommandPaletteOptionState,
    callback_action: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct CommandPaletteOptionState {
    pub(in crate::visual) row_count: usize,
    pub(in crate::visual) provider_group_workspace_editor_app: bool,
    pub(in crate::visual) shortcut_display_visible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum CommandPaletteStoryAction {
    Hover,
    Focus,
    KeyboardExecute,
    KeyboardClose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct CommandPaletteUpdate {
    pub(in crate::visual) action: &'static str,
    pub(in crate::visual) event: &'static str,
    pub(in crate::visual) state: &'static str,
}

impl Default for CommandPaletteOptionState {
    fn default() -> Self {
        Self {
            row_count: DEFAULT_ROW_COUNT,
            provider_group_workspace_editor_app: false,
            shortcut_display_visible: true,
        }
    }
}

impl Default for CommandPaletteScreenState {
    fn default() -> Self {
        Self {
            palette: command_palette_with_rows(DEFAULT_ROW_COUNT),
            option_state: CommandPaletteOptionState::default(),
            callback_action: "none",
        }
    }
}

impl CommandPaletteScreenState {
    pub(in crate::visual) fn apply_action(
        &mut self,
        action: CommandPaletteStoryAction,
    ) -> CommandPaletteUpdate {
        match action {
            CommandPaletteStoryAction::Hover => self.apply_hover(),
            CommandPaletteStoryAction::Focus => self.apply_focus(),
            CommandPaletteStoryAction::KeyboardExecute => self.apply_keyboard_execute(),
            CommandPaletteStoryAction::KeyboardClose => self.apply_keyboard_close(),
        }
    }

    pub(in crate::visual) fn apply_option(&mut self, setting: &str) {
        match setting {
            "command_palette.query" => self.apply_query(),
            "command_palette.highlight" => self.apply_highlight(),
            "command_palette.row_count" => self.apply_row_count(),
            "command_palette.provider_group" => self.apply_provider_group(),
            "command_palette.shortcut_display" => self.apply_shortcut_display(),
            _ => {}
        }
    }

    #[cfg(test)]
    pub(in crate::visual) fn query(&self) -> &str {
        self.palette.query_model()
    }

    #[cfg(test)]
    pub(in crate::visual) fn highlighted_index(&self) -> Option<usize> {
        self.palette.command_highlighted_index_model()
    }

    #[cfg(test)]
    pub(in crate::visual) fn row_count(&self) -> usize {
        self.palette.result_rows_model().len()
    }

    #[cfg(test)]
    pub(in crate::visual) const fn option_state(&self) -> CommandPaletteOptionState {
        self.option_state
    }

    pub(in crate::visual) const fn callback_action(&self) -> &'static str {
        self.callback_action
    }

    fn apply_query(&mut self) {
        let events = self
            .palette
            .apply_launcher_action(CommandLauncherAction::SetQuery(THEME_QUERY.to_string()));
        assert_query_events(&events);
        self.callback_action = "command_palette_query";
    }

    fn apply_highlight(&mut self) {
        let mut events = Vec::new();
        for _ in 0..HIGHLIGHT_MOVE_COUNT {
            events = self
                .palette
                .apply_launcher_action(CommandLauncherAction::MoveHighlight(HighlightMove::Next));
        }
        assert_highlight_event(&events);
        self.callback_action = "command_palette_highlight";
    }

    fn apply_row_count(&mut self) {
        self.palette = command_palette_with_rows(EXPANDED_ROW_COUNT);
        self.option_state.row_count = EXPANDED_ROW_COUNT;
        self.callback_action = "command_palette_row_count";
    }

    fn apply_provider_group(&mut self) {
        self.palette = command_palette_with_provider_groups();
        self.option_state.provider_group_workspace_editor_app = true;
        self.callback_action = "command_palette_provider_group";
    }

    fn apply_shortcut_display(&mut self) {
        self.option_state.shortcut_display_visible = false;
        self.callback_action = "command_palette_shortcut_display";
    }

    fn apply_hover(&mut self) -> CommandPaletteUpdate {
        let result = self.apply_core_action(UiAction::hover(self.palette.state_id().clone(), true));
        assert!(result.handled, "core command palette must handle hover");
        self.callback_action = "command_palette_hover";
        CommandPaletteUpdate::new(
            "command_palette_hover",
            "command_palette_hovered",
            "hover=true",
        )
    }

    fn apply_focus(&mut self) -> CommandPaletteUpdate {
        let result = self.apply_core_action(UiAction::focus(self.palette.state_id().clone()));
        assert!(result.handled, "core command palette must handle focus");
        self.callback_action = "command_palette_focus";
        CommandPaletteUpdate::new(
            "command_palette_focus",
            "command_palette_focused",
            "focus=true",
        )
    }

    fn apply_keyboard_execute(&mut self) -> CommandPaletteUpdate {
        let highlight = self
            .palette
            .apply_launcher_action(CommandLauncherAction::Keyboard(
                CommandKeyboardInput::ArrowDown,
            ));
        assert_highlighted_row(&highlight, 1, "format");
        let events = self
            .palette
            .apply_launcher_action(CommandLauncherAction::Keyboard(CommandKeyboardInput::Enter));
        assert_executed_row(&events, "format");
        self.callback_action = "command_palette_keyboard_execute";
        CommandPaletteUpdate::new(
            "command_palette_keyboard_execute",
            "command_palette_result_executed",
            "executed=format",
        )
    }

    fn apply_keyboard_close(&mut self) -> CommandPaletteUpdate {
        let events = self
            .palette
            .apply_launcher_action(CommandLauncherAction::Keyboard(
                CommandKeyboardInput::Escape,
            ));
        assert_closed(&events);
        self.callback_action = "command_palette_keyboard_close";
        CommandPaletteUpdate::new(
            "command_palette_keyboard_close",
            "command_palette_closed",
            "closed=true",
        )
    }

    fn apply_core_action(&mut self, action: UiAction) -> UiActionResult {
        self.palette.apply_action(&action)
    }
}

fn assert_query_events(events: &[CommandLauncherEvent]) {
    assert!(
        matches!(
            events,
            [
                CommandLauncherEvent::QueryChanged(value),
                CommandLauncherEvent::ResultHighlighted { index: Some(THEME_ROW_INDEX), .. }
            ] if value == THEME_QUERY
        ),
        "core command palette must update query and highlight matching result"
    );
}

fn assert_highlight_event(events: &[CommandLauncherEvent]) {
    assert!(
        matches!(
            events,
            [CommandLauncherEvent::ResultHighlighted {
                index: Some(THEME_ROW_INDEX),
                ..
            }]
        ),
        "core command palette must move highlight to the requested row"
    );
}

fn assert_highlighted_row(events: &[CommandLauncherEvent], index: usize, id: &str) {
    assert!(
        matches!(
            events,
            [CommandLauncherEvent::ResultHighlighted {
                index: Some(actual_index),
                id: Some(actual_id),
            }] if *actual_index == index && actual_id == id
        ),
        "core command palette must move highlight through keyboard"
    );
}

fn assert_executed_row(events: &[CommandLauncherEvent], id: &str) {
    assert!(
        matches!(
            events,
            [CommandLauncherEvent::ResultExecuted { id: actual_id }] if actual_id == id
        ),
        "core command palette must execute the highlighted result"
    );
}

fn assert_closed(events: &[CommandLauncherEvent]) {
    assert!(
        matches!(events, [CommandLauncherEvent::Closed]),
        "core command palette must close through keyboard escape"
    );
}

impl CommandPaletteUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}

fn command_palette_with_rows(count: usize) -> CommandPalette {
    let mut palette = CommandPalette::new("Command palette");
    for index in 0..count {
        palette = palette.result_row(command_row(index));
    }
    palette
}

fn command_palette_with_provider_groups() -> CommandPalette {
    CommandPalette::new("Command palette")
        .result_row(CommandResultRow::new("workspace", "Workspace").provider_id("workspace"))
        .result_row(CommandResultRow::new("editor", "Editor").provider_id("editor"))
        .result_row(CommandResultRow::new("app", "App").provider_id("app"))
}

fn command_row(index: usize) -> CommandResultRow {
    match index {
        0 => CommandResultRow::new("open", "Open File"),
        1 => CommandResultRow::new("format", "Format Document"),
        THEME_ROW_INDEX => CommandResultRow::new("theme", "Theme Preferences"),
        _ => CommandResultRow::new(format!("row-{index}"), format!("Command {index}")),
    }
}
