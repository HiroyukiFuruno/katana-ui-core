use katana_ui_core::atom::shortcut_combo::{
    KeyCombo, KeyKind, KeyModifiers, RuntimePlatform, ShortcutCombo, ShortcutPlatform,
    ShortcutPlatformProvider, ShortcutSeparator,
};
use katana_ui_core::molecule::shortcut_cheatsheet::{
    ShortcutCheatsheet, ShortcutCheatsheetAction, ShortcutCheatsheetEvent, ShortcutCheatsheetGroup,
    ShortcutCheatsheetItem, ShortcutCheatsheetLayout,
};
use katana_ui_core::render_model::{UiNode, UiSize, UiTone};

#[test]
fn shortcut_combo_serializes_and_renders_platform_display() {
    let combo = KeyCombo::new(KeyModifiers::command_shift(), KeyKind::Char('p'));
    let value =
        ShortcutCombo::new("Palette", combo.clone()).platform_display(ShortcutPlatform::MacOS);
    let encoded = serde_json::to_string(&combo);
    let decoded = encoded
        .ok()
        .and_then(|it| serde_json::from_str::<KeyCombo>(&it).ok());

    assert_eq!(Some(combo), decoded);
    assert_eq!("⌘⇧P", value.visual_text(RuntimePlatform::MacOS));
    assert_eq!(
        "Command + Shift + P",
        value.accessibility_text(RuntimePlatform::MacOS)
    );
}

#[test]
fn shortcut_combo_supports_separator_override_and_runtime_auto() {
    let combo = KeyCombo::new(KeyModifiers::control_shift(), KeyKind::Char('p'));
    let mac = ShortcutCombo::new("Palette", combo.clone())
        .platform_display(ShortcutPlatform::MacOS)
        .separator(ShortcutSeparator::Plus);
    let auto = ShortcutCombo::new("Palette", combo).platform_display(ShortcutPlatform::Auto);

    assert_eq!("⌃+⇧+P", mac.visual_text(RuntimePlatform::MacOS));
    assert_eq!("Ctrl+Shift+P", auto.visual_text(RuntimePlatform::Windows));
    assert_eq!("Ctrl+Shift+P", auto.visual_text(RuntimePlatform::Linux));
}

#[test]
fn shortcut_combo_uses_adapter_platform_callback_and_renders_props() {
    let provider = StaticPlatformProvider(RuntimePlatform::Windows);
    let combo = ShortcutCombo::new(
        "Palette",
        KeyCombo::new(KeyModifiers::command_shift(), KeyKind::Char('p')),
    )
    .platform_display(ShortcutPlatform::Auto)
    .separator(ShortcutSeparator::Space)
    .size(UiSize::Large)
    .tone(UiTone::Accent);
    let node = UiNode::from(combo.clone());

    assert_eq!("Ctrl Shift P", combo.visual_text_with_provider(&provider));
    assert_eq!("Command + Shift + P", node.props().accessibility_label);
    assert_eq!(UiSize::Large, node.props().size);
    assert_eq!(UiTone::Accent, node.props().tone);
}

#[test]
fn shortcut_cheatsheet_filters_groups_and_emits_selection_event() {
    let open = KeyCombo::new(KeyModifiers::command_shift(), KeyKind::Char('o'));
    let close = KeyCombo::new(KeyModifiers::command_shift(), KeyKind::Char('w'));
    let mut sheet = ShortcutCheatsheet::new("Shortcuts")
        .group(
            ShortcutCheatsheetGroup::new("File")
                .item(ShortcutCheatsheetItem::new(
                    "open",
                    "Open file",
                    open.clone(),
                ))
                .item(ShortcutCheatsheetItem::new("close", "Close file", close)),
        )
        .group(
            ShortcutCheatsheetGroup::new("Navigation").item(ShortcutCheatsheetItem::new(
                "jump",
                "Jump to symbol",
                KeyCombo::new(KeyModifiers::control_shift(), KeyKind::Char('j')),
            )),
        )
        .query("open");

    assert_eq!(vec!["open"], visible_ids(&sheet));

    let event = sheet.apply_action(ShortcutCheatsheetAction::SelectShortcut("open".to_string()));
    assert_eq!(
        Some(ShortcutCheatsheetEvent::ShortcutSelected {
            id: "open".to_string(),
            combo: open
        }),
        event
    );

    let query_event = sheet.apply_action(ShortcutCheatsheetAction::SetQuery("file".to_string()));
    assert_eq!(
        Some(ShortcutCheatsheetEvent::QueryChanged("file".to_string())),
        query_event
    );
    assert_eq!(vec!["open", "close"], visible_ids(&sheet));
}

#[test]
fn shortcut_cheatsheet_layout_is_typed_and_rendered() {
    let sheet = ShortcutCheatsheet::new("Shortcuts")
        .group_layout(ShortcutCheatsheetLayout::OneColumn)
        .group(
            ShortcutCheatsheetGroup::new("File").item(ShortcutCheatsheetItem::new(
                "open",
                "Open file",
                KeyCombo::new(KeyModifiers::command_shift(), KeyKind::Char('o')),
            )),
        );
    let node = UiNode::from(sheet);

    assert_eq!("OneColumn", node.props().interaction.value);
    assert_eq!(1, node.props().interaction.item_count);
}

fn visible_ids(sheet: &ShortcutCheatsheet) -> Vec<&str> {
    sheet
        .visible_items()
        .into_iter()
        .map(ShortcutCheatsheetItem::id)
        .collect()
}

struct StaticPlatformProvider(RuntimePlatform);

impl ShortcutPlatformProvider for StaticPlatformProvider {
    fn runtime_platform(&self) -> RuntimePlatform {
        self.0
    }
}
