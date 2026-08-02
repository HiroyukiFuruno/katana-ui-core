use super::StorybookWindowState;
use crate::visual::dedicated_dynamic_array_editor;
use crate::visual::preview_detail;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{
    ArrayEditorItem, DynamicArrayEditor, DynamicArrayEditorAction as CoreDynamicArrayEditorAction,
    DynamicArrayEditorEvent,
};

const DEFAULT_ITEM_COUNT: usize = 3;
const FIRST_ROW_INDEX: usize = 0;
const SECOND_ROW_INDEX: usize = 1;
#[cfg(test)]
const VISIBLE_ROW_COUNT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum DynamicArrayEditorAction {
    Add,
    Remove,
    Reorder,
    Hover,
    Focus,
    KeyboardEdit,
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::visual) struct DynamicArrayEditorScreenState {
    editor: DynamicArrayEditor,
    callback_event: &'static str,
}

impl Default for DynamicArrayEditorScreenState {
    fn default() -> Self {
        Self {
            editor: default_editor(),
            callback_event: "callback=idle",
        }
    }
}

impl DynamicArrayEditorScreenState {
    pub(in crate::visual) fn apply_action(
        &mut self,
        action: DynamicArrayEditorAction,
    ) -> DynamicArrayUpdate {
        match action {
            DynamicArrayEditorAction::Add => self.add_item(),
            DynamicArrayEditorAction::Remove => self.remove_item(),
            DynamicArrayEditorAction::Reorder => self.reorder_items(),
            DynamicArrayEditorAction::Hover => self.hover_editor(),
            DynamicArrayEditorAction::Focus => self.focus_editor(),
            DynamicArrayEditorAction::KeyboardEdit => self.keyboard_edit_item(),
        }
    }

    pub(in crate::visual) fn apply_option(&mut self, setting: &str) -> DynamicArrayUpdate {
        match setting {
            "array.rows" => {
                self.editor = default_editor();
                self.callback_event = "callback=rows";
                DynamicArrayUpdate::new("array_rows_option", "array_changed", "array.rows=3")
            }
            "array.add_remove" => {
                self.add_core_row();
                self.callback_event = "callback=add_remove";
                DynamicArrayUpdate::new("array_add_remove_option", "array_changed", "array.rows=4")
            }
            "array.reorder" => {
                self.reorder_core_rows();
                self.callback_event = "callback=reorder";
                DynamicArrayUpdate::new(
                    "array_reorder_option",
                    "array_changed",
                    "array.order=2,1,3",
                )
            }
            "array.theme_row" => {
                let _ = self.validate_items();
                self.callback_event = "callback=theme";
                DynamicArrayUpdate::new(
                    "array_theme_row_option",
                    "array_callback",
                    "array.theme_row=accent",
                )
            }
            _ => DynamicArrayUpdate::new("array_option_unknown", "array_ignored", "array.rows=3"),
        }
    }

    #[cfg(test)]
    pub(in crate::visual) fn item_count(&self) -> usize {
        self.editor.items().len()
    }

    pub(in crate::visual) fn row_label(&self, row: usize) -> &'static str {
        match self
            .editor
            .items()
            .get(row)
            .map(|item| item.id.as_str())
            .unwrap_or_default()
        {
            "row-1" => "Item 1",
            "row-2" => "Item 2",
            "row-3" => "Item 3",
            "row-4" => "Item 4",
            _ => "Item",
        }
    }

    pub(in crate::visual) fn callback_event(&self) -> &'static str {
        self.callback_event
    }

    #[cfg(test)]
    pub(in crate::visual) fn order_label(&self) -> &'static str {
        let ids = self
            .editor
            .items()
            .iter()
            .take(VISIBLE_ROW_COUNT)
            .map(|item| item.id.as_str())
            .collect::<Vec<_>>();
        if ids == ["row-1", "row-2", "row-3"] {
            return "order=1,2,3";
        }
        if ids == ["row-2", "row-1", "row-3"] || ids == ["row-2", "row-1"] {
            return "order=2,1,3";
        }
        "order=changed"
    }

    fn add_item(&mut self) -> DynamicArrayUpdate {
        self.add_core_row();
        self.callback_event = "callback=add";
        DynamicArrayUpdate::new("array_add", "array_changed", "rows=4")
    }

    fn remove_item(&mut self) -> DynamicArrayUpdate {
        let events = self
            .editor
            .apply_array_action(CoreDynamicArrayEditorAction::RemoveItem(
                "row-3".to_string(),
            ));
        assert_removed(&events, "row-3");
        self.callback_event = "callback=remove";
        DynamicArrayUpdate::new("array_remove", "array_changed", "rows=2")
    }

    fn reorder_items(&mut self) -> DynamicArrayUpdate {
        self.reorder_core_rows();
        self.callback_event = "callback=reorder";
        DynamicArrayUpdate::new("array_reorder", "array_changed", "order=2,1,3")
    }

    fn hover_editor(&mut self) -> DynamicArrayUpdate {
        let result = self
            .editor
            .apply_action(&UiAction::hover(self.editor.state_id().clone(), true));
        assert!(result.handled, "core dynamic array must handle hover");
        self.callback_event = "callback=hover";
        DynamicArrayUpdate::new("array_hover", "array_hovered", "hover=true")
    }

    fn focus_editor(&mut self) -> DynamicArrayUpdate {
        let result = self
            .editor
            .apply_action(&UiAction::focus(self.editor.state_id().clone()));
        assert!(result.handled, "core dynamic array must handle focus");
        self.callback_event = "callback=focus";
        DynamicArrayUpdate::new("array_focus", "array_focused", "focus=true")
    }

    fn keyboard_edit_item(&mut self) -> DynamicArrayUpdate {
        let events = self
            .editor
            .apply_array_action(CoreDynamicArrayEditorAction::EditItem {
                id: "row-1".to_string(),
                value: "Edited".to_string(),
            });
        assert_edited(&events, "row-1");
        self.callback_event = "callback=edit";
        DynamicArrayUpdate::new("array_keyboard_edit", "array_changed", "edited=row-1")
    }

    fn validate_items(&mut self) -> DynamicArrayUpdate {
        let events = self
            .editor
            .apply_array_action(CoreDynamicArrayEditorAction::Validate);
        assert_valid(&events);
        self.callback_event = "callback=validate";
        DynamicArrayUpdate::new("array_validate", "array_validated", "valid=true")
    }

    fn add_core_row(&mut self) {
        if self.editor.items().len() > DEFAULT_ITEM_COUNT {
            return;
        }
        let events = self
            .editor
            .apply_array_action(CoreDynamicArrayEditorAction::AddItem(row(
                "row-4", "Item 4",
            )));
        assert_added(&events, "row-4");
    }

    fn reorder_core_rows(&mut self) {
        let events = self
            .editor
            .apply_array_action(CoreDynamicArrayEditorAction::ReorderItem {
                from: SECOND_ROW_INDEX,
                to: FIRST_ROW_INDEX,
            });
        assert_reordered(&events, "row-2");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct DynamicArrayUpdate {
    pub(in crate::visual) action: &'static str,
    pub(in crate::visual) event: &'static str,
    pub(in crate::visual) state: &'static str,
}

impl DynamicArrayUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}

