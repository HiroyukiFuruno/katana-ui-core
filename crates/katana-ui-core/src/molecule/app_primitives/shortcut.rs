use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{
    UiInteractionState, UiNode, UiNodeKind, UiShortcutProps, UiStateId, UiVisualRole,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShortcutPlatform {
    MacOs,
    Windows,
    Linux,
    Plain,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortcutCombo {
    label: String,
    state_id: UiStateId,
    platform: ShortcutPlatform,
    keys: Vec<String>,
}

impl ShortcutCombo {
    #[must_use]
    pub fn new(
        label: impl Into<String>,
        keys: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::ShortcutCombo),
            platform: ShortcutPlatform::Plain,
            keys: keys.into_iter().map(Into::into).collect(),
        }
    }

    #[must_use]
    pub fn platform(mut self, platform: ShortcutPlatform) -> Self {
        self.platform = platform;
        self
    }

    #[must_use]
    pub fn display_text(&self) -> String {
        let separator = match self.platform {
            ShortcutPlatform::MacOs => "",
            ShortcutPlatform::Plain => " ",
            ShortcutPlatform::Windows | ShortcutPlatform::Linux => "+",
        };
        self.keys
            .iter()
            .map(|key| display_key(self.platform, key))
            .collect::<Vec<_>>()
            .join(separator)
    }
}

impl From<ShortcutCombo> for UiNode {
    fn from(value: ShortcutCombo) -> Self {
        let combo = value.display_text();
        UiNode::from_state(UiNodeKind::ShortcutCombo, value.label, value.state_id)
            .visual_role(UiVisualRole::Shortcut)
            .shortcut(UiShortcutProps {
                platform: format!("{:?}", value.platform),
                combo,
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortcutCheatsheetEntry {
    pub id: String,
    pub label: String,
    pub combo: ShortcutCombo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShortcutCheatsheetEvent {
    None,
    QueryChanged(String),
    ShortcutSelected(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortcutCheatsheet {
    label: String,
    state_id: UiStateId,
    entries: Vec<ShortcutCheatsheetEntry>,
    query: String,
    selected_index: usize,
    last_event: ShortcutCheatsheetEvent,
}

impl ShortcutCheatsheet {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::ShortcutCheatsheet),
            entries: Vec::new(),
            query: String::new(),
            selected_index: 0,
            last_event: ShortcutCheatsheetEvent::None,
        }
    }

    #[must_use]
    pub fn entry(mut self, entry: ShortcutCheatsheetEntry) -> Self {
        self.entries.push(entry);
        self
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn filtered_entries(&self) -> Vec<&ShortcutCheatsheetEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.label.contains(&self.query) || entry.id.contains(&self.query))
            .collect()
    }

    #[must_use]
    pub fn last_event(&self) -> &ShortcutCheatsheetEvent {
        &self.last_event
    }
}

impl ComponentAction for ShortcutCheatsheet {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = state(self);
        if action.target() != &self.state_id {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        match action {
            UiAction::SetValue { value, .. } => {
                self.query = value.clone();
                self.last_event = ShortcutCheatsheetEvent::QueryChanged(value.clone());
            }
            UiAction::SetSelectedIndex { selected_index, .. } => {
                self.selected_index = *selected_index;
                if let Some(entry) = self.filtered_entries().get(*selected_index) {
                    self.last_event = ShortcutCheatsheetEvent::ShortcutSelected(entry.id.clone());
                }
            }
            _ => return UiActionResult::ignored(self.state_id.clone(), before),
        }
        UiActionResult::handled(self.state_id.clone(), action, before, state(self))
    }
}

impl From<ShortcutCheatsheet> for UiNode {
    fn from(value: ShortcutCheatsheet) -> Self {
        let state = state(&value);
        UiNode::from_state(UiNodeKind::ShortcutCheatsheet, value.label, value.state_id)
            .interaction(state)
    }
}

fn display_key(platform: ShortcutPlatform, key: &str) -> String {
    match (platform, key) {
        (ShortcutPlatform::MacOs, "Command") => "⌘".to_string(),
        (ShortcutPlatform::MacOs, "Shift") => "⇧".to_string(),
        (ShortcutPlatform::MacOs, "Option") => "⌥".to_string(),
        (ShortcutPlatform::MacOs, "Control") => "⌃".to_string(),
        _ => key.to_string(),
    }
}

fn state(value: &ShortcutCheatsheet) -> UiInteractionState {
    UiInteractionState {
        value: value.query.clone(),
        item_count: value.filtered_entries().len(),
        selected_index: value.selected_index,
        has_selection: matches!(
            value.last_event,
            ShortcutCheatsheetEvent::ShortcutSelected(_)
        ),
        ..UiInteractionState::default()
    }
}
