use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::molecule::{
    CodeDiffLine, CodeDiffLineKind, CodeDiffMode, CodeDiffSource, CollapsedBlock,
    ColorBlendingMode, HighlightRange, RgbaColor,
};
use katana_ui_core::{atom, molecule, render_model::UiSize};

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

pub(super) fn examples() -> Vec<StoryExample> {
    vec![code_diff_story(), color_picker_story()]
}

fn code_diff_story() -> StoryExample {
    let mut diff = molecule::CodeDiff::new("Code diff")
        .source(CodeDiffSource::Unified {
            text: "- old\n+ new\n  日本語 diff".to_string(),
        })
        .mode(CodeDiffMode::Inline)
        .line(removed_line())
        .line(added_line())
        .highlight(HighlightRange {
            start_line: OLD_LINE_NUMBER,
            end_line: HIGHLIGHT_END_LINE,
        })
        .collapsed_block(CollapsedBlock {
            start_line: COLLAPSED_START_LINE,
            line_count: COLLAPSED_LINE_COUNT,
        })
        .child(atom::Text::new("mode: split / inline"))
        .child(atom::Text::new("direction: left-right / top-bottom"))
        .child(atom::Text::new("collapse: 非表示 4 行"))
        .child(atom::Text::new("whitespace: space=· tab=→"))
        .child(atom::Text::new("日本語 highlight: 表示ずれなし"));
    let target = diff.state_id().clone();
    let result = diff.apply_action(&UiAction::code_diff_mode(target, "Split"));
    StoryCatalog::interactive_story("code-diff", diff, result.callback_log)
}

fn removed_line() -> CodeDiffLine {
    CodeDiffLine {
        old_number: Some(OLD_LINE_NUMBER),
        new_number: None,
        kind: CodeDiffLineKind::Removed,
        text: "old line".to_string(),
    }
}

fn added_line() -> CodeDiffLine {
    CodeDiffLine {
        old_number: None,
        new_number: Some(NEW_LINE_NUMBER),
        kind: CodeDiffLineKind::Added,
        text: "new line".to_string(),
    }
}

fn color_picker_story() -> StoryExample {
    let picker = molecule::ColorPicker::new("Color picker")
        .open(true)
        .rgba(RgbaColor::new(
            COLOR_RED,
            COLOR_GREEN,
            COLOR_BLUE,
            COLOR_ALPHA,
        ))
        .hue(COLOR_HUE)
        .alpha(COLOR_ALPHA)
        .blending(ColorBlendingMode::Normal)
        .color_area("saturation/value plane")
        .trigger_size(UiSize::Large)
        .eyedropper_callback("storybook-eyedropper")
        .title("Brand accent")
        .child(
            atom::ColorSwatch::new("preview: transparent checker + opaque RGB")
                .value("rgba(64, 128, 255, 204)"),
        )
        .child(atom::Text::new(
            "settings: mode=RGBA eyedropper=true blending=Normal/Additive",
        ))
        .child(atom::Text::new(
            "trigger: color-only large border with transparent checker",
        ))
        .child(atom::Text::new(
            "floating panel: rgb/rgba popup with saturation/value plane",
        ))
        .child(atom::Text::new(
            "state: open=true value=rgba(64,128,255,204)",
        ))
        .child(atom::Text::new(
            "event: RgbaChanged AlphaChanged BlendingChanged",
        ))
        .child(atom::Text::new(
            "action: open close drag-plane drag-hue drag-alpha numeric-input eyedropper",
        ))
        .child(atom::Text::new("preset: RGBA RGB readonly disabled"))
        .child(atom::Text::new("U8 fields: R=64 G=128 B=255 A=204"))
        .child(atom::Text::new("plane: saturation/value with drag handle"))
        .child(atom::Text::new("slider: hue=214 alpha=204 with handles"))
        .child(atom::SlideControl::new("Hue slider").value(COLOR_HUE.to_string()))
        .child(atom::SlideControl::new("Alpha slider").value(COLOR_ALPHA.to_string()));
    let target = picker.state_id().clone();
    let logs = color_picker_callback_logs(target);
    StoryCatalog::interactive_story("color-picker-rgba", picker, logs)
}

fn color_picker_callback_logs(
    target: katana_ui_core::render_model::UiStateId,
) -> Vec<UiCallbackLog> {
    vec![
        color_picker_log(
            &target,
            "color_picker_open",
            "open=false",
            "ColorPickerOpened open=true",
        ),
        color_picker_log(
            &target,
            "color_drag",
            "plane=saturation/value R=64 G=128 B=255 A=204",
            "RgbaChanged plane=drag R=72 G=136 B=240 A=188",
        ),
        color_picker_log(&target, "color_hue_drag", "hue=214", "HueChanged hue=226"),
        color_picker_log(
            &target,
            "color_alpha_drag",
            "alpha=204",
            "AlphaChanged alpha=188",
        ),
        color_picker_log(
            &target,
            "color_numeric_input",
            "R=64 G=128 B=255 A=204",
            "NumericFieldChanged R=72 G=136 B=240 A=188",
        ),
        color_picker_log(
            &target,
            "color_blending_changed",
            "blending=Normal",
            "BlendingChanged blending=Additive",
        ),
        color_picker_log(
            &target,
            "color_eyedropper_request",
            "eyedropper=ready",
            "EyedropperRequested action=storybook-eyedropper",
        ),
        color_picker_log(
            &target,
            "color_picker_close",
            "open=true",
            "ColorPickerClosed open=false",
        ),
        color_picker_log(
            &target,
            "color_picker_readonly_blocked",
            "readonly=true",
            "ReadonlySuppressed value=rgba(64,128,255,204)",
        ),
        color_picker_log(
            &target,
            "color_picker_disabled_blocked",
            "disabled=true",
            "DisabledSuppressed value=rgba(64,128,255,204)",
        ),
    ]
}

fn color_picker_log(
    target: &katana_ui_core::render_model::UiStateId,
    action: &str,
    before: &str,
    after: &str,
) -> UiCallbackLog {
    UiCallbackLog::new(target.clone(), action, before, after)
}
