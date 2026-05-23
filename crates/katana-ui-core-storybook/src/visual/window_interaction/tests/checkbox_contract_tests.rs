use super::super::button_operation::{StorybookButtonOperation, button_operation_at};
use super::super::{StorybookWindowState, apply_click};
use crate::catalog::StoryCatalog;
use crate::visual::preview_detail;
use crate::visual::{dedicated_dod_form_binary_choice_live, layout_metrics};
use std::collections::BTreeSet;

const HERO_X: usize = preview_detail::HERO_PREVIEW_X_FOR_TEST;
const HERO_Y: usize = preview_detail::HERO_PREVIEW_Y_FOR_TEST;
const CLICK_CENTER: usize = 2;

#[test]
fn checkbox_hit_target_includes_mark_label_and_row() {
    let mut state = StorybookWindowState {
        selected_page: "checkbox",
        ..StorybookWindowState::default()
    };
    let row = dedicated_dod_form_binary_choice_live::checkbox_row_rect(0, HERO_X, HERO_Y);
    let mark = dedicated_dod_form_binary_choice_live::checkbox_mark_rect(0, HERO_X, HERO_Y);
    let label = dedicated_dod_form_binary_choice_live::checkbox_label_rect(0, HERO_X, HERO_Y);
    let action_rect = preview_detail::component_action_hit_rect("checkbox");

    for (index, point) in [
        (mark.x + CLICK_CENTER, mark.y + CLICK_CENTER),
        (label.x + CLICK_CENTER, label.y + CLICK_CENTER),
        (row.x + CLICK_CENTER, row.y + CLICK_CENTER),
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(
            Some(StorybookButtonOperation::PreviewComponent),
            button_operation_at(&state, point.0, point.1)
        );
        assert!(action_rect.contains(point.0, point.1));
        assert!(apply_click(&mut state, point.0, point.1));
        assert_eq!(index + 1, state.screen_state.action_count);
    }
    assert_eq!(3, state.screen_state.action_count);
    assert_eq!("checkbox_toggle", state.screen_state.last_action);
    assert_eq!("checked_changed", state.screen_state.last_event);
    assert_eq!("before=false after=true", state.screen_state.state_label);
    assert!(state.screen_state.is_checkbox_checked());
}

#[test]
fn checkbox_hit_target_rect_stays_inside_preview_body() {
    let hit_rect = preview_detail::component_action_hit_rect("checkbox");
    let row = dedicated_dod_form_binary_choice_live::checkbox_row_rect(0, HERO_X, HERO_Y);
    let read =
        dedicated_dod_form_binary_choice_live::checkbox_state_read_button_rect(HERO_X, HERO_Y);
    let toggle = dedicated_dod_form_binary_choice_live::checkbox_toggle_button_rect(HERO_X, HERO_Y);
    let reset = dedicated_dod_form_binary_choice_live::checkbox_reset_button_rect(HERO_X, HERO_Y);

    assert!(hit_rect.inside_content());
    assert!(hit_rect.overlaps(row));
    assert!(hit_rect.overlaps(read));
    assert!(hit_rect.overlaps(toggle));
    assert!(hit_rect.overlaps(reset));
    assert!(!read.overlaps(toggle));
    assert!(!toggle.overlaps(reset));
    assert!(
        layout_metrics::button_setting_hit_rect().x > hit_rect.right(),
        "inspector setting row must not overlap checkbox preview body"
    );
}

#[test]
fn checkbox_control_buttons_apply_expected_actions_and_state_changes() {
    let mut state = StorybookWindowState {
        selected_page: "checkbox",
        ..StorybookWindowState::default()
    };
    let read =
        dedicated_dod_form_binary_choice_live::checkbox_state_read_button_rect(HERO_X, HERO_Y);
    let toggle = dedicated_dod_form_binary_choice_live::checkbox_toggle_button_rect(HERO_X, HERO_Y);
    let reset = dedicated_dod_form_binary_choice_live::checkbox_reset_button_rect(HERO_X, HERO_Y);
    let row = dedicated_dod_form_binary_choice_live::checkbox_row_rect(0, HERO_X, HERO_Y);

    assert!(apply_click(
        &mut state,
        read.x + CLICK_CENTER,
        read.y + CLICK_CENTER
    ));
    assert_eq!("checkbox_state_read", state.screen_state.last_action);
    assert_eq!("checked_read", state.screen_state.last_event);
    assert_eq!("before=false after=false", state.screen_state.state_label);
    assert!(!state.screen_state.is_checkbox_checked());

    assert!(apply_click(
        &mut state,
        toggle.x + CLICK_CENTER,
        toggle.y + CLICK_CENTER
    ));
    assert_eq!("checkbox_toggle", state.screen_state.last_action);
    assert_eq!("checked_changed", state.screen_state.last_event);
    assert_eq!("before=false after=true", state.screen_state.state_label);
    assert!(state.screen_state.is_checkbox_checked());

    assert!(apply_click(
        &mut state,
        row.x + CLICK_CENTER,
        row.y + CLICK_CENTER
    ));
    assert_eq!("checkbox_toggle", state.screen_state.last_action);
    assert_eq!("before=true after=false", state.screen_state.state_label);
    assert!(!state.screen_state.is_checkbox_checked());

    assert!(apply_click(
        &mut state,
        reset.x + CLICK_CENTER,
        reset.y + CLICK_CENTER
    ));
    assert_eq!("checkbox_reset", state.screen_state.last_action);
    assert_eq!("checked_changed", state.screen_state.last_event);
    assert_eq!("before=false after=false", state.screen_state.state_label);
    assert!(!state.screen_state.is_checkbox_checked());
}

