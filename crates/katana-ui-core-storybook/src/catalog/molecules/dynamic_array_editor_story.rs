use crate::catalog::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiActionSource, UiCallbackLog};
use katana_ui_core::molecule::ArrayEditorItem;
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::{atom, molecule};

const INITIAL_ROWS: usize = 3;

pub(super) fn story() -> StoryExample {
    let mut editor = editor();
    let target = editor.state_id().clone();
    let mut logs = editor.apply_action(&set_rows_action(&target, INITIAL_ROWS + 1));
    logs.callback_log.push(event_log(
        &target,
        "array_add",
        "rows=3 order=1,2,3",
        "event=array_changed rows=4 order=1,2,3,4 callback=add",
    ));
    let mut callback_logs = logs.callback_log;
    callback_logs.extend(remove_logs(&mut editor, &target));
    callback_logs.extend(reorder_logs(&mut editor, &target));
    StoryCatalog::interactive_story("dynamic-array-editor", editor, callback_logs)
}

fn editor() -> molecule::DynamicArrayEditor {
    molecule::DynamicArrayEditor::new("Dynamic array")
        .add_action("array_add")
        .delete_action("array_remove")
        .reorder_action("array_reorder")
        .edit_action("array_edit")
        .empty_state("No rows")
        .item(item("row-1", "Row 1", "alpha", true))
        .item(item("row-2", "Row 2", "beta", true))
        .item(item("row-3", "Row 3", "gamma", false))
        .item_count(INITIAL_ROWS)
        .child(atom::Button::new("Add row"))
        .child(atom::Button::new("Remove row"))
        .child(atom::Button::new("Move row"))
        .child(atom::Text::new("rows=3 order=1,2,3"))
}

fn item(id: &str, label: &str, value: &str, removable: bool) -> ArrayEditorItem {
    let mut item = ArrayEditorItem::new(id, label);
    item.value = value.to_string();
    item.removable = removable;
    item
}

fn remove_logs(
    editor: &mut molecule::DynamicArrayEditor,
    target: &UiStateId,
) -> Vec<UiCallbackLog> {
    let mut result = editor.apply_action(&set_rows_action(target, INITIAL_ROWS - 1));
    result.callback_log.push(event_log(
        target,
        "array_remove",
        "rows=3 order=1,2,3",
        "event=array_changed rows=2 order=1,3 callback=remove",
    ));
    result.callback_log
}

fn reorder_logs(
    editor: &mut molecule::DynamicArrayEditor,
    target: &UiStateId,
) -> Vec<UiCallbackLog> {
    let mut result = editor.apply_action(&UiAction::SetSelectedIndex {
        target: target.clone(),
        selected_index: 1,
        selected: true,
        source: UiActionSource::Generic,
    });
    result.callback_log.push(event_log(
        target,
        "array_reorder",
        "rows=3 order=1,2,3",
        "event=array_changed rows=3 order=2,1,3 callback=reorder",
    ));
    result.callback_log
}

fn set_rows_action(target: &UiStateId, rows: usize) -> UiAction {
    UiAction::SetValue {
        target: target.clone(),
        value: format!("rows={rows}"),
        source: UiActionSource::Generic,
        progress: None,
        color_drag: None,
    }
}

fn event_log(
    target: &UiStateId,
    action: &str,
    before: impl Into<String>,
    after: impl Into<String>,
) -> UiCallbackLog {
    UiCallbackLog::new(target.clone(), action, before, after)
}
