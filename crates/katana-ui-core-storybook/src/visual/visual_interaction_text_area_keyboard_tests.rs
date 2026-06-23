use super::layout_metrics::LayoutRect;
use super::visual_interaction_test_support::{
    component_body_pixel_diff, pixel_at, rect_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, TextAreaKey, apply_click};
use super::{palette, preview_detail, render, window_interaction};
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::theme::ThemeSnapshot;

const PAGE: &str = "text-area";
const BODY_DIFF_THRESHOLD: usize = 80;
const CARET_HIDDEN_FRAME: usize = 30;
const CLEAR_ACTION_PRESET: usize = 12;
const DISABLED_PRESET: usize = 16;
const RUNTIME_MARKER_HEIGHT: usize = 4;
const LABEL_SIZE: f32 = 10.0;

#[test]
fn text_area_field_accepts_keyboard_input_after_focus() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let before = render_with_state(&state);
    let field = text_area_field_rect();

    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    assert!(state.screen_state.text_area_focused());
    assert!(window_interaction::apply_text_area_key(
        &mut state,
        TextAreaKey::Character('k')
    ));
    assert!(window_interaction::apply_text_area_key(
        &mut state,
        TextAreaKey::Character('u')
    ));
    assert!(window_interaction::apply_text_area_key(
        &mut state,
        TextAreaKey::Character('c')
    ));

    let after = render_with_state(&state);
    assert!(state.screen_state.text_area_value().ends_with("kuc"));
    assert_eq!("text_area_type", state.screen_state.last_action);
    assert_eq!("text_area_changed", state.screen_state.last_event);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn text_area_keyboard_requires_focus_backspaces_and_commits_enter() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let initial_value = state.screen_state.text_area_value().to_string();

    assert!(!window_interaction::apply_text_area_key(
        &mut state,
        TextAreaKey::Character('x')
    ));
    assert_eq!(initial_value, state.screen_state.text_area_value());

    let field = text_area_field_rect();
    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    assert!(window_interaction::apply_text_area_key(
        &mut state,
        TextAreaKey::Character('x')
    ));
    assert!(window_interaction::apply_text_area_key(
        &mut state,
        TextAreaKey::Backspace
    ));
    assert_eq!(initial_value, state.screen_state.text_area_value());
    assert!(window_interaction::apply_text_area_key(
        &mut state,
        TextAreaKey::Submit
    ));
    assert_eq!("text_area_submit", state.screen_state.last_action);
    assert_eq!("text_area_submitted", state.screen_state.last_event);
    assert_eq!("value=typed", state.screen_state.state_label);
}

#[test]
fn text_area_keyboard_state_is_preset_scoped() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let initial_value = state.screen_state.text_area_value().to_string();
    let field = text_area_field_rect();

    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    assert!(window_interaction::apply_text_area_key(
        &mut state,
        TextAreaKey::Character('z')
    ));
    assert!(state.screen_state.text_area_value().ends_with('z'));

    state.select_preset(1);
    assert_eq!(initial_value, state.screen_state.text_area_value());

    state.select_preset(0);
    assert!(state.screen_state.text_area_value().ends_with('z'));
}

#[test]
fn text_area_disabled_blocks_keyboard_backspace_and_clear_storybook_routes() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        preset_index: DISABLED_PRESET,
        ..StorybookWindowState::default()
    };
    let field = text_area_field_rect();

    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    let disabled_value = state.screen_state.text_area_value().to_string();
    assert!(window_interaction::apply_text_area_key(
        &mut state,
        TextAreaKey::Character('x')
    ));
    assert_eq!(disabled_value, state.screen_state.text_area_value());
    assert_disabled_blocked(&state);

    assert!(window_interaction::apply_text_area_key(
        &mut state,
        TextAreaKey::Backspace
    ));
    assert_eq!(disabled_value, state.screen_state.text_area_value());
    assert_disabled_blocked(&state);

    state.select_preset(CLEAR_ACTION_PRESET);
    state.screen_state.register_settings_contract_change(
        PAGE,
        super::storybook_ui_option_contract::StorybookUiOptionContract::new(
            "text_area.disabled",
            "false",
            "true",
        ),
    );
    let clear = clear_action_rect();
    assert!(apply_click(&mut state, clear.x + 1, clear.y + 1));
    assert_eq!(disabled_value, state.screen_state.text_area_value());
    assert_disabled_blocked(&state);
}

