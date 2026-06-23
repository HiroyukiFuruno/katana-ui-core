use super::super::button_operation::{StorybookButtonOperation, button_operation_at};
use super::super::{
    StorybookCursorStyle, StorybookWindowState, apply_click, cursor_style_at_for_test,
};
use crate::catalog::StoryCatalog;
use crate::test_assert::KucTestExpect;
use crate::visual::preview_detail;
use crate::visual::visual_interaction_test_support::component_body_pixel_diff;
use crate::visual::{
    dedicated_dod_form_binary_choice_live, layout_metrics, render, storybook_ui_option_contract,
};
use std::collections::BTreeSet;

const HERO_X: usize = preview_detail::HERO_PREVIEW_X_FOR_TEST;
const HERO_Y: usize = preview_detail::HERO_PREVIEW_Y_FOR_TEST;
const CLICK_CENTER: usize = 2;
const DARK_THEME: &str = "dark";

#[test]
fn radio_hit_target_includes_mark_label_and_row() {
    let mut state = StorybookWindowState {
        selected_page: "radio",
        ..StorybookWindowState::default()
    };
    let row = dedicated_dod_form_binary_choice_live::radio_row_rect(0, HERO_X, HERO_Y);
    let mark = dedicated_dod_form_binary_choice_live::radio_mark_rect(0, HERO_X, HERO_Y);
    let label = dedicated_dod_form_binary_choice_live::radio_label_rect(0, HERO_X, HERO_Y);
    let action_rect = preview_detail::component_action_hit_rect("radio");

    for (index, point) in [
        (mark.x + CLICK_CENTER, mark.y + CLICK_CENTER),
        (label.x + CLICK_CENTER, label.y + CLICK_CENTER),
        (row.x + CLICK_CENTER, row.y + CLICK_CENTER),
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(
            Some(StorybookButtonOperation::RadioSelect),
            button_operation_at(&state, point.0, point.1)
        );
        assert_eq!(
            StorybookCursorStyle::PointingHand,
            cursor_style_at_for_test(&state, point.0, point.1)
        );
        assert!(action_rect.contains(point.0, point.1));
        assert!(apply_click(&mut state, point.0, point.1));
        assert_eq!(index + 1, state.screen_state.action_count);
    }
    assert_eq!("radio_select", state.screen_state.last_action);
    assert_eq!("radio_selected", state.screen_state.last_event);
    assert!(state.screen_state.is_radio_selected());
}

#[test]
fn radio_hit_target_rect_stays_inside_preview_body() {
    let hit_rect = preview_detail::component_action_hit_rect("radio");
    let row = dedicated_dod_form_binary_choice_live::radio_row_rect(0, HERO_X, HERO_Y);
    let read = dedicated_dod_form_binary_choice_live::radio_state_read_button_rect(HERO_X, HERO_Y);
    let select = dedicated_dod_form_binary_choice_live::radio_select_button_rect(HERO_X, HERO_Y);
    let reset = dedicated_dod_form_binary_choice_live::radio_reset_button_rect(HERO_X, HERO_Y);

    assert!(hit_rect.inside_content());
    assert!(hit_rect.overlaps(row));
    assert!(hit_rect.overlaps(read));
    assert!(hit_rect.overlaps(select));
    assert!(hit_rect.overlaps(reset));
    assert!(!read.overlaps(select));
    assert!(!select.overlaps(reset));
    assert!(
        layout_metrics::button_setting_hit_rect().x > hit_rect.right(),
        "inspector setting row must not overlap radio preview body"
    );
}

