use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{
    CodeDiffLine, CodeDiffLineKind, CodeDiffMode, CodeDiffSource, CollapsedBlock,
    ColorBlendingMode, DisclosureTriggerArea, HighlightRange, RgbaColor, TreeLineStyle, TreeNode,
};
use katana_ui_core::{atom, molecule};

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        StoryCatalog::story(
            "code-diff",
            molecule::CodeDiff::new("Code diff")
                .source(CodeDiffSource::Unified {
                    text: "- old\n+ new".to_string(),
                })
                .mode(CodeDiffMode::Inline)
                .line(CodeDiffLine {
                    old_number: Some(1),
                    new_number: None,
                    kind: CodeDiffLineKind::Removed,
                    text: "old".to_string(),
                })
                .line(CodeDiffLine {
                    old_number: None,
                    new_number: Some(1),
                    kind: CodeDiffLineKind::Added,
                    text: "new".to_string(),
                })
                .highlight(HighlightRange {
                    start_line: 1,
                    end_line: 2,
                })
                .collapsed_block(CollapsedBlock {
                    start_line: 3,
                    line_count: 4,
                })
                .child(atom::Text::new("- old"))
                .child(atom::Text::new("+ new")),
        ),
        StoryCatalog::story(
            "color-picker-rgba",
            molecule::ColorPicker::new("Color picker")
                .open(true)
                .rgba(RgbaColor::new(64, 128, 255, 204))
                .hue(214)
                .alpha(204)
                .blending(ColorBlendingMode::Screen)
                .child(atom::ColorSwatch::new("Preview").value("rgba(64, 128, 255, 204)"))
                .child(atom::SlideControl::new("Alpha").value("204")),
        ),
        StoryCatalog::story(
            "command-palette",
            molecule::CommandPalette::new("Command palette")
                .open(true)
                .selected_index(0)
                .item_count(1)
                .child(molecule::SearchBox::new("Search"))
                .child(molecule::SelectionList::new("Commands"))
                .child(atom::Text::new("Action")),
        ),
        StoryCatalog::story(
            "dynamic-array-editor",
            molecule::DynamicArrayEditor::new("Dynamic array")
                .item_count(1)
                .child(atom::Button::new("Add"))
                .child(atom::Text::new("Item")),
        ),
        tree_view_story(),
    ]
}

fn tree_view_story() -> StoryExample {
    let mut tree = molecule::TreeView::new("Tree view")
        .default_open(true)
        .line_display(true)
        .line_style(TreeLineStyle::Solid)
        .line_width(1)
        .icons_visible(true)
        .directory_icon("<svg data-icon=\"folder\"/>")
        .file_icon("<svg data-icon=\"file\"/>")
        .tree_font_role("body")
        .tree_theme_id("dark")
        .empty_area_context_menu(true)
        .toggle_icon("<svg data-icon=\"chevron\"/>")
        .toggle_trigger_area(DisclosureTriggerArea::IconAndText)
        .item(
            TreeNode::new("atoms", "Atoms", 0)
                .directory()
                .expanded(true),
        )
        .item(TreeNode::new("button", "Button", 1).file().selected(true))
        .item_count(2)
        .child(atom::Text::new("Parent"))
        .child(atom::Text::new("Child"));
    let result = tree.apply_action(&UiAction::click(tree.state_id().clone()));
    StoryCatalog::interactive_story("tree-view", tree, result.callback_log)
}