#[test]
fn text_area_focus_cursor_blinks_inside_field() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let value = state.screen_state.text_area_value();
    let line = value.lines().last().unwrap_or_default();
    let line_index = value.lines().count().saturating_sub(1);
    let caret = super::dedicated_dod_form_input_live::text_area_caret_rect(
        origin.x,
        origin.y,
        measured_body_width(line),
        line_index,
    );
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());

    let inactive = render_with_state(&state);
    assert_eq!(Some(colors.surface), pixel_at(&inactive, caret.x, caret.y));

    let field = text_area_field_rect();
    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    let active = render_with_state(&state);
    assert_eq!(Some(colors.accent), pixel_at(&active, caret.x, caret.y));

    assert!(
        state
            .screen_state
            .update_text_area_caret_visibility(CARET_HIDDEN_FRAME)
    );
    let hidden = render_with_state(&state);
    assert_eq!(Some(colors.surface), pixel_at(&hidden, caret.x, caret.y));
}

#[test]
fn text_area_caret_touches_measured_line_edge() {
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let line = "abcdefb";
    let value_width = measured_body_width(line);
    let caret = super::dedicated_dod_form_input_live::text_area_caret_rect(
        origin.x,
        origin.y,
        value_width,
        0,
    );

    assert_eq!(
        origin.x + super::dedicated_dod_form_input_live::TEXT_AREA_LINE_X + value_width,
        caret.x
    );
}

#[test]
fn text_area_status_and_runtime_marker_stay_inside_frame() {
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let frame = LayoutRect::new(
        origin.x,
        origin.y,
        super::dedicated_dod_common::AREA_WIDTH,
        super::dedicated_dod_common::AREA_HEIGHT,
    );
    let field = super::dedicated_dod_form_input_live::text_area_rect(origin.x, origin.y);

    assert_eq!(super::dedicated_dod_common::AREA_HEIGHT, origin.height);
    assert!(rect_inside(field, frame));
    for rect in super::dedicated_dod_form_input_live::text_area_status_rects(origin.x, origin.y) {
        assert!(rect_inside(rect, frame));
        assert!(!rect.overlaps(field));
    }

    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let before = render_with_state(&state);
    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    assert!(window_interaction::apply_text_area_key(
        &mut state,
        TextAreaKey::Character('x')
    ));
    let after = render_with_state(&state);
    let marker = LayoutRect::new(
        origin.x,
        origin.bottom() - RUNTIME_MARKER_HEIGHT,
        origin.width,
        1,
    );
    let outside = LayoutRect::new(origin.x, origin.bottom(), origin.width, 1);

    assert!(rect_pixel_diff(marker, &before, &after) > 0);
    assert_eq!(0, rect_pixel_diff(outside, &before, &after));
}

fn render_with_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn text_area_field_rect() -> LayoutRect {
    let rect = preview_detail::component_action_hit_rect(PAGE);
    super::dedicated_dod_form_input_live::text_area_rect(rect.x, rect.y)
}

fn clear_action_rect() -> LayoutRect {
    let rect = preview_detail::component_action_hit_rect(PAGE);
    super::dedicated_dod_form_input_live::text_area_clear_action_rect(rect.x, rect.y)
}

fn assert_disabled_blocked(state: &StorybookWindowState) {
    assert_eq!("text_area_disabled_blocked", state.screen_state.last_action);
    assert_eq!("text_area_disabled_ignored", state.screen_state.last_event);
    assert_eq!("text_area.disabled", state.screen_state.last_setting);
    assert_eq!("true", state.screen_state.last_setting_value);
    assert_eq!("disabled=true", state.screen_state.state_label);
}

fn measured_body_width(value: &str) -> usize {
    let facade = UiCoreFacade::default();
    let text = super::text::TextRenderer::load(&facade, "body");
    text.measure_width(value, LABEL_SIZE)
}

fn rect_inside(inner: LayoutRect, outer: LayoutRect) -> bool {
    inner.x >= outer.x
        && inner.y >= outer.y
        && inner.right() <= outer.right()
        && inner.bottom() <= outer.bottom()
}
