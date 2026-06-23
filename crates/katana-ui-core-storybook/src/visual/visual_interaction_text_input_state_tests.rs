use super::preview_detail;
use super::window_interaction::{self, StorybookWindowState, TextInputKey, apply_click};

const PAGE: &str = "text-input";
const VALUE_PRESET: usize = 0;
const PLACEHOLDER_PRESET: usize = 3;

#[test]
fn text_input_preset_tab_switching_keeps_runtime_state_isolated() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let field = text_input_field_rect();

    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    assert!(window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Character('a')
    ));
    let value_preset_value = state.screen_state.text_input_value().to_string();
    assert!(state.screen_state.text_input_focused());
    assert!(state.screen_state.text_input_caret_visible());

    state.select_preset(PLACEHOLDER_PRESET);
    assert_ne!(value_preset_value, state.screen_state.text_input_value());
    assert!(!state.screen_state.text_input_focused());
    assert!(!state.screen_state.text_input_caret_visible());

    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    assert!(window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Character('z')
    ));
    let placeholder_preset_value = state.screen_state.text_input_value().to_string();
    assert_ne!(value_preset_value, placeholder_preset_value);
    assert!(state.screen_state.text_input_focused());
    assert!(state.screen_state.text_input_caret_visible());

    state.select_preset(VALUE_PRESET);
    assert_eq!(value_preset_value, state.screen_state.text_input_value());
    assert!(state.screen_state.text_input_focused());
    assert!(state.screen_state.text_input_caret_visible());

    state.select_preset(PLACEHOLDER_PRESET);
    assert_eq!(
        placeholder_preset_value,
        state.screen_state.text_input_value()
    );
    assert!(state.screen_state.text_input_focused());
    assert!(state.screen_state.text_input_caret_visible());
}

#[test]
fn text_input_keyboard_routes_to_selected_instance_state() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let field = text_input_field_rect();

    state.select_instance("text-input.primary");
    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    assert!(window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Character('1')
    ));
    let primary = state
        .screen_state
        .text_input_value_for("text-input.primary")
        .to_string();

    state.select_instance("text-input.secondary");
    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    assert!(window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Character('2')
    ));
    let secondary = state
        .screen_state
        .text_input_value_for("text-input.secondary")
        .to_string();

    state.select_instance("text-input.primary");
    assert_eq!(
        primary,
        state
            .screen_state
            .text_input_value_for("text-input.primary")
    );
    assert!(primary.ends_with('1'));

    state.select_instance("text-input.secondary");
    assert_eq!(
        secondary,
        state
            .screen_state
            .text_input_value_for("text-input.secondary")
    );
    assert!(
        state
            .screen_state
            .text_input_value_for("text-input.secondary")
            .ends_with('2')
    );
    assert_ne!(primary, secondary);
}

fn text_input_field_rect() -> super::layout_metrics::LayoutRect {
    let rect = preview_detail::component_action_hit_rect(PAGE);
    super::dedicated_dod_form_input_live::search_field_rect(rect.x, rect.y)
}
