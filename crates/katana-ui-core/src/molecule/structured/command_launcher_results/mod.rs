mod actions;
mod render;
mod row;

pub use actions::{CommandKeyboardInput, CommandLauncherAction, CommandLauncherEvent};
pub use row::{CommandResultRow, CommandResultRows, HighlightMove};

use super::model::CommandPalette;
use crate::interaction::{VirtualRange, VirtualizationConfig};

impl CommandPalette {
    #[must_use]
    pub fn result_row(mut self, row: CommandResultRow) -> Self {
        self.model.command_result_rows.push(row);
        self.state.item_count = self.model.command_result_rows.len();
        self
    }

    #[must_use]
    pub fn highlighted_index(mut self, value: Option<usize>) -> Self {
        self.model.command_highlighted_index = value;
        self.state.has_selection = value.is_some();
        self.state.selected_index = value.unwrap_or_default();
        self
    }

    #[must_use]
    pub fn virtualization(mut self, value: VirtualizationConfig) -> Self {
        self.model.command_virtualization = Some(value);
        self
    }

    #[must_use]
    pub fn result_rows_model(&self) -> &[CommandResultRow] {
        &self.model.command_result_rows
    }

    #[must_use]
    pub fn command_highlighted_index_model(&self) -> Option<usize> {
        self.model.command_highlighted_index
    }

    #[must_use]
    pub fn command_virtual_range_model(&self) -> Option<VirtualRange> {
        row::virtual_range(
            &self.model.command_result_rows,
            self.model.command_highlighted_index,
            &self.model.command_virtualization,
        )
    }

    pub fn apply_launcher_action(
        &mut self,
        action: CommandLauncherAction,
    ) -> Vec<CommandLauncherEvent> {
        actions::apply(self, action)
    }
}

pub(crate) use render::CommandPaletteRenderer;
