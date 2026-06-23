use super::{
    ShortcutCheatsheetAction, ShortcutCheatsheetGroup, ShortcutCheatsheetItem, StoryCatalog,
    StoryExample, UiCallbackLog, UiStateId, UiTone, atom, molecule,
};

pub(super) fn shortcut_combo_story() -> StoryExample {
    let combo = atom::ShortcutCombo::new("Open command palette", command_combo('k'))
        .platform_display(atom::ShortcutPlatform::MacOS)
        .separator(atom::ShortcutSeparator::None)
        .size(katana_ui_core::render_model::UiSize::Medium)
        .tone(UiTone::Accent)
        .accessibility_label("Open command palette shortcut");
    let logs = vec![UiCallbackLog::new(
        UiStateId::new("state:ShortcutCombo:storybook"),
        "shortcut_platform_preview",
        "platform=Auto",
        "platform=MacOS combo=Command+K",
    )];
    StoryCatalog::interactive_story("shortcut-combo", combo, logs)
}

pub(super) fn shortcut_cheatsheet_story() -> StoryExample {
    let mut cheatsheet = molecule::ShortcutCheatsheet::new("Shortcut cheatsheet")
        .group_layout(molecule::ShortcutCheatsheetLayout::TwoColumn)
        .group(shortcut_group(
            "Navigation",
            "command-palette",
            "Command palette",
            'k',
        ))
        .group(shortcut_group("Editing", "format", "Format document", 'f'))
        .query("format");
    let query = cheatsheet.apply_action(ShortcutCheatsheetAction::SetQuery("format".to_string()));
    let selected = cheatsheet.apply_action(ShortcutCheatsheetAction::SelectShortcut(
        "format".to_string(),
    ));
    let logs = vec![UiCallbackLog::new(
        UiStateId::new("state:ShortcutCheatsheet:storybook"),
        "shortcut_filter_select",
        "query=none selected=false",
        format!("query={query:?} selected={selected:?}"),
    )];
    StoryCatalog::interactive_story("shortcut-cheatsheet", cheatsheet, logs)
}

fn shortcut_group(title: &str, id: &str, label: &str, key: char) -> ShortcutCheatsheetGroup {
    ShortcutCheatsheetGroup::new(title).item(ShortcutCheatsheetItem::new(
        id,
        label,
        command_combo(key),
    ))
}

fn command_combo(key: char) -> atom::KeyCombo {
    atom::KeyCombo::new(
        atom::KeyModifiers {
            command: true,
            control: false,
            alt: false,
            shift: false,
            meta: false,
        },
        atom::KeyKind::Char(key),
    )
}
