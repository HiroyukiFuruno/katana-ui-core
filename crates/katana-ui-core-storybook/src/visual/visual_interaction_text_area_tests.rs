use super::interaction_spec::StorybookInteractionSpec;
use super::render_context::ScenarioContext;
use super::screen_state::StorybookScreenState;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{
    StorybookVisual, inspector_rows, palette, preview_detail, storybook_ui_option_contract,
    window_interaction,
};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "text-area";
const CHAT_PRESET: usize = 0;
const NEWLINE_PRESET: usize = 1;
const WRAP_PRESET: usize = 2;
const RESIZE_PRESET: usize = 3;
const AUTO_GROW_PRESET: usize = 4;
const VERTICAL_SCROLL_PRESET: usize = 5;
const HORIZONTAL_SCROLL_PRESET: usize = 6;
const TAB_BEHAVIOR_PRESET: usize = 7;
const VERTICAL_SCROLLBAR_PRESET: usize = 8;
const HORIZONTAL_SCROLLBAR_PRESET: usize = 9;
const LEADING_SLOT_PRESET: usize = 10;
const TRAILING_BUTTONS_PRESET: usize = 11;
const CLEAR_ACTION_PRESET: usize = 12;
const REQUIRED_PRESET_COUNT: usize = 24;
const REQUIRED_OPTION_COUNT: usize = 24;
const BODY_DIFF_THRESHOLD: usize = 80;
const TEXT_AREA_X_OFFSET: usize = 18;
const TEXT_AREA_Y_OFFSET: usize = 32;
const TEXT_AREA_FILL_SAMPLE_X_OFFSET: usize = 8;
const TEXT_AREA_FILL_SAMPLE_Y_OFFSET: usize = 8;
const TEXT_AREA_RESIZE_GRIP_X_OFFSET: usize = 250;
const TEXT_AREA_RESIZE_GRIP_Y_OFFSET: usize = 110;

#[test]
fn text_area_exposes_leaf_presets_options_and_multiline_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!(
        &[
            "chat composer",
            "newline key",
            "wrap policy",
            "resize handle",
            "auto grow",
            "vertical scroll",
            "horizontal scroll",
            "tab behavior",
            "vertical scrollbar",
            "horizontal scrollbar",
            "leading svg",
            "icon callbacks",
            "clear action",
            "value",
            "placeholder",
            "font role",
            "disabled",
            "readonly",
            "invalid",
            "min rows",
            "max rows",
            "ime",
            "leading slot",
            "trailing slot",
        ],
        presets
    );
    assert_eq!("text_area_type", spec.action);
    assert_eq!("text_area_changed", spec.event);
    assert_eq!("text_area.resize_enabled", spec.option);
    assert_eq!("resize=true", spec.state);
}

#[test]
fn text_area_inspector_displays_text_area_option_contract() -> Result<(), String> {
    let rows = inspector_setting_rows()?;

    for option in [
        "text_area.submit_key",
        "text_area.newline_key",
        "text_area.tab_behavior",
        "text_area.auto_grow",
        "text_area.wrap_policy",
        "text_area.resize_enabled",
        "text_area.vertical_scroll_enabled:...",
        "text_area.horizontal_scroll_enable...",
        "text_area.vertical_scrollbar_visib...",
        "text_area.horizontal_scrollbar_vis...",
        "text_area.leading_slot.icon",
        "text_area.trailing_icon_buttons",
        "text_area.clear_action",
        "text_area.value",
        "text_area.placeholder",
        "text_area.font_role",
        "text_area.disabled",
        "text_area.readonly",
        "text_area.invalid",
        "text_area.min_rows",
        "text_area.max_rows",
        "text_area.ime_enabled",
        "text_area.leading_slot_reserved: f...",
        "text_area.trailing_slot_reserved: ...",
    ] {
        assert!(
            rows.iter().any(|row| row.contains(option)),
            "missing text-area inspector row for {option}"
        );
    }
    Ok(())
}

#[test]
fn text_area_presets_render_distinct_multiline_bodies() {
    let chat = StorybookVisual.render_preset(DARK_THEME, PAGE, CHAT_PRESET, 0);
    let newline = StorybookVisual.render_preset(DARK_THEME, PAGE, NEWLINE_PRESET, 0);
    let wrap = StorybookVisual.render_preset(DARK_THEME, PAGE, WRAP_PRESET, 0);
    let resize = StorybookVisual.render_preset(DARK_THEME, PAGE, RESIZE_PRESET, 0);
    let auto_grow = StorybookVisual.render_preset(DARK_THEME, PAGE, AUTO_GROW_PRESET, 0);
    let vertical = StorybookVisual.render_preset(DARK_THEME, PAGE, VERTICAL_SCROLL_PRESET, 0);
    let horizontal = StorybookVisual.render_preset(DARK_THEME, PAGE, HORIZONTAL_SCROLL_PRESET, 0);
    let tab = StorybookVisual.render_preset(DARK_THEME, PAGE, TAB_BEHAVIOR_PRESET, 0);
    let vertical_bar =
        StorybookVisual.render_preset(DARK_THEME, PAGE, VERTICAL_SCROLLBAR_PRESET, 0);
    let horizontal_bar =
        StorybookVisual.render_preset(DARK_THEME, PAGE, HORIZONTAL_SCROLLBAR_PRESET, 0);
    let leading_slot = StorybookVisual.render_preset(DARK_THEME, PAGE, LEADING_SLOT_PRESET, 0);
    let trailing_buttons =
        StorybookVisual.render_preset(DARK_THEME, PAGE, TRAILING_BUTTONS_PRESET, 0);
    let clear_action = StorybookVisual.render_preset(DARK_THEME, PAGE, CLEAR_ACTION_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &chat, &newline) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &newline, &wrap) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &wrap, &resize) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &resize, &auto_grow) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &auto_grow, &vertical) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &vertical, &horizontal) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &horizontal, &tab) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &tab, &vertical_bar) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &vertical_bar, &horizontal_bar) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &horizontal_bar, &leading_slot) > BODY_DIFF_THRESHOLD);
    assert!(
        component_body_pixel_diff(PAGE, &leading_slot, &trailing_buttons) > BODY_DIFF_THRESHOLD
    );
    assert!(
        component_body_pixel_diff(PAGE, &trailing_buttons, &clear_action) > BODY_DIFF_THRESHOLD
    );
}

