use super::screen_state_tree_ids::tree_static_id;
use super::{
    EmptyState, EmptyStateAction, EmptyStateActionId, EmptyStateEvent, FileTree, FileTreeAction,
    FileTreeHitTestInput, FileTreeItem, FileTreeState, StorybookScreenState, UiAction,
};

impl StorybookScreenState {
    pub(in crate::visual) fn register_empty_state_primary_action(&mut self) {
        self.action_count += 1;
        assert!(
            matches!(
                empty_state().apply_action(EmptyStateActionId::Primary),
                Some(EmptyStateEvent::Actioned { id: EmptyStateActionId::Primary, action_id })
                    if action_id == "reload"
            ),
            "core EmptyState primary action must emit a typed Actioned event"
        );
        self.last_action = "empty_state_primary";
        self.last_event = "empty_state_actioned";
        self.last_setting = "empty_state.primary_action";
        self.last_setting_value = "reload";
        self.state_label = "action=reload";
    }

    pub(in crate::visual) fn register_empty_state_hover(&mut self) {
        self.action_count += 1;
        self.preview_hovered = true;
        self.last_action = "empty_state_hover";
        self.last_event = "hover_start";
        self.last_setting = "empty_state.hover";
        self.last_setting_value = "hover";
        self.state_label = "hover=primary";
    }

    pub(in crate::visual) fn register_empty_state_focus(&mut self) {
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "empty_state_focus";
        self.last_event = "focus";
        self.last_setting = "empty_state.focus";
        self.last_setting_value = "focus";
        self.state_label = "focus=primary";
    }

    pub(in crate::visual) fn register_empty_state_keyboard_action(&mut self) {
        if !self.button_focused {
            self.last_action = "empty_state_keyboard_without_focus";
            self.last_event = "empty_state_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        assert!(
            matches!(
                empty_state().apply_action(EmptyStateActionId::Primary),
                Some(EmptyStateEvent::Actioned { id: EmptyStateActionId::Primary, action_id })
                    if action_id == "reload"
            ),
            "core EmptyState keyboard action must emit the primary Actioned event"
        );
        self.last_action = "empty_state_keyboard_primary";
        self.last_event = "empty_state_actioned";
        self.last_setting = "empty_state.keyboard";
        self.last_setting_value = "reload";
        self.state_label = "keyboard=reload";
    }

    pub(in crate::visual) fn register_tree_view_hover(&mut self) {
        self.action_count += 1;
        let mut file_tree_state = FileTreeState::default();
        file_tree_state.set_hovered_item(Some(TREE_FILE_ID.to_string()));
        let tree = FileTree::render_with_state_and_offset(
            &tree_items(),
            TREE_FILE_ID,
            TREE_VIEWPORT_WIDTH,
            TREE_VIEWPORT_HEIGHT,
            0,
            &file_tree_state,
        );
        let tree_view = &tree.root().children()[0];
        assert_eq!(TREE_FILE_ID, tree_view.props().tree.hovered_id);
        self.preview_hovered = true;
        self.last_action = "tree_hover_item";
        self.last_event = "hover_start";
        self.last_setting = "tree.hover";
        self.last_setting_value = TREE_FILE_ID;
        self.state_label = "hover=katana/a.md";
    }

    pub(in crate::visual) fn register_tree_view_focus(&mut self) {
        self.action_count += 1;
        let Some(target) = FileTree::hit_target_for_item_with_state(
            &tree_items(),
            &FileTreeState::default(),
            TREE_FILE_ID,
            0,
            TREE_VIEWPORT_WIDTH,
        ) else {
            self.last_action = "tree_focus_miss";
            self.last_event = "tree_focus_ignored";
            self.state_label = "focus=miss";
            return;
        };
        assert_eq!(TREE_FILE_ID, target.item_id);
        self.button_focused = true;
        self.tree_view_focused_id = TREE_FILE_ID;
        self.last_action = "tree_focus_item";
        self.last_event = "tree_item_focused";
        self.last_setting = "tree.focus";
        self.last_setting_value = TREE_FILE_ID;
        self.state_label = "focus=katana/a.md";
    }