#[test]
fn checkbox_visual_and_catalog_use_same_typed_action_names() {
    let checkbox = StoryCatalog
        .examples()
        .into_iter()
        .find(|it| it.page == "checkbox")
        .expect("checkbox story missing");
    let catalog_actions: BTreeSet<String> = checkbox
        .callback_logs
        .iter()
        .map(|it| it.action.clone())
        .filter(|it| {
            matches!(
                it.as_str(),
                "checkbox_state_read" | "checkbox_toggle" | "checkbox_reset"
            )
        })
        .collect();

    let mut state = StorybookWindowState {
        selected_page: "checkbox",
        ..StorybookWindowState::default()
    };
    let read =
        dedicated_dod_form_binary_choice_live::checkbox_state_read_button_rect(HERO_X, HERO_Y);
    let toggle = dedicated_dod_form_binary_choice_live::checkbox_toggle_button_rect(HERO_X, HERO_Y);
    let reset = dedicated_dod_form_binary_choice_live::checkbox_reset_button_rect(HERO_X, HERO_Y);
    let mut visual_actions: BTreeSet<String> = BTreeSet::new();
    for point in [
        (read.x + CLICK_CENTER, read.y + CLICK_CENTER),
        (toggle.x + CLICK_CENTER, toggle.y + CLICK_CENTER),
        (reset.x + CLICK_CENTER, reset.y + CLICK_CENTER),
    ] {
        assert!(apply_click(&mut state, point.0, point.1));
        visual_actions.insert(state.screen_state.last_action.to_string());
    }

    assert_eq!(catalog_actions, visual_actions);
}

#[test]
fn checkbox_state_read_and_toggle_keep_core_state_id_and_checked_in_sync() {
    let mut state = StorybookWindowState {
        selected_page: "checkbox",
        ..StorybookWindowState::default()
    };
    let read =
        dedicated_dod_form_binary_choice_live::checkbox_state_read_button_rect(HERO_X, HERO_Y);
    let toggle = dedicated_dod_form_binary_choice_live::checkbox_toggle_button_rect(HERO_X, HERO_Y);
    let reset = dedicated_dod_form_binary_choice_live::checkbox_reset_button_rect(HERO_X, HERO_Y);
    let initial_state_id = state
        .screen_state
        .checkbox_state_snapshot()
        .state_id
        .clone();

    assert!(apply_click(
        &mut state,
        read.x + CLICK_CENTER,
        read.y + CLICK_CENTER
    ));
    assert_eq!(
        initial_state_id,
        state.screen_state.checkbox_state_snapshot().state_id
    );
    assert!(!state.screen_state.checkbox_state_snapshot().checked);

    assert!(apply_click(
        &mut state,
        toggle.x + CLICK_CENTER,
        toggle.y + CLICK_CENTER
    ));
    assert_eq!(
        initial_state_id,
        state.screen_state.checkbox_state_snapshot().state_id
    );
    assert!(state.screen_state.checkbox_state_snapshot().checked);
    assert_eq!("before=false after=true", state.screen_state.state_label);

    assert!(apply_click(
        &mut state,
        reset.x + CLICK_CENTER,
        reset.y + CLICK_CENTER
    ));
    assert_eq!(
        initial_state_id,
        state.screen_state.checkbox_state_snapshot().state_id
    );
    assert!(!state.screen_state.checkbox_state_snapshot().checked);
    assert_eq!("before=true after=false", state.screen_state.state_label);
}
