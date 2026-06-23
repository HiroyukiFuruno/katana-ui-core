use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_hover_at, apply_settings_list_scroll_for_audit, focus_clickable_at_for_audit,
};
use super::{layout_metrics, preview_detail, render, storybook_ui_option_contract};

const PAGE: &str = "settings-list";
const PRIMARY_INSTANCE: &str = "settings-list.primary";
const SECONDARY_INSTANCE: &str = "settings-list.secondary";

#[test]
fn settings_list_inspector_options_mutate_field_control_and_reset_semantic_state()
-> Result<(), String> {
    for &(setting, expected_state) in expected_states() {
        let mut state = page_state();
        let before = render_state(&state);
        click_option(&mut state, setting)?;
        let after = render_state(&state);

        assert_eq!(setting, state.screen_state.last_setting);
        assert_eq!(expected_state, state.screen_state.state_label);
        assert_settings_list_runtime(setting, &state);
        assert!(component_body_pixel_diff(PAGE, &before, &after) > 0);
    }
    Ok(())
}

#[test]
fn settings_list_window_interaction_keeps_query_field_collapse_and_reset_instance_isolated()
-> Result<(), String> {
    let mut state = page_state();
    state.select_instance(PRIMARY_INSTANCE);
    let before = render_state(&state);

    click_component(&mut state);

    assert_eq!(1, state.screen_state.action_count);
    assert_eq!(
        "settings_filter_update_collapse",
        state.screen_state.last_action
    );
    assert_eq!("settings_field_changed", state.screen_state.last_event);
    assert_eq!("dirty=font-size", state.screen_state.state_label);
    assert!(state.screen_state.settings_list.has_dirty_font_size());
    assert_eq!(
        "settings_update_field",
        state.screen_state.settings_list.callback_action()
    );

    click_option(&mut state, "settings_list.query")?;
    assert!(state.screen_state.settings_list.has_query_filter());
    click_option(&mut state, "settings_list.default_collapsed")?;
    assert!(
        state
            .screen_state
            .settings_list
            .has_collapsed_chat_section()
    );
    click_option(&mut state, "settings_list.reset")?;
    assert!(!state.screen_state.settings_list.has_dirty_font_size());
    assert_eq!(
        "settings_reset_field",
        state.screen_state.settings_list.callback_action()
    );
    let primary = state.screen_state.clone();
    assert!(component_body_pixel_diff(PAGE, &before, &render_state(&state)) > 0);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!(0, state.screen_state.action_count);
    assert_eq!("idle", state.screen_state.state_label);
    assert!(!state.screen_state.settings_list.has_query_filter());
    assert!(
        !state
            .screen_state
            .settings_list
            .has_collapsed_chat_section()
    );
    assert!(!state.screen_state.settings_list.has_dirty_font_size());

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary, state.screen_state);
    Ok(())
}

#[test]
fn settings_list_live_hover_focus_keyboard_and_scroll_use_core_actions() {
    let target = preview_detail::component_action_hit_rect(PAGE);
    let mut hover = page_state();
    let before_hover = render_state(&hover);
    assert!(apply_hover_at(&mut hover, target.x + 1, target.y + 1));
    let after_hover = render_state(&hover);
    assert_eq!("settings_hover_field", hover.screen_state.last_action);
    assert_eq!("hover_start", hover.screen_state.last_event);
    assert_eq!("hover=app.font-size", hover.screen_state.state_label);
    assert!(hover.screen_state.settings_list.hovered);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let mut keyboard = page_state();
    assert!(focus_clickable_at_for_audit(
        &mut keyboard,
        target.x + 1,
        target.y + 1
    ));
    assert_eq!("settings_focus_field", keyboard.screen_state.last_action);
    assert_eq!("settings_field_focused", keyboard.screen_state.last_event);
    assert_eq!("focus=app.font-size", keyboard.screen_state.state_label);
    let before_key = render_state(&keyboard);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut keyboard));
    let after_key = render_state(&keyboard);
    assert_eq!("settings_keyboard_next", keyboard.screen_state.last_action);
    assert_eq!("settings_field_focused", keyboard.screen_state.last_event);
    assert_eq!("focus=next", keyboard.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_key, &after_key) > 0);

    let mut scroll = page_state();
    let before_scroll = render_state(&scroll);
    assert!(apply_settings_list_scroll_for_audit(
        &mut scroll,
        target.x + 1,
        target.y + 1
    ));
    let after_scroll = render_state(&scroll);
    assert_eq!("settings_scroll", scroll.screen_state.last_action);
    assert_eq!("scroll_by", scroll.screen_state.last_event);
    assert_eq!("scroll=1", scroll.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_scroll, &after_scroll) > 0);
}

