use super::super::button_operation::{StorybookButtonOperation, button_operation_at};
use super::super::{StorybookWindowState, apply_click};
use crate::visual::button_options::{StorybookButtonOptionControl, control_rect};
use crate::visual::navigation_tree::{NavigationRow, row_from_click};
use crate::visual::{layout_metrics, preview_detail, render};

const UI_INTERACTION_DIFF_THRESHOLD: usize = 500;
const BUTTON_BODY_DIFF_THRESHOLD: usize = 40;
const TEXT_BUTTON_PAGE: &str = "text-button";
const BUTTON_VARIANT_PAGES: &[(&str, &str, &str)] = &[
    ("button", "button_press", "button_clicked"),
    ("text-button", "text_button_press", "text_button_clicked"),
    ("svg-button", "svg_button_press", "svg_button_clicked"),
    (
        "icon-text-button",
        "icon_text_button_press",
        "icon_text_button_clicked",
    ),
];

#[test]
fn click_button_operation_updates_action_event_state_for_button_preview() {
    let mut state = StorybookWindowState::default();
    let button = preview_detail::button_action_hit_rect("button");

    assert_eq!(
        Some(StorybookButtonOperation::PreviewButton),
        button_operation_at(&state, button.x + 1, button.y + 1)
    );
    assert!(apply_click(&mut state, button.x + 1, button.y + 1));
    assert_eq!(1, state.screen_state.action_count);
    assert_eq!("button_press", state.screen_state.last_action);
    assert_eq!("button_clicked", state.screen_state.last_event);
    assert_eq!("pressed=true", state.screen_state.state_label);
}

#[test]
fn each_button_variant_hit_rect_emits_its_own_action_and_event() {
    for &(page, action, event) in BUTTON_VARIANT_PAGES {
        let mut state = StorybookWindowState {
            selected_page: page,
            ..StorybookWindowState::default()
        };
        let target = preview_detail::button_action_hit_rect(page);

        assert!(target.width > 0, "{page} hit rect missing");
        assert_eq!(
            Some(StorybookButtonOperation::PreviewButton),
            button_operation_at(
                &state,
                target.x + target.width / 2,
                target.y + target.height / 2
            ),
            "{page} hit rect must map to the visible button body"
        );
        assert!(apply_click(
            &mut state,
            target.x + target.width / 2,
            target.y + target.height / 2
        ));
        assert_eq!(action, state.screen_state.last_action, "{page} action");
        assert_eq!(event, state.screen_state.last_event, "{page} event");
        assert_eq!(1, state.screen_state.action_count, "{page} action count");
    }
}

#[test]
fn repeated_button_click_returns_to_released_state_then_presses_again() {
    let mut state = StorybookWindowState::default();
    let button = preview_detail::button_action_hit_rect("button");

    assert!(apply_click(&mut state, button.x + 1, button.y + 1));
    assert_eq!(1, state.screen_state.action_count);
    assert_eq!("pressed=true", state.screen_state.state_label);

    assert!(state.screen_state.release_button_press());
    assert_eq!("pressed=false", state.screen_state.state_label);

    assert!(apply_click(&mut state, button.x + 1, button.y + 1));
    assert_eq!(2, state.screen_state.action_count);
    assert_eq!("pressed=true", state.screen_state.state_label);
}

#[test]
fn preset_tabs_do_not_share_button_press_state() {
    let mut state = StorybookWindowState::default();
    let button = preview_detail::button_action_hit_rect("button");
    let classic = layout_metrics::preset_tab_rect(layout_metrics::PRESET_INTERACTIVE_INDEX);

    assert!(apply_click(&mut state, button.x + 1, button.y + 1));
    assert_eq!("pressed=true", state.screen_state.state_label);
    assert!(apply_click(&mut state, classic.x + 1, classic.y + 1));

    assert_eq!(layout_metrics::PRESET_INTERACTIVE_INDEX, state.preset_index);
    assert_eq!("idle", state.screen_state.state_label);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!(0, state.screen_state.action_count);

    let modern = layout_metrics::preset_tab_rect(0);
    assert!(apply_click(&mut state, modern.x + 1, modern.y + 1));
    assert_eq!(0, state.preset_index);
    assert_eq!("pressed=true", state.screen_state.state_label);
    assert_eq!("button_press", state.screen_state.last_action);
}

#[test]
fn button_and_text_button_do_not_share_button_press_state() {
    let mut state = StorybookWindowState::default();
    let button = preview_detail::button_action_hit_rect("button");

    assert!(apply_click(&mut state, button.x + 1, button.y + 1));
    assert_eq!("pressed=true", state.screen_state.state_label);
    click_page(&mut state, TEXT_BUTTON_PAGE);

    assert_eq!(TEXT_BUTTON_PAGE, state.selected_page);
    assert_eq!("idle", state.screen_state.state_label);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!(0, state.screen_state.action_count);

    let text_button = preview_detail::button_action_hit_rect(TEXT_BUTTON_PAGE);
    assert!(apply_click(
        &mut state,
        text_button.x + 1,
        text_button.y + 1
    ));
    assert_eq!("text_button_press", state.screen_state.last_action);

    click_page(&mut state, "button");
    assert_eq!("button", state.selected_page);
    assert_eq!("button_press", state.screen_state.last_action);

    click_page(&mut state, TEXT_BUTTON_PAGE);
    assert_eq!("text_button_press", state.screen_state.last_action);
}

