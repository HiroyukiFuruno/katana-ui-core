use katana_ui_core::atom::{KeyCombo, KeyKind, KeyModifiers};
use katana_ui_core::molecule::{
    ShortcutCheatsheet, ShortcutCheatsheetEvent, ShortcutCheatsheetGroup, ShortcutCheatsheetItem,
};

pub(in crate::visual) const DEFAULT_GROUP_COUNT: usize = 2;
pub(in crate::visual) const EXPANDED_GROUP_COUNT: usize = 3;
pub(in crate::visual) const DEFAULT_ITEM_COUNT: usize = 2;
pub(in crate::visual) const EXPANDED_ITEM_COUNT: usize = 4;
pub(in crate::visual) const FILTERED_RESULT_COUNT: usize = 1;
pub(in crate::visual) const QUERY_CATEGORY: &str = "カテゴリ";
pub(in crate::visual) const FORMAT_ID: &str = "format";

pub(in crate::visual) fn assert_query_event(event: Option<ShortcutCheatsheetEvent>) {
    assert!(
        matches!(event, Some(ShortcutCheatsheetEvent::QueryChanged(value)) if value == QUERY_CATEGORY),
        "core shortcut cheatsheet must emit query change"
    );
}

pub(in crate::visual) fn assert_selected_event(event: Option<ShortcutCheatsheetEvent>) {
    assert!(
        matches!(event, Some(ShortcutCheatsheetEvent::ShortcutSelected { id, .. }) if id == FORMAT_ID),
        "core shortcut cheatsheet must select the requested shortcut"
    );
}

pub(in crate::visual) fn default_cheatsheet() -> ShortcutCheatsheet {
    ShortcutCheatsheet::new("Shortcut cheatsheet")
        .group(default_group("Editing"))
        .group(default_group("Navigation"))
}

pub(in crate::visual) fn cheatsheet_with_group_count(count: usize) -> ShortcutCheatsheet {
    let mut cheatsheet = ShortcutCheatsheet::new("Shortcut cheatsheet");
    for index in 0..count {
        cheatsheet = cheatsheet.group(default_group(format!("Group {index}")));
    }
    cheatsheet
}

pub(in crate::visual) fn cheatsheet_with_item_count(count: usize) -> ShortcutCheatsheet {
    let mut group = ShortcutCheatsheetGroup::new("Editing");
    for index in 0..count {
        group = group.item(shortcut_item(
            format!("item-{index}"),
            format!("Item {index}"),
        ));
    }
    ShortcutCheatsheet::new("Shortcut cheatsheet").group(group)
}

fn default_group(title: impl Into<String>) -> ShortcutCheatsheetGroup {
    let title = title.into();
    if title == "Editing" {
        return ShortcutCheatsheetGroup::new(title)
            .item(shortcut_item("format", "カテゴリ format"));
    }
    ShortcutCheatsheetGroup::new(title).item(shortcut_item("open", "Open command"))
}

fn shortcut_item(id: impl Into<String>, label: impl Into<String>) -> ShortcutCheatsheetItem {
    ShortcutCheatsheetItem::new(
        id,
        label,
        KeyCombo::new(KeyModifiers::command_shift(), KeyKind::Char('P')),
    )
}