fn expected_states() -> &'static [(&'static str, &'static str)] {
    &[
        (
            "settings_list.label",
            "settings_list.label=Workspace settings",
        ),
        ("settings_list.density", "settings_list.density=Compact"),
        (
            "settings_list.dirty_visualization",
            "settings_list.dirty=Highlight",
        ),
        ("settings_list.query", "settings_list.query=format"),
        ("settings_list.sections", "settings_list.sections=app+lint"),
        (
            "settings_list.section_label",
            "settings_list.section.label=Editor",
        ),
        (
            "settings_list.section_description",
            "settings_list.section.description=visible",
        ),
        (
            "settings_list.section_icon",
            "settings_list.section.icon=gear",
        ),
        ("settings_list.field_count", "settings_list.field.count=5"),
        (
            "settings_list.section_footer",
            "settings_list.section.footer=policy",
        ),
        (
            "settings_list.section_collapsible",
            "settings_list.section.collapsible=true",
        ),
        (
            "settings_list.default_collapsed",
            "settings_list.section.collapsed=true",
        ),
        (
            "settings_list.field_label",
            "settings_list.field.label=Font size",
        ),
        (
            "settings_list.field_description",
            "settings_list.field.description=visible",
        ),
        (
            "settings_list.control_kind",
            "settings_list.control.kind=Number",
        ),
        (
            "settings_list.control_options",
            "settings_list.control.options=4",
        ),
        (
            "settings_list.custom_control",
            "settings_list.control.custom=button",
        ),
        ("settings_list.set_value", "settings_list.value=changed"),
        ("settings_list.reset", "settings_list.reset=default"),
    ]
}

fn assert_settings_list_runtime(setting: &str, state: &StorybookWindowState) {
    let settings_list = &state.screen_state.settings_list;
    let options = settings_list.option_state();
    match setting {
        "settings_list.label" => assert!(options.label_workspace),
        "settings_list.density" => assert!(options.density_compact),
        "settings_list.dirty_visualization" => assert!(options.dirty_highlight),
        "settings_list.query" => assert!(settings_list.has_query_filter()),
        "settings_list.sections" => assert!(options.sections_app_lint),
        "settings_list.section_label" => assert!(options.section_label_editor),
        "settings_list.section_description" => assert!(options.section_description_visible),
        "settings_list.section_icon" => assert!(options.section_icon_gear),
        "settings_list.field_count" => assert_eq!(5, options.field_count),
        "settings_list.section_footer" => assert!(options.section_footer_policy),
        "settings_list.section_collapsible" => assert!(options.section_collapsible),
        "settings_list.default_collapsed" => {
            assert!(options.default_collapsed);
            assert!(settings_list.has_collapsed_chat_section());
        }
        "settings_list.field_label" => assert!(options.field_label_font_size),
        "settings_list.field_description" => assert!(options.field_description_visible),
        "settings_list.control_kind" => assert!(options.control_kind_number),
        "settings_list.control_options" => assert_eq!(4, options.control_option_count),
        "settings_list.custom_control" => assert!(options.custom_control_button),
        "settings_list.set_value" => {
            assert!(options.value_changed);
            assert!(settings_list.has_dirty_font_size());
        }
        "settings_list.reset" => {
            assert!(options.reset_default);
            assert!(!settings_list.has_dirty_font_size());
        }
        _ => {}
    }
}

fn click_option(state: &mut StorybookWindowState, setting: &str) -> Result<(), String> {
    let index = option_index(setting)?;
    let row = layout_metrics::inspector_setting_row_hit_rect(index);

    assert!(apply_click(state, row.x + 1, row.y + 1));
    Ok(())
}

fn click_component(state: &mut StorybookWindowState) {
    let rect = preview_detail::component_action_hit_rect(PAGE);

    assert!(apply_click(state, rect.x + 1, rect.y + 1));
}

fn option_index(setting: &str) -> Result<usize, String> {
    storybook_ui_option_contract::options_for_page(PAGE)
        .iter()
        .position(|option| option.setting == setting)
        .ok_or_else(|| format!("missing settings-list option `{setting}`"))
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn page_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}
