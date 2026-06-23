use super::button_options::{self, StorybookButtonOptionControl};
use super::storybook_ui_option_contract;
use super::window_interaction::{StorybookWindowState, apply_click};
use crate::requirements::StoryRequirements;

#[test]
fn button_inspector_rows_select_matching_preset_tabs() {
    for page in button_pages() {
        for control in StorybookButtonOptionControl::all() {
            let mut state = StorybookWindowState {
                selected_page: page,
                ..StorybookWindowState::default()
            };
            let row = button_options::control_rect(control);
            let option = option_for_control(page, control);
            assert!(
                option.is_some(),
                "{page} button option contract missing `{}`",
                control.setting_name()
            );
            let Some(option) = option else {
                continue;
            };

            assert!(
                apply_click(&mut state, row.x + 1, row.y + 1),
                "{page} button Inspector control `{}` was not clickable",
                control.setting_name()
            );
            assert_eq!(
                button_options::preset_index_for_control(control),
                state.preset_index,
                "{page} button Inspector control `{}` did not select matching preset",
                control.setting_name()
            );
            assert_eq!(
                option.after,
                state.screen_state.last_setting_value,
                "{page} button Inspector control `{}` did not apply contract value",
                control.setting_name()
            );
            assert_eq!(
                "button_option_apply",
                state.screen_state.last_action,
                "{page} button Inspector control `{}` did not apply action",
                control.setting_name()
            );
            assert_eq!(
                "button_option_changed",
                state.screen_state.last_event,
                "{page} button Inspector control `{}` did not emit event",
                control.setting_name()
            );
            assert_eq!(
                control.state_label(state.screen_state.button_options),
                state.screen_state.state_label,
                "{page} button Inspector control `{}` did not update semantic state",
                control.setting_name()
            );
        }
    }
}

#[test]
fn button_inspector_rows_apply_action_event_and_state_for_every_button_page() {
    for page in button_pages() {
        for control in StorybookButtonOptionControl::all() {
            let mut state = StorybookWindowState {
                selected_page: page,
                ..StorybookWindowState::default()
            };
            let row = button_options::control_rect(control);

            assert!(apply_click(&mut state, row.x + 1, row.y + 1));
            assert_eq!("button_option_apply", state.screen_state.last_action);
            assert_eq!("button_option_changed", state.screen_state.last_event);
            assert_eq!(
                control.state_label(state.screen_state.button_options),
                state.screen_state.state_label,
                "{page} {} state label",
                control.setting_name()
            );
        }
    }
}

fn button_pages() -> impl Iterator<Item = &'static str> {
    StoryRequirements::required_pages()
        .iter()
        .copied()
        .filter(|page| button_options::is_button_page(page))
}

fn option_for_control(
    page: &str,
    control: StorybookButtonOptionControl,
) -> Option<storybook_ui_option_contract::StorybookUiOptionContract> {
    storybook_ui_option_contract::options_for_page(page)
        .iter()
        .copied()
        .find(|option| option.setting == control.setting_name())
}