fn default_editor() -> DynamicArrayEditor {
    DynamicArrayEditor::new("Rows")
        .add_action("array_add")
        .delete_action("array_remove")
        .reorder_action("array_reorder")
        .edit_action("array_edit")
        .item(row("row-1", "Item 1"))
        .item(row("row-2", "Item 2"))
        .item(row("row-3", "Item 3"))
}

fn row(id: &str, label: &str) -> ArrayEditorItem {
    ArrayEditorItem::new(id, label).value(label)
}

fn assert_added(events: &[DynamicArrayEditorEvent], id: &str) {
    assert!(
        matches!(events, [DynamicArrayEditorEvent::ItemAdded { id: actual }] if actual == id),
        "core dynamic array must add item"
    );
}

fn assert_removed(events: &[DynamicArrayEditorEvent], id: &str) {
    assert!(
        matches!(events, [DynamicArrayEditorEvent::ItemRemoved { id: actual }] if actual == id),
        "core dynamic array must remove item"
    );
}

fn assert_reordered(events: &[DynamicArrayEditorEvent], id: &str) {
    assert!(
        matches!(events, [DynamicArrayEditorEvent::ItemReordered { id: actual, .. }] if actual == id),
        "core dynamic array must reorder item"
    );
}

fn assert_edited(events: &[DynamicArrayEditorEvent], id: &str) {
    assert!(
        matches!(events, [DynamicArrayEditorEvent::ItemEdited { id: actual }] if actual == id),
        "core dynamic array must edit item"
    );
}

fn assert_valid(events: &[DynamicArrayEditorEvent]) {
    assert!(
        matches!(
            events,
            [DynamicArrayEditorEvent::ValidationChanged {
                valid: true,
                message,
            }] if message == "valid"
        ),
        "core dynamic array must validate item values"
    );
}

pub(super) fn operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<DynamicArrayEditorAction> {
    if state.selected_page != "dynamic-array-editor" {
        return None;
    }
    let base = preview_detail::component_action_hit_rect(state.selected_page);
    dedicated_dynamic_array_editor::action_at(base.x, base.y, x, y)
}

#[cfg(test)]
mod tests {
    use super::{DynamicArrayEditorAction, DynamicArrayEditorScreenState};

    #[test]
    fn dynamic_array_boundaries_cover_unknown_options_labels_duplicate_add_and_short_orders() {
        let mut state = DynamicArrayEditorScreenState::default();
        assert_eq!("order=1,2,3", state.order_label());
        assert_eq!("array_option_unknown", state.apply_option("unknown").action);
        assert_eq!("Item", state.row_label(99));

        state.apply_action(DynamicArrayEditorAction::Add);
        assert_eq!(4, state.item_count());
        assert_eq!("Item 4", state.row_label(3));
        state.apply_action(DynamicArrayEditorAction::Add);
        assert_eq!(4, state.item_count());

        let mut short = DynamicArrayEditorScreenState::default();
        short.apply_action(DynamicArrayEditorAction::Remove);
        assert_eq!("order=changed", short.order_label());
        short.apply_action(DynamicArrayEditorAction::Reorder);
        assert_eq!("order=2,1,3", short.order_label());
    }
}