#[test]
fn click_button_option_operation_updates_action_event_state_option_and_rendering() {
    let mut state = StorybookWindowState::default();
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );
    let setting = control_rect(StorybookButtonOptionControl::Label);

    assert_eq!(
        Some(StorybookButtonOperation::ButtonOption(
            StorybookButtonOptionControl::Label
        )),
        button_operation_at(&state, setting.x + 1, setting.y + 1)
    );
    assert!(apply_click(&mut state, setting.x + 1, setting.y + 1));
    assert_eq!("button_option_apply", state.screen_state.last_action);
    assert_eq!("button_option_changed", state.screen_state.last_event);
    assert_eq!("label=ja", state.screen_state.state_label);
    assert_eq!("label", state.screen_state.last_setting);
    assert_eq!("保存する", state.screen_state.last_setting_value);
    assert!(state.screen_state.has_settings_override());

    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );
    assert!(pixel_diff(&before, &after) > UI_INTERACTION_DIFF_THRESHOLD);
}

#[test]
fn click_button_option_control_updates_action_event_state_option_and_rendering() {
    for control in StorybookButtonOptionControl::all() {
        assert_button_option_control_updates_action_event_state_option_and_rendering(control);
    }
}

fn assert_button_option_control_updates_action_event_state_option_and_rendering(
    control: StorybookButtonOptionControl,
) {
    let mut state = StorybookWindowState::default();
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );
    let rect = control_rect(control);

    assert_eq!(
        Some(StorybookButtonOperation::ButtonOption(control)),
        button_operation_at(&state, rect.x + 1, rect.y + 1)
    );
    assert!(apply_click(&mut state, rect.x + 1, rect.y + 1));
    assert_eq!(1, state.screen_state.settings_revision);
    assert_eq!("button_option_apply", state.screen_state.last_action);
    assert_eq!("button_option_changed", state.screen_state.last_event);
    assert_eq!(control.setting_name(), state.screen_state.last_setting);
    assert_eq!(
        control.setting_value(state.screen_state.button_options),
        state.screen_state.last_setting_value
    );
    assert_eq!(
        control.state_label(state.screen_state.button_options),
        state.screen_state.state_label
    );

    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );
    assert!(pixel_diff(&before, &after) > UI_INTERACTION_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(&before, &after) > BUTTON_BODY_DIFF_THRESHOLD);
}

#[test]
fn click_button_option_disabled_state_blocks_action_event_for_button_preview() {
    let mut state = StorybookWindowState::default();
    let control = control_rect(StorybookButtonOptionControl::Disabled);
    let button = preview_detail::button_action_hit_rect("button");

    assert!(apply_click(&mut state, control.x + 1, control.y + 1));
    assert!(apply_click(&mut state, button.x + 1, button.y + 1));
    assert_eq!(0, state.screen_state.action_count);
    assert_eq!("button_press_blocked", state.screen_state.last_action);
    assert_eq!("button_disabled_ignored", state.screen_state.last_event);
    assert_eq!("disabled=true", state.screen_state.state_label);
}

fn pixel_diff(before: &crate::visual::Canvas, after: &crate::visual::Canvas) -> usize {
    before
        .pixels()
        .iter()
        .zip(after.pixels().iter())
        .filter(|(left, right)| left != right)
        .count()
}

fn component_body_pixel_diff(
    before: &crate::visual::Canvas,
    after: &crate::visual::Canvas,
) -> usize {
    let rect = preview_detail::button_action_hit_rect("button");
    let mut diff = 0;
    for current_y in rect.y..rect.bottom() {
        for current_x in rect.x..rect.right() {
            let index = current_y * before.width() + current_x;
            if before.pixels()[index] != after.pixels()[index] {
                diff += 1;
            }
        }
    }
    diff
}

fn click_page(state: &mut StorybookWindowState, page: &'static str) {
    let target = click_target_for_page(page);

    assert!(target.is_some());
    if let Some((x, y)) = target {
        assert!(apply_click(state, x, y));
    }
}

fn click_target_for_page(page: &str) -> Option<(usize, usize)> {
    for y in 0..layout_metrics::CONTENT_HEIGHT {
        let x = layout_metrics::NAV_ROW_X + 1;
        if matches!(
            row_from_click(x, y, Default::default()),
            Some(
                NavigationRow::Page { page: found, .. }
                | NavigationRow::PageWithoutSection { page: found, .. }
            ) if found == page
        ) {
            return Some((x, y));
        }
    }
    None
}
