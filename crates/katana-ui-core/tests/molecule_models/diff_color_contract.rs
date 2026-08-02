use katana_ui_core::atom::Text;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiActionSource};
use katana_ui_core::molecule::{
    CodeDiff, CodeDiffDirection, CodeDiffLine, CodeDiffLineKind, CodeDiffMode, CodeDiffSource,
    CodeDiffWhitespace, CollapsedBlock, ColorBlendingMode, ColorPicker, HighlightRange, RgbaColor,
};
use katana_ui_core::render_model::{UiColorBlendingMode, UiSize, UiTree};

const FIRST_LINE: usize = 1;
const COLLAPSED_START_LINE: usize = 2;
const COLLAPSED_LINE_COUNT: usize = 3;
const LONG_LINE_COLUMN: usize = 80;
const LONG_LINE_COLUMN_JSON: &str = "\"long_line_column\":80";
const DIFF_LANGUAGE_JSON: &str = "\"language\":\"rust\"";
const BRAND_RED: u8 = 64;
const BRAND_GREEN: u8 = 128;
const BRAND_BLUE: u8 = 255;
const BRAND_ALPHA: u8 = 200;
const BRAND_HUE: u16 = 220;
const MODEL_ALPHA: u8 = 180;
const CHILD_COUNT: usize = 3;

#[test]
fn code_diff_has_kuc_owned_source_lines_highlights_and_collapsed_blocks() {
    let diff = CodeDiff::new("Diff")
        .source(CodeDiffSource::Split {
            before: "old".to_string(),
            after: "new".to_string(),
        })
        .mode(CodeDiffMode::Split)
        .line(CodeDiffLine {
            old_number: Some(FIRST_LINE),
            new_number: None,
            kind: CodeDiffLineKind::Removed,
            text: "old".to_string(),
        })
        .highlight(HighlightRange {
            start_line: FIRST_LINE,
            end_line: FIRST_LINE,
        })
        .collapsed_block(CollapsedBlock {
            start_line: COLLAPSED_START_LINE,
            line_count: COLLAPSED_LINE_COUNT,
        })
        .child(Text::new("before"))
        .child(Text::new("after"));

    assert!(matches!(
        diff.source_model(),
        Some(CodeDiffSource::Split { .. })
    ));
    assert_eq!(CodeDiffMode::Split, diff.mode_model());
    assert_eq!(CodeDiffLineKind::Removed, diff.lines()[0].kind);
    assert_eq!(FIRST_LINE, diff.highlights()[0].start_line);
    assert_eq!(COLLAPSED_LINE_COUNT, diff.collapsed_blocks()[0].line_count);
    assert_eq!(1, UiTree::new(diff).root().props().interaction.item_count);
}

#[test]
fn code_diff_snapshot_requires_typed_layout_whitespace_newline_and_collapse_details()
-> serde_json::Result<()> {
    let diff = CodeDiff::new("Diff detail")
        .source(CodeDiffSource::Split {
            before: "let value = old;\n".to_string(),
            after: "let value = new".to_string(),
        })
        .mode(CodeDiffMode::Inline)
        .direction(CodeDiffDirection::Vertical)
        .language("rust")
        .whitespace(CodeDiffWhitespace::visible("·", "→"))
        .long_line_column(LONG_LINE_COLUMN)
        .trailing_newline_difference(true)
        .line(CodeDiffLine {
            old_number: Some(FIRST_LINE),
            new_number: None,
            kind: CodeDiffLineKind::Removed,
            text: "let value = old;".to_string(),
        })
        .line(CodeDiffLine {
            old_number: None,
            new_number: Some(FIRST_LINE),
            kind: CodeDiffLineKind::Added,
            text: "let value = new".to_string(),
        });
    let encoded = serde_json::to_string(&diff)?;

    assert!(encoded.contains("\"mode\":\"Inline\""));
    assert!(encoded.contains("\"direction\":\"Vertical\""));
    assert!(encoded.contains(DIFF_LANGUAGE_JSON));
    assert!(encoded.contains("\"space_symbol\":\"·\""));
    assert!(encoded.contains("\"tab_symbol\":\"→\""));
    assert!(encoded.contains(LONG_LINE_COLUMN_JSON));
    assert!(encoded.contains("\"trailing_newline_difference\":true"));
    assert_eq!(CodeDiffDirection::Vertical, diff.direction_model());
    assert_eq!("rust", diff.language_model());
    assert_eq!(Some(LONG_LINE_COLUMN), diff.long_line_column_model());
    assert!(diff.has_trailing_newline_difference());
    assert!(diff.whitespace_model().is_some());
    Ok(())
}

