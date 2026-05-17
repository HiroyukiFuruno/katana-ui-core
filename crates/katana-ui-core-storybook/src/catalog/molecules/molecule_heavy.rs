use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::molecule::{
    CodeDiffLine, CodeDiffLineKind, CodeDiffMode, CodeDiffSource, CollapsedBlock,
    ColorBlendingMode, HighlightRange, RgbaColor,
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
        StoryCatalog::story(
            "tree-view",
            molecule::TreeView::new("Tree view")
                .open(true)
                .item_count(2)
                .child(atom::Text::new("Parent"))
                .child(atom::Text::new("Child")),
        ),
    ]
}
