use super::items::ArrayEditorItem;
use super::model::DynamicArrayEditor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DynamicArrayEditorAction {
    AddItem(ArrayEditorItem),
    RemoveItem(String),
    ReorderItem { from: usize, to: usize },
    EditItem { id: String, value: String },
    Validate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DynamicArrayEditorEvent {
    ItemAdded { id: String },
    ItemRemoved { id: String },
    ItemReordered { id: String, from: usize, to: usize },
    ItemEdited { id: String },
    ValidationChanged { valid: bool, message: String },
}

impl DynamicArrayEditor {
    pub fn apply_array_action(
        &mut self,
        action: DynamicArrayEditorAction,
    ) -> Vec<DynamicArrayEditorEvent> {
        match action {
            DynamicArrayEditorAction::AddItem(item) => self.add_array_item(item),
            DynamicArrayEditorAction::RemoveItem(id) => self.remove_array_item(&id),
            DynamicArrayEditorAction::ReorderItem { from, to } => self.reorder_array_item(from, to),
            DynamicArrayEditorAction::EditItem { id, value } => self.edit_array_item(&id, value),
            DynamicArrayEditorAction::Validate => self.validate_array_items(),
        }
    }

    fn add_array_item(&mut self, item: ArrayEditorItem) -> Vec<DynamicArrayEditorEvent> {
        let id = item.id.clone();
        self.items.push(item);
        self.sync_item_count();
        vec![DynamicArrayEditorEvent::ItemAdded { id }]
    }

    fn remove_array_item(&mut self, id: &str) -> Vec<DynamicArrayEditorEvent> {
        let Some(index) = self.items.iter().position(|item| item.id == id) else {
            return Vec::new();
        };
        if !self.items[index].removable {
            return Vec::new();
        }
        let removed = self.items.remove(index);
        self.sync_item_count();
        vec![DynamicArrayEditorEvent::ItemRemoved { id: removed.id }]
    }

    fn reorder_array_item(&mut self, from: usize, to: usize) -> Vec<DynamicArrayEditorEvent> {
        if from >= self.items.len() || to >= self.items.len() || from == to {
            return Vec::new();
        }
        let item = self.items.remove(from);
        let id = item.id.clone();
        self.items.insert(to, item);
        vec![DynamicArrayEditorEvent::ItemReordered { id, from, to }]
    }

    fn edit_array_item(&mut self, id: &str, value: String) -> Vec<DynamicArrayEditorEvent> {
        let Some(item) = self.items.iter_mut().find(|item| item.id == id) else {
            return Vec::new();
        };
        item.value = value;
        vec![DynamicArrayEditorEvent::ItemEdited { id: id.to_string() }]
    }

    fn validate_array_items(&self) -> Vec<DynamicArrayEditorEvent> {
        let valid = !self.items.is_empty() && self.items.iter().all(|item| !item.value.is_empty());
        let message = if valid {
            "valid"
        } else {
            "array item value required"
        };
        vec![DynamicArrayEditorEvent::ValidationChanged {
            valid,
            message: message.to_string(),
        }]
    }

    fn sync_item_count(&mut self) {
        self.state.item_count = self.items.len();
    }
}