#[test]
fn code_diff_is_not_complete_with_only_generic_value_or_item_count() {
    let diff = CodeDiff::new("Diff detail").item_count(COLLAPSED_LINE_COUNT);

    assert!(diff.source_model().is_none());
    assert!(diff.lines().is_empty());
    assert!(diff.collapsed_blocks().is_empty());
    assert!(diff.whitespace_model().is_none());
    assert!(diff.language_model().is_empty());
    assert_eq!(CodeDiffMode::Split, diff.mode_model());
    assert_eq!(CodeDiffDirection::Horizontal, diff.direction_model());
}

#[test]
fn color_picker_keeps_rgba_hue_alpha_and_blending_model() {
    let mut picker = ColorPicker::new("Color")
        .rgba(RgbaColor::new(
            BRAND_RED,
            BRAND_GREEN,
            BRAND_BLUE,
            BRAND_ALPHA,
        ))
        .hue(BRAND_HUE)
        .alpha(MODEL_ALPHA)
        .blending(ColorBlendingMode::Multiply)
        .color_area("saturation/value square")
        .trigger_size(UiSize::XLarge)
        .title("Brand color")
        .rgba_mode(true)
        .trigger_border(false)
        .eyedropper_callback("pick-screen-color")
        .readonly(true);
    let result = picker.apply_action(&UiAction::color_drag(
        picker.state_id().clone(),
        katana_ui_core::interaction::RgbaActionValue::new(0, 0, 0, 255),
        BRAND_HUE,
        true,
    ));
    let open = picker.apply_action(&UiAction::set_open(picker.state_id().clone(), true));

    assert!(!result.handled);
    assert!(open.handled);
    assert_eq!(MODEL_ALPHA, picker.color_value().alpha);
    assert_eq!(BRAND_HUE, picker.hue_value());
    assert_eq!(MODEL_ALPHA, picker.alpha_value());
    assert_eq!(ColorBlendingMode::Multiply, picker.blending_mode());
    assert!(picker.previews_color());
    assert_eq!("saturation/value square", picker.color_area_model());
    assert_eq!(UiSize::XLarge, picker.trigger_size_model());
    assert_eq!("Brand color", picker.title_model());
    assert!(picker.uses_rgba_mode());
    assert!(!picker.has_trigger_border());
    assert_eq!("pick-screen-color", picker.eyedropper_callback_model());
}

#[test]
fn color_picker_keeps_typed_model_and_projects_children_to_neutral_tree() {
    let picker = ColorPicker::new("Color")
        .rgba(RgbaColor::new(
            BRAND_RED,
            BRAND_GREEN,
            BRAND_BLUE,
            BRAND_ALPHA,
        ))
        .hue(BRAND_HUE)
        .child(Text::new("surface"))
        .child(Text::new("hue"))
        .child(Text::new("alpha"));

    assert_eq!(BRAND_HUE, picker.hue_value());
    assert_eq!(CHILD_COUNT, UiTree::new(picker).root().children().len());
}

#[test]
fn color_blending_parser_and_render_mapping_cover_every_mode() {
    let cases = [
        (
            "normal",
            ColorBlendingMode::Normal,
            UiColorBlendingMode::Normal,
        ),
        (
            "additive",
            ColorBlendingMode::Additive,
            UiColorBlendingMode::Additive,
        ),
        (
            "replace",
            ColorBlendingMode::Replace,
            UiColorBlendingMode::Replace,
        ),
        (
            "multiply",
            ColorBlendingMode::Multiply,
            UiColorBlendingMode::Multiply,
        ),
        (
            "screen",
            ColorBlendingMode::Screen,
            UiColorBlendingMode::Screen,
        ),
    ];

    for (name, mode, rendered) in cases {
        assert_eq!(Some(mode), ColorBlendingMode::parse(name));
        assert_eq!(
            rendered,
            UiTree::new(ColorPicker::new("Color").blending(mode))
                .root()
                .props()
                .color_picker
                .blending
        );
    }
    assert_eq!(None, ColorBlendingMode::parse("unsupported"));
}

#[test]
fn color_picker_rejects_unknown_blending_mode_without_mutation() {
    let mut picker = ColorPicker::new("Color").blending(ColorBlendingMode::Screen);
    let result = picker.apply_action(&UiAction::SetValue {
        target: picker.state_id().clone(),
        value: "unsupported".to_string(),
        source: UiActionSource::ColorPickerBlending,
        progress: None,
        color_drag: None,
    });

    assert!(!result.handled);
    assert_eq!(ColorBlendingMode::Screen, picker.blending_mode());
}