#[test]
fn text_area_setting_option_updates_multiline_style() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn text_area_preview_action_updates_multiline_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn text_area_trailing_icon_button_emits_callback_action() {
    let mut state = window_interaction::StorybookWindowState {
        selected_page: PAGE,
        preset_index: TRAILING_BUTTONS_PRESET,
        ..window_interaction::StorybookWindowState::default()
    };
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let rect = super::dedicated_dod_form_input_live::text_area_trailing_icon_button_rects(
        origin.x, origin.y,
    )[0];

    assert!(window_interaction::apply_click(
        &mut state,
        rect.x + 1,
        rect.y + 1
    ));
    assert_eq!("text_area_icon_button", state.screen_state.last_action);
    assert_eq!(
        "text_area_icon_button_clicked",
        state.screen_state.last_event
    );
    assert_eq!(
        "text_area.trailing_icon_buttons.action",
        state.screen_state.last_setting
    );
    assert_eq!("icon_button=clicked", state.screen_state.state_label);
}

#[test]
fn text_area_storybook_uses_external_search_svg_source_and_callback() -> Result<(), String> {
    let examples = crate::StoryCatalog.examples();
    let text_area = examples
        .iter()
        .find(|it| it.page == PAGE)
        .ok_or_else(|| "text-area story example".to_string())?;
    let text_entry = &text_area.tree.root().props().text_entry;

    assert_eq!(
        Some(crate::storybook_svg_fixtures::SEARCH_SVG),
        text_entry
            .leading_slot
            .as_ref()
            .and_then(|slot| slot.icon.as_ref())
            .map(|icon| icon.svg_source.as_str())
    );
    assert_eq!(
        Some("text_area.clear"),
        text_entry
            .trailing_icon_buttons
            .first()
            .and_then(|slot| slot.action.as_ref())
            .map(|action| action.callback.as_str())
    );
    Ok(())
}

#[test]
fn text_area_light_and_dark_surfaces_use_theme_tokens() {
    assert_text_area_tokens(DARK_THEME, ThemeSnapshot::dark());
    assert_text_area_tokens(LIGHT_THEME, ThemeSnapshot::light());
}

fn inspector_setting_rows() -> Result<Vec<String>, String> {
    let examples = crate::StoryCatalog.examples();
    let example = examples
        .iter()
        .find(|it| it.page == PAGE)
        .ok_or_else(|| "text-area story example".to_string())?;
    let screen_state = StorybookScreenState::default();
    let scenario = ScenarioContext {
        selected_page: PAGE,
        selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
        preset_index: CHAT_PRESET,
        preset_tab_scroll_x: 0,
        tree_expansion: Default::default(),
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        screen_state: &screen_state,
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
    };

    Ok(inspector_rows::settings_rows(
        example.tree.root(),
        example,
        scenario,
    ))
}

fn assert_text_area_tokens(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, CHAT_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let area_x = rect.x + TEXT_AREA_X_OFFSET;
    let area_y = rect.y + TEXT_AREA_Y_OFFSET;

    assert_eq!(Some(colors.border), pixel_at(&canvas, area_x, area_y));
    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            area_x + TEXT_AREA_FILL_SAMPLE_X_OFFSET,
            area_y + TEXT_AREA_FILL_SAMPLE_Y_OFFSET
        )
    );
}

#[test]
fn text_area_resize_and_scroll_presets_render_only_when_enabled() {
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let chat = StorybookVisual.render_preset(DARK_THEME, PAGE, CHAT_PRESET, 0);
    let resize = StorybookVisual.render_preset(DARK_THEME, PAGE, RESIZE_PRESET, 0);
    let vertical = StorybookVisual.render_preset(DARK_THEME, PAGE, VERTICAL_SCROLL_PRESET, 0);
    let horizontal = StorybookVisual.render_preset(DARK_THEME, PAGE, HORIZONTAL_SCROLL_PRESET, 0);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let grip_x = rect.x + TEXT_AREA_RESIZE_GRIP_X_OFFSET;
    let grip_y = rect.y + TEXT_AREA_RESIZE_GRIP_Y_OFFSET;

    assert_ne!(Some(colors.accent), pixel_at(&chat, grip_x, grip_y));
    assert_eq!(Some(colors.accent), pixel_at(&resize, grip_x, grip_y));
    assert!(component_body_pixel_diff(PAGE, &chat, &vertical) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &chat, &horizontal) > BODY_DIFF_THRESHOLD);
}