#[test]
fn radio_control_buttons_apply_expected_actions_and_state_changes() {
    let mut state = StorybookWindowState {
        selected_page: "radio",
        ..StorybookWindowState::default()
    };
    let read = dedicated_dod_form_binary_choice_live::radio_state_read_button_rect(HERO_X, HERO_Y);
    let select = dedicated_dod_form_binary_choice_live::radio_select_button_rect(HERO_X, HERO_Y);
    let reset = dedicated_dod_form_binary_choice_live::radio_reset_button_rect(HERO_X, HERO_Y);
    let row = dedicated_dod_form_binary_choice_live::radio_row_rect(0, HERO_X, HERO_Y);

    assert!(apply_click(
        &mut state,
        read.x + CLICK_CENTER,
        read.y + CLICK_CENTER
    ));
    assert_eq!("radio_state_read", state.screen_state.last_action);
    assert_eq!("selected_read", state.screen_state.last_event);
    assert_eq!("before=false after=false", state.screen_state.state_label);
    assert!(!state.screen_state.is_radio_selected());

    assert!(apply_click(
        &mut state,
        select.x + CLICK_CENTER,
        select.y + CLICK_CENTER
    ));
    assert_eq!("radio_select", state.screen_state.last_action);
    assert_eq!("radio_selected", state.screen_state.last_event);
    assert_eq!("before=false after=true", state.screen_state.state_label);
    assert!(state.screen_state.is_radio_selected());

    assert!(apply_click(
        &mut state,
        row.x + CLICK_CENTER,
        row.y + CLICK_CENTER
    ));
    assert_eq!("radio_select", state.screen_state.last_action);
    assert_eq!("before=true after=true", state.screen_state.state_label);
    assert!(state.screen_state.is_radio_selected());

    assert!(apply_click(
        &mut state,
        reset.x + CLICK_CENTER,
        reset.y + CLICK_CENTER
    ));
    assert_eq!("radio_reset", state.screen_state.last_action);
    assert_eq!("radio_selected", state.screen_state.last_event);
    assert_eq!("before=true after=false", state.screen_state.state_label);
    assert!(!state.screen_state.is_radio_selected());
}

#[test]
fn radio_visual_and_catalog_use_same_typed_action_names() -> Result<(), String> {
    let radio = StoryCatalog
        .examples()
        .into_iter()
        .find(|it| it.page == "radio")
        .ok_or_else(|| "radio story missing".to_string())?;
    let catalog_actions: BTreeSet<String> = radio
        .callback_logs
        .iter()
        .map(|it| it.action.clone())
        .filter(|it| {
            matches!(
                it.as_str(),
                "radio_state_read" | "radio_select" | "radio_reset"
            )
        })
        .collect();

    let mut state = StorybookWindowState {
        selected_page: "radio",
        ..StorybookWindowState::default()
    };
    let read = dedicated_dod_form_binary_choice_live::radio_state_read_button_rect(HERO_X, HERO_Y);
    let select = dedicated_dod_form_binary_choice_live::radio_select_button_rect(HERO_X, HERO_Y);
    let reset = dedicated_dod_form_binary_choice_live::radio_reset_button_rect(HERO_X, HERO_Y);
    let mut visual_actions: BTreeSet<String> = BTreeSet::new();
    for point in [
        (read.x + CLICK_CENTER, read.y + CLICK_CENTER),
        (select.x + CLICK_CENTER, select.y + CLICK_CENTER),
        (reset.x + CLICK_CENTER, reset.y + CLICK_CENTER),
    ] {
        assert!(apply_click(&mut state, point.0, point.1));
        visual_actions.insert(state.screen_state.last_action.to_string());
    }

    assert_eq!(catalog_actions, visual_actions);
    Ok(())
}

#[test]
fn radio_state_read_and_select_keep_core_state_id_and_selected_in_sync() {
    let mut state = StorybookWindowState {
        selected_page: "radio",
        ..StorybookWindowState::default()
    };
    let read = dedicated_dod_form_binary_choice_live::radio_state_read_button_rect(HERO_X, HERO_Y);
    let select = dedicated_dod_form_binary_choice_live::radio_select_button_rect(HERO_X, HERO_Y);
    let reset = dedicated_dod_form_binary_choice_live::radio_reset_button_rect(HERO_X, HERO_Y);
    let initial_state_id = state.screen_state.radio_state_snapshot().state_id.clone();

    assert!(apply_click(
        &mut state,
        read.x + CLICK_CENTER,
        read.y + CLICK_CENTER
    ));
    assert_eq!(
        initial_state_id,
        state.screen_state.radio_state_snapshot().state_id
    );
    assert!(!state.screen_state.radio_state_snapshot().checked);

    assert!(apply_click(
        &mut state,
        select.x + CLICK_CENTER,
        select.y + CLICK_CENTER
    ));
    assert_eq!(
        initial_state_id,
        state.screen_state.radio_state_snapshot().state_id
    );
    assert!(state.screen_state.radio_state_snapshot().checked);
    assert_eq!("before=false after=true", state.screen_state.state_label);

    assert!(apply_click(
        &mut state,
        reset.x + CLICK_CENTER,
        reset.y + CLICK_CENTER
    ));
    assert_eq!(
        initial_state_id,
        state.screen_state.radio_state_snapshot().state_id
    );
    assert!(!state.screen_state.radio_state_snapshot().checked);
    assert_eq!("before=true after=false", state.screen_state.state_label);
}

