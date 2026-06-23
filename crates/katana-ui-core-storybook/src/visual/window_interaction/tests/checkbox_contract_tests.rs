use super::super::button_operation::{StorybookButtonOperation, button_operation_at};
use super::super::{
    StorybookCursorStyle, StorybookWindowState, apply_click, cursor_style_at_for_test,
};
use crate::catalog::StoryCatalog;
use crate::visual::preview_detail;
use crate::visual::{Canvas, dedicated_dod_form_binary_choice_live, layout_metrics, render};
use katana_ui_core::atom::Checkbox;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use std::collections::BTreeSet;

const HERO_X: usize = preview_detail::HERO_PREVIEW_X_FOR_TEST;
const HERO_Y: usize = preview_detail::HERO_PREVIEW_Y_FOR_TEST;
const CLICK_CENTER: usize = 2;
const CHECKBOX_CHECKED_PRESET_INDEX: usize = 1;
const CHECKBOX_ACCENT: u32 = 0x569cd6;
const CHECKBOX_GLYPH: u32 = 0xf8fafc;

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
            Some(StorybookButtonOperation::CheckboxToggle(0)),
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
    assert_eq!(3, state.screen_state.action_count);
    assert_eq!("checkbox_toggle", state.screen_state.last_action);
    assert_eq!("checked_changed", state.screen_state.last_event);
    assert_eq!("before=false after=true", state.screen_state.state_label);
    assert!(state.screen_state.is_checkbox_checked());
}

#[test]
fn checkbox_rows_toggle_independently_and_can_both_be_checked() {
    let mut state = StorybookWindowState {
        selected_page: "checkbox",
        ..StorybookWindowState::default()
    };
    let first_row = dedicated_dod_form_binary_choice_live::checkbox_row_rect(0, HERO_X, HERO_Y);
    let second_row = dedicated_dod_form_binary_choice_live::checkbox_row_rect(1, HERO_X, HERO_Y);

    assert_eq!(
        Some(StorybookButtonOperation::CheckboxToggle(0)),
        button_operation_at(
            &state,
            first_row.x + CLICK_CENTER,
            first_row.y + CLICK_CENTER
        )
    );
    assert!(apply_click(
        &mut state,
        first_row.x + CLICK_CENTER,
        first_row.y + CLICK_CENTER
    ));
    let first_checked_canvas = render_checkbox(&state);
    assert!(state.screen_state.is_checkbox_checked_at(0));
    assert!(!state.screen_state.is_checkbox_checked_at(1));

    assert_eq!(
        Some(StorybookButtonOperation::CheckboxToggle(1)),
        button_operation_at(
            &state,
            second_row.x + CLICK_CENTER,
            second_row.y + CLICK_CENTER
        )
    );
    assert!(apply_click(
        &mut state,
        second_row.x + CLICK_CENTER,
        second_row.y + CLICK_CENTER
    ));
    let both_checked_canvas = render_checkbox(&state);
    let second_mark = dedicated_dod_form_binary_choice_live::checkbox_mark_rect(1, HERO_X, HERO_Y);
    assert!(state.screen_state.is_checkbox_checked_at(0));
    assert!(state.screen_state.is_checkbox_checked_at(1));
    assert!(state.screen_state.checkbox_state_snapshot_at(0).checked);
    assert!(state.screen_state.checkbox_state_snapshot_at(1).checked);
    assert_ne!(
        pixel_at(
            &first_checked_canvas,
            second_mark.x + CLICK_CENTER,
            second_mark.y + CLICK_CENTER
        ),
        pixel_at(
            &both_checked_canvas,
            second_mark.x + CLICK_CENTER,
            second_mark.y + CLICK_CENTER
        ),
        "second checkbox row must render its own checked mark"
    );
}

#[test]
fn checkbox_status_reflects_focused_secondary_row_state() {
    let mut state = StorybookWindowState {
        selected_page: "checkbox",
        ..StorybookWindowState::default()
    };
    let second_row = dedicated_dod_form_binary_choice_live::checkbox_row_rect(1, HERO_X, HERO_Y);
    let status = dedicated_dod_form_binary_choice_live::checkbox_state_row_rect(HERO_X, HERO_Y);

    assert!(apply_click(
        &mut state,
        second_row.x + CLICK_CENTER,
        second_row.y + CLICK_CENTER
    ));
    assert!(state.screen_state.is_checkbox_checked_at(1));
    assert!(!state.screen_state.is_checkbox_checked_at(0));

    let canvas = render_checkbox(&state);
    assert!(
        text_in_rect(&canvas, status).contains("checked=true"),
        "checkbox status row must report the focused secondary row state"
    );
}

