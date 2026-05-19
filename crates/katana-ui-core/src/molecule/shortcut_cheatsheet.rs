use crate::atom::shortcut_combo::{KeyCombo, ShortcutCombo};
use crate::render_model::{UiInteractionState, UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShortcutCheatsheetLayout {
    TwoColumn,
    OneColumn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortcutCheatsheetItem {
    id: String,
    label: String,
    combo: KeyCombo,
}

impl ShortcutCheatsheetItem {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>, combo: KeyCombo) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            combo,
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub const fn combo(&self) -> &KeyCombo {
        &self.combo
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortcutCheatsheetGroup {
    title: String,
    items: Vec<ShortcutCheatsheetItem>,
}

impl ShortcutCheatsheetGroup {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
        }
    }

    #[must_use]
    pub fn item(mut self, value: ShortcutCheatsheetItem) -> Self {
        self.items.push(value);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShortcutCheatsheetAction {
    SetQuery(String),
    SelectShortcut(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShortcutCheatsheetEvent {
    QueryChanged(String),
    ShortcutSelected { id: String, combo: KeyCombo },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortcutCheatsheet {
    label: String,
    state_id: UiStateId,
    groups: Vec<ShortcutCheatsheetGroup>,
    query: String,
    group_layout: ShortcutCheatsheetLayout,
    selected_id: Option<String>,
}

impl ShortcutCheatsheet {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::ShortcutCheatsheet),
            groups: Vec::new(),
            query: String::new(),
            group_layout: ShortcutCheatsheetLayout::TwoColumn,
            selected_id: None,
        }
    }

    #[must_use]
    pub fn group(mut self, value: ShortcutCheatsheetGroup) -> Self {
        self.groups.push(value);
        self
    }

    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = value.into();
        self.selected_id = None;
        self
    }

    #[must_use]
    pub fn visible_items(&self) -> Vec<&ShortcutCheatsheetItem> {
        let query = self.query.to_lowercase();
        self.groups
            .iter()
            .flat_map(|group| group.visible_items(&query))
            .collect()
    }

    #[must_use]
    pub fn apply_action(
        &mut self,
        action: ShortcutCheatsheetAction,
    ) -> Option<ShortcutCheatsheetEvent> {
        match action {
            ShortcutCheatsheetAction::SetQuery(value) => {
                self.query = value.clone();
                self.selected_id = None;
                Some(ShortcutCheatsheetEvent::QueryChanged(value))
            }
            ShortcutCheatsheetAction::SelectShortcut(id) => self.select_shortcut(&id),
        }
    }

    fn select_shortcut(&mut self, id: &str) -> Option<ShortcutCheatsheetEvent> {
        let selected = self
            .visible_items()
            .into_iter()
            .find(|item| item.id == id)
            .map(|item| (item.id.clone(), item.combo.clone()))?;
        self.selected_id = Some(id.to_string());
        Some(ShortcutCheatsheetEvent::ShortcutSelected {
            id: selected.0,
            combo: selected.1,
        })
    }
}

impl From<ShortcutCheatsheet> for UiNode {
    fn from(value: ShortcutCheatsheet) -> Self {
        let count = value.visible_items().len();
        let visible_items = value
            .visible_items()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();
        let mut node =
            UiNode::from_state(UiNodeKind::ShortcutCheatsheet, value.label, value.state_id)
                .interaction(UiInteractionState {
                    value: value.query.clone(),
                    item_count: count,
                    has_selection: value.selected_id.is_some(),
                    ..UiInteractionState::default()
                });
        for item in visible_items {
            node = node.child(ShortcutCombo::new(item.label.clone(), item.combo.clone()));
        }
        node
    }
}

impl ShortcutCheatsheetGroup {
    fn visible_items<'a>(&'a self, query: &str) -> Vec<&'a ShortcutCheatsheetItem> {
        if query.is_empty() || self.title.to_lowercase().contains(query) {
            return self.items.iter().collect();
        }
        self.items
            .iter()
            .filter(|item| item.label.to_lowercase().contains(query))
            .collect()
    }
}
