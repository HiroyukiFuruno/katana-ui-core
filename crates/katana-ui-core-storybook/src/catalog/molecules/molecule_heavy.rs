use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{RgbaActionValue, UiAction};
use katana_ui_core::molecule::{
    CodeDiffLine, CodeDiffLineKind, CodeDiffMode, CodeDiffSource, CollapsedBlock,
    ColorBlendingMode, DiagnosticAction, DiagnosticFixPreview, DiagnosticItem, DiagnosticLocation,
    DiagnosticSeverity, DiagnosticsGroupBy, DiagnosticsListAction, DiagnosticsListOptions,
    DiagnosticsSortBy, DisclosureTriggerArea, HighlightRange, RgbaColor, TreeLineStyle, TreeNode,
};
use katana_ui_core::{atom, molecule};

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        code_diff_story(),
        color_picker_story(),
        diagnostics_list_story(),
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

const OLD_LINE_NUMBER: usize = 1;
const NEW_LINE_NUMBER: usize = 1;
const HIGHLIGHT_END_LINE: usize = 2;
const COLLAPSED_START_LINE: usize = 3;
const COLLAPSED_LINE_COUNT: usize = 4;
const COLOR_RED: u8 = 64;
const COLOR_GREEN: u8 = 128;
const COLOR_BLUE: u8 = 255;
const COLOR_ALPHA: u8 = 204;
const COLOR_HUE: u16 = 214;
const DIAGNOSTIC_ERROR_LINE: u32 = 12;
const DIAGNOSTIC_ERROR_DIFF_LINE: usize = 12;
const DIAGNOSTIC_ERROR_COLUMN: u32 = 9;
const DIAGNOSTIC_WARNING_LINE: u32 = 24;
const DIAGNOSTIC_WARNING_COLUMN: u32 = 5;

fn code_diff_story() -> StoryExample {
    let mut diff = molecule::CodeDiff::new("Code diff")
        .source(CodeDiffSource::Unified {
            text: "- old\n+ new".to_string(),
        })
        .mode(CodeDiffMode::Inline)
        .line(CodeDiffLine {
            old_number: Some(OLD_LINE_NUMBER),
            new_number: None,
            kind: CodeDiffLineKind::Removed,
            text: "old".to_string(),
        })
        .line(CodeDiffLine {
            old_number: None,
            new_number: Some(NEW_LINE_NUMBER),
            kind: CodeDiffLineKind::Added,
            text: "new".to_string(),
        })
        .highlight(HighlightRange {
            start_line: OLD_LINE_NUMBER,
            end_line: HIGHLIGHT_END_LINE,
        })
        .collapsed_block(CollapsedBlock {
            start_line: COLLAPSED_START_LINE,
            line_count: COLLAPSED_LINE_COUNT,
        })
        .child(atom::Text::new("- old"))
        .child(atom::Text::new("+ new"));
    let target = diff.state_id().clone();
    let result = diff.apply_action(&UiAction::code_diff_mode(target, "Split"));
    StoryCatalog::interactive_story("code-diff", diff, result.callback_log)
}

fn color_picker_story() -> StoryExample {
    let mut picker = molecule::ColorPicker::new("Color picker")
        .open(true)
        .rgba(RgbaColor::new(
            COLOR_RED,
            COLOR_GREEN,
            COLOR_BLUE,
            COLOR_ALPHA,
        ))
        .hue(COLOR_HUE)
        .alpha(COLOR_ALPHA)
        .blending(ColorBlendingMode::Screen)
        .child(atom::ColorSwatch::new("Preview").value("rgba(64, 128, 255, 204)"))
        .child(atom::SlideControl::new("Alpha").value(COLOR_ALPHA.to_string()));
    let target = picker.state_id().clone();
    let result = picker.apply_action(&UiAction::color_drag(
        target,
        RgbaActionValue::new(COLOR_RED, COLOR_GREEN, COLOR_BLUE, COLOR_ALPHA),
        COLOR_HUE,
        true,
    ));
    StoryCatalog::interactive_story("color-picker-rgba", picker, result.callback_log)
}

fn diagnostics_list_story() -> StoryExample {
    let mut diagnostics = molecule::DiagnosticsList::new("Diagnostics")
        .option(DiagnosticsListOptions {
            group_by: DiagnosticsGroupBy::Severity,
            sort_by: DiagnosticsSortBy::Severity,
            severity_filter: [DiagnosticSeverity::Error, DiagnosticSeverity::Warning]
                .into_iter()
                .collect(),
            ..DiagnosticsListOptions::default()
        })
        .item(diagnostic_error())
        .item(diagnostic_warning())
        .empty_slot(atom::Text::new("No diagnostics"))
        .bulk_preview(
            molecule::ModalOverlay::new("Bulk fix preview")
                .child(atom::Text::new("Apply all safe quick fixes")),
        );
    let target = diagnostics.state_id().clone();
    let events = diagnostics.apply_action(DiagnosticsListAction::ToggleFixPreview(
        molecule::DiagnosticId::new("syntax-error"),
    ));
    let log = katana_ui_core::interaction::UiCallbackLog::new(
        target,
        "diagnostic_fix_preview",
        "expanded=false",
        format!("events={events:?}"),
    );
    StoryCatalog::interactive_story("diagnostics-list", diagnostics, vec![log])
}

fn diagnostic_error() -> DiagnosticItem {
    DiagnosticItem::new(
        "syntax-error",
        DiagnosticSeverity::Error,
        "Missing semicolon",
        DiagnosticLocation::new(
            "crates/katana-ui-core/src/lib.rs",
            DIAGNOSTIC_ERROR_LINE,
            DIAGNOSTIC_ERROR_COLUMN,
        ),
    )
    .source("rustc")
    .quickfix(DiagnosticAction::new(
        "insert-semicolon",
        "Insert semicolon",
    ))
    .fix_preview(DiagnosticFixPreview::new(
        molecule::CodeDiff::new("Fix preview").line(CodeDiffLine {
            old_number: Some(DIAGNOSTIC_ERROR_DIFF_LINE),
            new_number: Some(DIAGNOSTIC_ERROR_DIFF_LINE),
            kind: CodeDiffLineKind::Added,
            text: "let value = compute();".to_string(),
        }),
    ))
}

fn diagnostic_warning() -> DiagnosticItem {
    DiagnosticItem::new(
        "unused-import",
        DiagnosticSeverity::Warning,
        "Unused import",
        DiagnosticLocation::new(
            "crates/katana-ui-core/src/story.rs",
            DIAGNOSTIC_WARNING_LINE,
            DIAGNOSTIC_WARNING_COLUMN,
        ),
    )
    .source("clippy")
    .quickfix(DiagnosticAction::new("remove-import", "Remove import"))
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