#[test]
fn checkbox_checked_preset_does_not_keep_mark_after_state_turns_false() {
    let mut state = StorybookWindowState {
        selected_page: "checkbox",
        ..StorybookWindowState::default()
    };
    state.select_preset(CHECKBOX_CHECKED_PRESET_INDEX);
    let first_row = dedicated_dod_form_binary_choice_live::checkbox_row_rect(0, HERO_X, HERO_Y);
    let first_mark = dedicated_dod_form_binary_choice_live::checkbox_mark_rect(0, HERO_X, HERO_Y);
    let initial_canvas = render_checkbox(&state);

    assert!(state.screen_state.is_checkbox_checked_at(0));
    assert!(count_color_in_rect(&initial_canvas, first_mark, CHECKBOX_ACCENT) > 0);
    assert!(apply_click(
        &mut state,
        first_row.x + CLICK_CENTER,
        first_row.y + CLICK_CENTER
    ));
    assert_eq!("before=true after=false", state.screen_state.state_label);
    assert!(!state.screen_state.is_checkbox_checked_at(0));

    let canvas = render_checkbox(&state);
    assert_eq!(0, count_color_in_rect(&canvas, first_mark, CHECKBOX_ACCENT));
    assert_eq!(0, count_color_in_rect(&canvas, first_mark, CHECKBOX_GLYPH));
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
    assert_eq!("checked=false", state.screen_state.state_label);
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
fn checkbox_visual_and_catalog_use_same_typed_action_names() -> Result<(), String> {
    let checkbox = StoryCatalog
        .examples()
        .into_iter()
        .find(|it| it.page == "checkbox")
        .ok_or_else(|| "checkbox story missing".to_string())?;
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
    Ok(())
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

#[test]
fn checkbox_row_toggle_matches_core_public_checkbox_action_snapshot() {
    let mut storybook_state = StorybookWindowState {
        selected_page: "checkbox",
        ..StorybookWindowState::default()
    };
    let first_row = dedicated_dod_form_binary_choice_live::checkbox_row_rect(0, HERO_X, HERO_Y);
    let before = storybook_state
        .screen_state
        .checkbox_state_snapshot()
        .clone();
    let mut core_checkbox = Checkbox::new("Storybook Checkbox").set_state(before.clone());
    let _result =
        core_checkbox.apply_action(&UiAction::checkbox_checked(before.state_id.clone(), true));
    let expected = core_checkbox.state_snapshot();

    assert!(apply_click(
        &mut storybook_state,
        first_row.x + CLICK_CENTER,
        first_row.y + CLICK_CENTER
    ));
    let actual = storybook_state.screen_state.checkbox_state_snapshot();

    assert_eq!(expected.state_id, actual.state_id);
    assert_eq!(expected.checked, actual.checked);
    assert_eq!(
        expected.interaction.has_selection,
        actual.interaction.has_selection
    );
    assert_eq!(
        expected.interaction.selected_index,
        actual.interaction.selected_index
    );
}

fn render_checkbox(state: &StorybookWindowState) -> Canvas {
    render::render_storybook_canvas_with_screen_state(
        "dark",
        "checkbox",
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    if x >= canvas.width() || y >= canvas.height() {
        return None;
    }
    Some(canvas.pixels()[y * canvas.width() + x])
}

fn count_color_in_rect(canvas: &Canvas, rect: layout_metrics::LayoutRect, color: u32) -> usize {
    (rect.y..rect.bottom())
        .flat_map(|y| (rect.x..rect.right()).map(move |x| (x, y)))
        .filter(|(x, y)| pixel_at(canvas, *x, *y) == Some(color))
        .count()
}

fn text_in_rect(canvas: &Canvas, rect: layout_metrics::LayoutRect) -> String {
    canvas
        .text_runs()
        .iter()
        .filter(|run| rect.overlaps(run.rect()))
        .map(|run| run.text())
        .collect::<Vec<_>>()
        .join(" ")
}
