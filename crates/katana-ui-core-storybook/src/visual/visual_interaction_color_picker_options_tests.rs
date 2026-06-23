use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, preview_detail, render, storybook_ui_option_contract};

const PAGE: &str = "color-picker-rgba";
const PRIMARY_INSTANCE: &str = "color-picker.primary";
const SECONDARY_INSTANCE: &str = "color-picker.secondary";
const READONLY_PRESET_INDEX: usize = 13;
const DISABLED_PRESET_INDEX: usize = 14;

#[test]
fn color_picker_inspector_options_mutate_hue_alpha_block_and_callback_semantic_state()
-> Result<(), String> {
    for &(setting, expected_state) in expected_states() {
        let mut state = page_state();
        let before = render_state(&state);
        click_option(&mut state, setting)?;
        let after = render_state(&state);

        assert_eq!(setting, state.screen_state.last_setting);
        assert_eq!(expected_state, state.screen_state.state_label);
        assert_color_picker_runtime(setting, &state);
        assert!(component_body_pixel_diff(PAGE, &before, &after) > 0);
    }
    Ok(())
}

#[test]
fn color_picker_window_interaction_keeps_drag_value_callback_and_blocked_state_isolated()
-> Result<(), String> {
    let mut state = page_state();
    state.select_instance(PRIMARY_INSTANCE);
    let before = render_state(&state);

    click_component(&mut state);

    assert_eq!(1, state.screen_state.action_count);
    assert_eq!("color_drag", state.screen_state.last_action);
    assert_eq!("rgba_changed", state.screen_state.last_event);
    assert_eq!("rgba=accent", state.screen_state.state_label);
    assert_eq!(
        "rgba(72, 136, 240, 188)",
        state.screen_state.color_picker.rgba_label()
    );
    assert_eq!(226, state.screen_state.color_picker.hue());
    assert_eq!(188, state.screen_state.color_picker.alpha());
    assert_eq!(
        "color_drag",
        state.screen_state.color_picker.callback_action()
    );
    let primary = state.screen_state.clone();
    let after = render_state(&state);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > 0);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!(0, state.screen_state.action_count);
    assert_eq!("idle", state.screen_state.state_label);
    assert_eq!(
        "rgba(64, 128, 255, 204)",
        state.screen_state.color_picker.rgba_label()
    );

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary, state.screen_state);
    Ok(())
}

#[test]
fn color_picker_readonly_and_disabled_preview_clicks_do_not_mutate_color() -> Result<(), String> {
    let mut readonly = page_state();
    readonly.select_preset(READONLY_PRESET_INDEX);
    let readonly_before = render_state(&readonly);

    click_component(&mut readonly);

    assert_eq!(0, readonly.screen_state.action_count);
    assert_eq!(
        "color_picker_readonly_blocked",
        readonly.screen_state.last_action
    );
    assert_eq!(
        "color_picker_write_blocked",
        readonly.screen_state.last_event
    );
    assert_eq!(
        "color_picker.readonly.blocks_writes",
        readonly.screen_state.state_label
    );
    assert_eq!(
        "rgba(64, 128, 255, 204)",
        readonly.screen_state.color_picker.rgba_label()
    );
    assert!(component_body_pixel_diff(PAGE, &readonly_before, &render_state(&readonly)) > 0);

    let mut disabled = page_state();
    disabled.select_preset(DISABLED_PRESET_INDEX);
    let disabled_before = render_state(&disabled);

    click_component(&mut disabled);

    assert_eq!(0, disabled.screen_state.action_count);
    assert_eq!(
        "color_picker_disabled_blocked",
        disabled.screen_state.last_action
    );
    assert_eq!(
        "color_picker_focus_blocked",
        disabled.screen_state.last_event
    );
    assert_eq!(
        "color_picker.disabled.blocks_focus",
        disabled.screen_state.state_label
    );
    assert_eq!(
        "rgba(64, 128, 255, 204)",
        disabled.screen_state.color_picker.rgba_label()
    );
    assert!(component_body_pixel_diff(PAGE, &disabled_before, &render_state(&disabled)) > 0);
    Ok(())
}

fn expected_states() -> &'static [(&'static str, &'static str)] {
    &[
        ("color_picker.rgba", "color_picker.rgba=rgba(64,128,255,.8)"),
        (
            "color_picker.value",
            "color_picker.value=rgba(72,136,240,.74)",
        ),
        ("color_picker.open", "color_picker.open=true"),
        ("color_picker.hue", "color_picker.hue=214"),
        ("color_picker.alpha", "color_picker.alpha=204"),
        ("color_picker.blending", "color_picker.blending=Multiply"),
        (
            "color_picker.color_area",
            "color_picker.color_area=saturation/value",
        ),
        (
            "color_picker.trigger_size",
            "color_picker.trigger.size=Large",
        ),
        ("color_picker.title", "color_picker.title=Brand accent"),
        ("color_picker.rgba_mode", "color_picker.rgba_mode=false"),
        (
            "color_picker.panel_scale_percent",
            "color_picker.panel.scale=100",
        ),
        (
            "color_picker.trigger_border",
            "color_picker.trigger.border=false",
        ),
        (
            "color_picker.eyedropper_callback",
            "color_picker.eyedropper=storybook-eyedropper",
        ),
        (
            "color_picker.readonly",
            "color_picker.readonly.blocks_writes",
        ),
        (
            "color_picker.disabled",
            "color_picker.disabled.blocks_focus",
        ),
    ]
}

fn assert_color_picker_runtime(setting: &str, state: &StorybookWindowState) {
    let color = &state.screen_state.color_picker;
    let options = color.option_state();
    match setting {
        "color_picker.rgba" => {
            assert_eq!("rgba(64, 128, 255, 204)", color.rgba_label());
            assert!(color.has_committed_color());
        }
        "color_picker.value" => {
            assert_eq!("rgba(72, 136, 240, 188)", color.rgba_label());
            assert!(color.has_committed_color());
        }
        "color_picker.open" => assert!(options.panel_open),
        "color_picker.hue" => assert_eq!(214, color.hue()),
        "color_picker.alpha" => assert_eq!(204, color.alpha()),
        "color_picker.blending" => assert!(options.blending_multiply),
        "color_picker.color_area" => assert!(options.color_area_visible),
        "color_picker.trigger_size" => assert!(options.trigger_large),
        "color_picker.title" => assert!(options.title_customized),
        "color_picker.rgba_mode" => assert!(!options.rgba_mode),
        "color_picker.panel_scale_percent" => assert_eq!(100, options.panel_scale_percent),
        "color_picker.trigger_border" => assert!(!options.trigger_border),
        "color_picker.eyedropper_callback" => {
            assert_eq!("color_eyedropper_request", color.callback_action());
        }
        "color_picker.readonly" => assert!(color.blocks_writes()),
        "color_picker.disabled" => assert!(color.blocks_focus()),
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
        .ok_or_else(|| format!("missing color-picker option `{setting}`"))
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
