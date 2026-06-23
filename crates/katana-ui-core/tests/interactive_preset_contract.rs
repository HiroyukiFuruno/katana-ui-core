use katana_ui_core::atom::{
    Button, Checkbox, ColorSwatch, IconTextButton, Radio, SlideControl, SvgButton, Text,
    TextButton, Toggle,
};
use katana_ui_core::layout::Stack;
use katana_ui_core::molecule::{
    SettingsControl, SettingsField, SettingsList, SettingsSection, TreeNode, TreeView,
};
use katana_ui_core::render_model::{
    UiBorder, UiCursor, UiHostActionSpec, UiNode, UiNodeKind, UiTextSpan, UiTextSpanStyle, UiTree,
};

#[test]
fn host_action_applies_control_interactive_preset() {
    let node = UiNode::from(Stack::new().child(Text::new("Open")))
        .host_action(UiHostActionSpec::command("app.open", "Open").payload("target"));

    assert_eq!(UiCursor::Pointer, node.props().common.cursor);
    assert_eq!(
        UiBorder::solid(1, 4, "control.hover.border"),
        node.props().common.hover_border
    );
}

#[test]
fn link_text_span_applies_control_interactive_preset() {
    let node: UiNode = Text::new("Release")
        .text_spans(vec![UiTextSpan {
            text: "Release".to_string(),
            style: UiTextSpanStyle::default(),
            link_target: "https://example.test".to_string(),
        }])
        .into();

    assert_eq!(UiCursor::Pointer, node.props().common.cursor);
    assert_eq!(
        UiBorder::solid(1, 4, "control.hover.border"),
        node.props().common.hover_border
    );
}

#[test]
fn interactive_atoms_expose_control_interactive_preset_by_default() {
    let cases = [
        ("button", UiNode::from(Button::new("Save"))),
        ("text-button", UiNode::from(TextButton::new("Save"))),
        ("svg-button", UiNode::from(SvgButton::new("Copy"))),
        (
            "icon-text-button",
            UiNode::from(IconTextButton::new("Open folder")),
        ),
        ("checkbox", UiNode::from(Checkbox::new("Enabled"))),
        ("radio", UiNode::from(Radio::new("Mode"))),
        ("toggle", UiNode::from(Toggle::new("Dark"))),
        ("color-swatch", UiNode::from(ColorSwatch::new("Accent"))),
        ("slide-control", UiNode::from(SlideControl::new("Opacity"))),
    ];

    for (name, node) in cases {
        assert_eq!(UiCursor::Pointer, node.props().common.cursor, "{name}");
        assert_eq!(
            UiBorder::solid(1, 4, "control.hover.border"),
            node.props().common.hover_border,
            "{name}"
        );
    }
}

#[test]
fn tree_view_rows_expose_control_interactive_preset() {
    let tree =
        UiTree::new(TreeView::new("Files").item(TreeNode::new("src/lib.rs", "lib.rs", 0).file()));
    let root = tree.root();

    assert_eq!(UiCursor::Pointer, root.props().common.cursor);
    assert_eq!(UiCursor::Pointer, root.props().tree.row_cursor);
    assert_eq!(
        UiBorder::solid(1, 4, "control.hover.border"),
        root.props().tree.row_hover_border
    );
}

#[test]
fn settings_list_fields_expose_control_interactive_preset() -> Result<(), std::io::Error> {
    let node: UiNode = SettingsList::new("Settings")
        .section(
            SettingsSection::new("display", "Display").field(SettingsField::new(
                "dark",
                "Dark",
                SettingsControl::Toggle { checked: true },
            )),
        )
        .into();
    let field = find_kind(&node, UiNodeKind::FormField)
        .ok_or_else(|| std::io::Error::other("settings field missing"))?;

    assert_eq!(UiCursor::Pointer, field.props().common.cursor);
    assert_eq!(
        UiBorder::solid(1, 4, "control.hover.border"),
        field.props().common.hover_border
    );
    Ok(())
}

fn find_kind(node: &UiNode, kind: UiNodeKind) -> Option<&UiNode> {
    if node.kind() == kind {
        return Some(node);
    }
    node.children()
        .iter()
        .find_map(|child| find_kind(child, kind))
}