#[test]
fn radio_disabled_option_blocks_select_and_reset_mutation() {
    let mut state = StorybookWindowState {
        selected_page: "radio",
        ..StorybookWindowState::default()
    };
    let disabled_option = storybook_ui_option_contract::options_for_page("radio")
        .iter()
        .find(|option| option.setting == "disabled")
        .cloned()
        .kuc_expect("radio disabled option must exist");
    assert!(
        state
            .screen_state
            .register_binary_choice_contract_setting("radio", disabled_option)
    );
    assert!(state.screen_state.is_radio_disabled());
    let before_state = state.screen_state.clone();
    let before_canvas = render_radio(&state);
    let select = dedicated_dod_form_binary_choice_live::radio_select_button_rect(HERO_X, HERO_Y);
    let reset = dedicated_dod_form_binary_choice_live::radio_reset_button_rect(HERO_X, HERO_Y);

    assert!(apply_click(
        &mut state,
        select.x + CLICK_CENTER,
        select.y + CLICK_CENTER
    ));
    assert!(apply_click(
        &mut state,
        reset.x + CLICK_CENTER,
        reset.y + CLICK_CENTER
    ));
    let after_canvas = render_radio(&state);

    assert_eq!(before_state, state.screen_state);
    assert_eq!(
        0,
        component_body_pixel_diff("radio", &before_canvas, &after_canvas)
    );
}

#[test]
fn radio_group_selection_keeps_only_one_selected_index() {
    let mut state = StorybookWindowState {
        selected_page: "radio",
        ..StorybookWindowState::default()
    };
    let first = dedicated_dod_form_binary_choice_live::radio_row_rect(0, HERO_X, HERO_Y);
    let second = dedicated_dod_form_binary_choice_live::radio_row_rect(1, HERO_X, HERO_Y);

    assert!(apply_click(
        &mut state,
        first.x + CLICK_CENTER,
        first.y + CLICK_CENTER
    ));
    assert!(state.screen_state.radio_state_snapshot().checked);
    assert!(
        state
            .screen_state
            .radio_state_snapshot()
            .interaction
            .has_selection
    );
    assert_eq!(
        0,
        state
            .screen_state
            .radio_state_snapshot()
            .interaction
            .selected_index
    );
    let first_canvas = render_radio(&state);

    assert_eq!(
        Some(StorybookButtonOperation::RadioSelectIndex(1)),
        button_operation_at(&state, second.x + CLICK_CENTER, second.y + CLICK_CENTER)
    );
    assert!(apply_click(
        &mut state,
        second.x + CLICK_CENTER,
        second.y + CLICK_CENTER
    ));
    assert!(state.screen_state.radio_state_snapshot().checked);
    assert!(
        state
            .screen_state
            .radio_state_snapshot()
            .interaction
            .has_selection
    );
    assert_eq!(
        1,
        state
            .screen_state
            .radio_state_snapshot()
            .interaction
            .selected_index
    );
    let second_canvas = render_radio(&state);

    assert!(component_body_pixel_diff("radio", &first_canvas, &second_canvas) > 0);
}

fn render_radio(state: &StorybookWindowState) -> super::super::super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        "radio",
        state.preset_index,
        state.screen_state.clone(),
    )
}
