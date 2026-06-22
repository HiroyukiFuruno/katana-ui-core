use crate::render_model::UiCursor;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileTreeHitTestInput {
    pub pointer_x: u32,
    pub pointer_y: u32,
    pub scroll_offset_y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileTreeHitRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileTreeHitTarget {
    pub item_id: String,
    pub rect: FileTreeHitRect,
    pub cursor: UiCursor,
    pub action: FileTreeAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileTreeAction {
    SelectFile { file_id: String },
    ToggleDirectory { directory_id: String },
    FocusItem { item_id: String },
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileTreeItem {
    pub id: String,
    pub label: String,
    pub icon: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FileTreeState {
    pub(super) collapsed_directory_ids: BTreeSet<String>,
    pub(super) hovered_item_id: Option<String>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FileTree;

impl FileTreeItem {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: String::new(),
        }
    }

    #[must_use]
    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = value.into();
        self
    }
}

impl FileTreeState {
    #[must_use]
    pub fn collapsed(mut self, directory_id: impl Into<String>) -> Self {
        self.collapsed_directory_ids.insert(directory_id.into());
        self
    }

    #[must_use]
    pub fn is_collapsed(&self, directory_id: &str) -> bool {
        self.collapsed_directory_ids.contains(directory_id)
    }

    pub fn toggle_directory(&mut self, directory_id: impl Into<String>) {
        let directory_id = directory_id.into();
        if self.collapsed_directory_ids.remove(&directory_id) {
            return;
        }
        self.collapsed_directory_ids.insert(directory_id);
    }

    #[must_use]
    pub fn hovered(mut self, item_id: impl Into<String>) -> Self {
        self.hovered_item_id = Some(item_id.into());
        self
    }

    pub fn set_hovered_item(&mut self, item_id: Option<String>) {
        self.hovered_item_id = item_id;
    }

    #[must_use]
    pub fn hovered_item_id(&self) -> Option<&str> {
        self.hovered_item_id.as_deref()
    }
}
