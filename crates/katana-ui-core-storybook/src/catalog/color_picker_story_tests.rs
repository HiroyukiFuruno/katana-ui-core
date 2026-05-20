use super::{StoryCatalog, StoryDetailContent};
use crate::catalog::panel_interaction::StorybookPanelInteractionReport;
use katana_ui_core::render_model::{UiColorBlendingMode, UiSize};

const COLOR_PICKER_PAGE: &str = "color-picker-rgba";

#[test]
fn color_picker_story_exposes_rgba_controls_as_contract_data() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == COLOR_PICKER_PAGE)
        .ok_or("color picker page missing")?;
    let props = story.tree.root().props();
    let labels = story
        .tree
        .root()
        .children()
        .iter()
        .map(|it| it.props().label.as_str())
        .collect::<Vec<_>>();

    assert_eq!("rgba(64, 128, 255, 204)", props.color_picker.rgba_css);
    assert_eq!(
        "rgba(64, 128, 255, 255)",
        props.color_picker.opaque_preview_css
    );
    assert!(props.color_picker.checker_background);
    assert!(props.color_picker.rgba_mode);
    assert!(props.color_picker.alpha_slider_visible);
    assert!(props.color_picker.eyedropper_visible);
    assert_eq!(214, props.color_picker.hue_degrees);
    assert_eq!(204, props.color_picker.alpha);
    assert_eq!(UiColorBlendingMode::Normal, props.color_picker.blending);
    assert_eq!("saturation/value plane", props.color_picker.color_plane);
    assert_eq!("storybook-eyedropper", props.color_picker.eyedropper_action);
    assert_eq!(75, props.color_picker.panel_scale_percent);
    assert_eq!(UiSize::Large, props.size);

    for expected in [
        "preview: transparent checker + opaque RGB",
        "settings: mode=RGBA eyedropper=true blending=Normal/Additive",
        "state: open=true value=rgba(64,128,255,204)",
        "event: RgbaChanged AlphaChanged BlendingChanged",
        "action: open close drag-plane drag-hue drag-alpha numeric-input eyedropper",
        "preset: RGBA RGB readonly disabled",
        "U8 fields: R=64 G=128 B=255 A=204",
        "plane: saturation/value with drag handle",
        "slider: hue=214 alpha=204 with handles",
    ] {
        assert!(
            labels.iter().any(|it| it.contains(expected)),
            "color picker preview lacks {expected}"
        );
    }
    Ok(())
}

#[test]
fn color_picker_story_logs_required_storybook_actions() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == COLOR_PICKER_PAGE)
        .ok_or("color picker page missing")?;
    let details = StoryDetailContent::from_example(story);

    for action in [
        "color_picker_open",
        "color_drag",
        "color_hue_drag",
        "color_alpha_drag",
        "color_numeric_input",
        "color_blending_changed",
        "color_eyedropper_request",
        "color_picker_close",
        "color_picker_readonly_blocked",
        "color_picker_disabled_blocked",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "color picker callback log lacks {action}"
        );
        assert!(
            details.settings.contains(action),
            "color picker settings inspector lacks {action}"
        );
    }

    for event in [
        "ColorPickerOpened",
        "RgbaChanged",
        "HueChanged",
        "AlphaChanged",
        "NumericFieldChanged",
        "BlendingChanged",
        "EyedropperRequested",
        "ReadonlySuppressed",
        "DisabledSuppressed",
    ] {
        assert!(
            story
                .callback_logs
                .iter()
                .any(|it| it.after.contains(event)),
            "color picker callback log lacks {event}"
        );
    }
    Ok(())
}

#[test]
fn color_picker_settings_report_covers_legacy_22_options() {
    let examples = StoryCatalog.examples();
    let report = StorybookPanelInteractionReport::build(&examples);

    for option in [
        "color_picker.mode",
        "color_picker.red",
        "color_picker.green",
        "color_picker.blue",
        "color_picker.alpha",
        "color_picker.blending",
        "color_picker.eyedropper",
        "color_picker.readonly",
        "color_picker.disabled",
    ] {
        assert!(
            report.settings_mutations.iter().any(|it| {
                it.page == COLOR_PICKER_PAGE
                    && it.option.name == option
                    && it.event == "color_picker_settings_changed"
            }),
            "missing color picker setting mutation for {option}"
        );
    }
}
