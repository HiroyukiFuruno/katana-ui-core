use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::StoryCatalog;
use crate::catalog::StoryPresetLabels;
use crate::test_assert::KucTestExpect;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "banner";
const ERROR_PRESET: usize = 0;
const VENDOR_PRESET: usize = 1;
const ATTACHMENT_PRESET: usize = 2;
const SUCCESS_PRESET: usize = 3;
const DETAILS_PRESET: usize = 4;
const TITLE_PRESET: usize = 5;
const LEADING_ICON_PRESET: usize = 6;
const PLACEMENT_PRESET: usize = 7;
const REQUIRED_PRESET_COUNT: usize = 8;
const REQUIRED_OPTION_COUNT: usize = 8;
const BODY_DIFF_THRESHOLD: usize = 80;
const BANNER_X: usize = 40;
const BANNER_Y: usize = 34;
const BANNER_SAMPLE_X_OFFSET: usize = 244;
const BANNER_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn banner_exposes_leaf_presets_options_and_details_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("banner_toggle_details", spec.action);
    assert_eq!("banner_details_toggled", spec.event);
    assert_eq!("banner.details_open", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("details_open=true", spec.state);
    assert!(
        options
            .iter()
            .any(|option| option.setting == "banner.title")
    );
    assert!(
        options
            .iter()
            .any(|option| option.setting == "banner.leading_icon")
    );
    assert!(
        options
            .iter()
            .any(|option| option.setting == "banner.placement")
    );
}

#[test]
fn banner_presets_render_distinct_error_adapter_attachment_success_and_details_states() {
    let error = StorybookVisual.render_preset(DARK_THEME, PAGE, ERROR_PRESET, 0);
    let adapter = StorybookVisual.render_preset(DARK_THEME, PAGE, VENDOR_PRESET, 0);
    let attachment = StorybookVisual.render_preset(DARK_THEME, PAGE, ATTACHMENT_PRESET, 0);
    let success = StorybookVisual.render_preset(DARK_THEME, PAGE, SUCCESS_PRESET, 0);
    let details = StorybookVisual.render_preset(DARK_THEME, PAGE, DETAILS_PRESET, 0);
    let title = StorybookVisual.render_preset(DARK_THEME, PAGE, TITLE_PRESET, 0);
    let icon = StorybookVisual.render_preset(DARK_THEME, PAGE, LEADING_ICON_PRESET, 0);
    let placement = StorybookVisual.render_preset(DARK_THEME, PAGE, PLACEMENT_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &error, &adapter) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &adapter, &attachment) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &attachment, &success) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &success, &details) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &details, &title) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &title, &icon) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &icon, &placement) > BODY_DIFF_THRESHOLD);
}

#[test]
fn banner_setting_option_updates_banner_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn banner_preview_action_updates_details_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn banner_story_connects_core_details_action_and_dismiss_events() {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|example| example.page == PAGE)
        .kuc_expect("banner story exists");

    for (action, event) in [
        ("banner_toggle_details", "BannerDetailsToggled"),
        ("banner_primary_action", "BannerActioned"),
        ("banner_dismiss", "BannerDismissed"),
    ] {
        assert!(
            story
                .callback_logs
                .iter()
                .any(|callback| callback.action == action && callback.after.contains(event)),
            "banner callback log lacks {action}/{event}"
        );
    }
}

#[test]
fn banner_light_and_dark_surface_uses_theme_surface() {
    assert_banner_token(DARK_THEME, ThemeSnapshot::dark());
    assert_banner_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_banner_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, ERROR_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + BANNER_X + BANNER_SAMPLE_X_OFFSET,
            component.y + BANNER_Y + BANNER_SAMPLE_Y_OFFSET
        )
    );
}
