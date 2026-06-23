use crate::atom::ShortcutCombo;
use crate::interaction::{
    RowHeightProvider, VirtualRange, VirtualRow, VirtualizationConfig, VirtualizationPlanner,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HighlightMove {
    Previous,
    Next,
    First,
    Last,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandResultRow {
    pub id: String,
    pub label: String,
    pub secondary_label: Option<String>,
    pub icon: Option<String>,
    pub shortcut: Option<ShortcutCombo>,
    pub provider_id: Option<String>,
    pub group_id: Option<String>,
    pub disabled: bool,
    pub disabled_reason: Option<String>,
}

pub type CommandResultRows = Vec<CommandResultRow>;

impl CommandResultRow {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            secondary_label: None,
            icon: None,
            shortcut: None,
            provider_id: None,
            group_id: None,
            disabled: false,
            disabled_reason: None,
        }
    }

    #[must_use]
    pub fn secondary_label(mut self, value: impl Into<String>) -> Self {
        self.secondary_label = Some(value.into());
        self
    }

    #[must_use]
    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(value.into());
        self
    }

    #[must_use]
    pub fn shortcut(mut self, value: ShortcutCombo) -> Self {
        self.shortcut = Some(value);
        self
    }

    #[must_use]
    pub fn provider_id(mut self, value: impl Into<String>) -> Self {
        self.provider_id = Some(value.into());
        self
    }

    #[must_use]
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    #[must_use]
    pub fn disabled(mut self, reason: impl Into<String>) -> Self {
        self.disabled = true;
        self.disabled_reason = Some(reason.into());
        self
    }
}

pub(super) fn virtual_range(
    rows: &[CommandResultRow],
    highlighted_index: Option<usize>,
    config: &Option<VirtualizationConfig>,
) -> Option<VirtualRange> {
    let mut config = config.clone()?;
    config.total_count = rows.len();
    config.focused_index = highlighted_index;
    config.keep_focused_in_window = highlighted_index.is_some();
    let mut range = VirtualizationPlanner::compute_visible_range(&config);
    include_highlight(&mut range, highlighted_index, &config);
    Some(range)
}

fn include_highlight(
    range: &mut VirtualRange,
    highlighted_index: Option<usize>,
    config: &VirtualizationConfig,
) {
    let Some(index) = highlighted_index else {
        return;
    };
    if (range.start..range.end).contains(&index) {
        return;
    }
    let visible_count = visible_count(config).max(1);
    range.start = index;
    range.end = index.saturating_add(visible_count).min(config.total_count);
    range.rows = (range.start..range.end)
        .map(|row_index| VirtualRow {
            index: row_index,
            aria_pos_in_set: row_index + 1,
        })
        .collect();
    range.focused_row = None;
}

fn visible_count(config: &VirtualizationConfig) -> usize {
    let row_height = match &config.row_height_provider {
        RowHeightProvider::Fixed { height } => *height,
        RowHeightProvider::Variable {
            fallback_height, ..
        } => *fallback_height,
        RowHeightProvider::Estimated {
            estimated_height, ..
        } => *estimated_height,
    };
    let row_height = row_height.max(1);
    usize::try_from(config.viewport_height / row_height).unwrap_or(1) + config.overscan
}