    pub(in crate::visual) fn register_tree_view_keyboard_select(&mut self) {
        if !self.button_focused {
            self.last_action = "tree_keyboard_without_focus";
            self.last_event = "tree_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let actions = [UiAction::set_value(
            "tree-view".into(),
            TREE_FILE_ID.to_string(),
        )];
        assert_eq!(Some(TREE_FILE_ID), FileTree::selected_item_id(&actions));
        self.tree_view_selected_id = TREE_FILE_ID;
        self.tree_view_focused_id = "";
        self.last_action = "tree_keyboard_select";
        self.last_event = "tree_selected";
        self.last_setting = "tree.keyboard";
        self.last_setting_value = TREE_FILE_ID;
        self.state_label = "selected=katana/a.md";
    }

    pub(in crate::visual) fn register_tree_view_pointer_click(
        &mut self,
        pointer_x: usize,
        pointer_y: usize,
    ) {
        let Some(target) = FileTree::hit_target_with_state(
            &tree_items(),
            &FileTreeState::default(),
            FileTreeHitTestInput {
                pointer_x: u32::try_from(pointer_x).unwrap_or(u32::MAX),
                pointer_y: tree_core_pointer_y(pointer_y, self.tree_view_scroll_offset),
                scroll_offset_y: self.tree_view_scroll_offset,
            },
            TREE_VIEWPORT_WIDTH,
        ) else {
            self.last_action = "tree_click_miss";
            self.last_event = "tree_click_ignored";
            self.state_label = "tree=miss";
            return;
        };
        self.action_count += 1;
        match target.action {
            FileTreeAction::SelectFile { file_id } => {
                self.tree_view_selected_id = tree_static_id(&file_id);
                self.last_action = "tree_select_file";
                self.last_event = "tree_selected";
                self.last_setting = "tree.selected";
                self.last_setting_value = self.tree_view_selected_id;
                self.state_label = "selected=katana/nested/b.md";
            }
            FileTreeAction::ToggleDirectory { directory_id } => {
                self.last_action = "tree_click_toggle";
                self.last_event = "tree_toggled";
                self.last_setting = "tree.toggle";
                self.last_setting_value = tree_static_id(&directory_id);
                self.state_label = "open=false";
            }
            FileTreeAction::FocusItem { item_id } => {
                self.last_action = "tree_focus_item";
                self.last_event = "tree_item_focused";
                self.last_setting = "tree.focus";
                self.last_setting_value = tree_static_id(&item_id);
                self.tree_view_focused_id = self.last_setting_value;
                self.state_label = "focus=katana/a.md";
            }
            FileTreeAction::None => {
                self.last_action = "tree_click_miss";
                self.last_event = "tree_click_ignored";
                self.state_label = "tree=miss";
            }
        }
    }

    pub(in crate::visual) fn register_tree_view_scroll_retention(&mut self) {
        self.action_count += 1;
        self.tree_view_scroll_offset = TREE_SCROLL_OFFSET;
        let Some(target) = FileTree::hit_target_with_state(
            &tree_items(),
            &FileTreeState::default(),
            FileTreeHitTestInput {
                pointer_x: TREE_ROW_TEXT_HIT_X,
                pointer_y: 1,
                scroll_offset_y: TREE_SCROLL_OFFSET,
            },
            TREE_VIEWPORT_WIDTH,
        ) else {
            self.last_action = "tree_scroll_miss";
            self.last_event = "tree_scroll_ignored";
            self.state_label = "scroll=miss";
            return;
        };
        assert_eq!(
            FileTreeAction::SelectFile {
                file_id: TREE_NESTED_FILE_ID.to_string(),
            },
            target.action
        );
        assert_eq!(0, target.rect.y);
        self.last_action = "tree_scroll_retained";
        self.last_event = "tree_scroll_offset_kept";
        self.last_setting = "tree.scroll";
        self.last_setting_value = "96";
        self.state_label = "scroll=retained";
    }

    pub(in crate::visual) fn scroll_tree_view(&mut self, delta_y: f32) -> bool {
        let before = self.tree_view_scroll_offset;
        let step = crate::visual::layout_metrics::SCROLL_STEP as u32;
        self.tree_view_scroll_offset = if delta_y < 0.0 {
            self.tree_view_scroll_offset
                .saturating_add(step)
                .min(TREE_SCROLL_OFFSET)
        } else {
            self.tree_view_scroll_offset.saturating_sub(step)
        };
        if self.tree_view_scroll_offset == before {
            return false;
        }
        self.action_count += 1;
        let Some(target) = FileTree::hit_target_with_state(
            &tree_items(),
            &FileTreeState::default(),
            FileTreeHitTestInput {
                pointer_x: TREE_ROW_TEXT_HIT_X,
                pointer_y: 1,
                scroll_offset_y: self.tree_view_scroll_offset,
            },
            TREE_VIEWPORT_WIDTH,
        ) else {
            self.last_action = "tree_scroll_miss";
            self.last_event = "tree_scroll_ignored";
            self.state_label = "scroll=miss";
            return true;
        };
        assert_ne!(FileTreeAction::None, target.action);
        self.last_action = "tree_scroll_retained";
        self.last_event = "tree_scroll_offset_kept";
        self.last_setting = "tree.scroll";
        self.last_setting_value = "wheel";
        self.state_label = "scroll=retained";
        true
    }
}

fn empty_state() -> EmptyState {
    EmptyState::new("No diagnostics")
        .body("日本語 mixed text")
        .primary_action(EmptyStateAction::new("reload", "Reload"))
        .secondary_action(EmptyStateAction::new("docs", "Open docs"))
}

const TREE_FILE_ID: &str = "katana/a.md";
const TREE_NESTED_FILE_ID: &str = "katana/nested/b.md";
const TREE_ROW_TEXT_HIT_X: u32 = 24;
const TREE_VIEWPORT_WIDTH: u32 = 240;
const TREE_VIEWPORT_HEIGHT: u32 = 120;
const TREE_SCROLL_OFFSET: u32 = 96;

fn tree_items() -> Vec<FileTreeItem> {
    vec![
        FileTreeItem::new(TREE_FILE_ID, TREE_FILE_ID),
        FileTreeItem::new(TREE_NESTED_FILE_ID, TREE_NESTED_FILE_ID).icon("markdown"),
    ]
}

fn tree_core_pointer_y(visual_pointer_y: usize, scroll_offset_y: u32) -> u32 {
    let visual_row = visual_pointer_y.saturating_sub(crate::visual::dedicated_dod_metrics::PX_6)
        / crate::visual::dedicated_dod_molecule_tree_parts::ROW_HEIGHT;
    let row_offset =
        (visual_row as u32).saturating_mul(katana_ui_core::molecule::TreeView::row_height());
    if scroll_offset_y == 0 {
        return katana_ui_core::molecule::TreeView::row_height()
            .saturating_add(1)
            .saturating_add(row_offset);
    }
    row_offset.saturating_add(1)
}
