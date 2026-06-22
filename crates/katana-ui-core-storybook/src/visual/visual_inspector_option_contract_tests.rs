use super::navigation_tree::TreeExpansionState;
use super::panel_scroll_state::PanelScrollOffsets;
use super::render_context::ScenarioContext;
use super::screen_state::StorybookScreenState;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{
    button_options, inspector_rows, layout_metrics, preview_detail, render,
    storybook_ui_option_contract,
};
use crate::StoryCatalog;
use crate::requirements::StoryRequirements;
use button_options::StorybookButtonOptionControl;

const CLIP_SUFFIX_WIDTH: usize = 3;
const COMPONENT_BODY_REPAINT_THRESHOLD: usize = 80;

#[test]
fn inspector_settings_rows_include_every_option_contract_for_each_story() {
    let examples = StoryCatalog.examples();
    let screen_state = StorybookScreenState::default();

    for example in &examples {
        let rows = inspector_rows::settings_rows(
            example.tree.root(),
            example,
            ScenarioContext {
                selected_page: example.page,
                selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
                preset_index: 0,
                preset_tab_scroll_x: 0,
                tree_expansion: TreeExpansionState::default(),
                scrollbar_visible: true,
                panel_scroll: PanelScrollOffsets::default(),
                screen_state: &screen_state,
                show_navigation_lines: true,
                show_navigation_text_connectors: false,
            },
        );
        let joined = rows.join("\n");

        for option in storybook_ui_option_contract::options_for_page(example.page) {
            assert!(
                setting_is_visible(&joined, option.setting),
                "{} Inspector settings missing option `{}`",
                example.page,
                option.setting
            );
        }
    }
}

fn setting_is_visible(rows: &str, setting: &str) -> bool {
    if rows.contains(setting) {
        return true;
    }
    let keep = inspector_rows::ROW_MAX_CHARS.saturating_sub(CLIP_SUFFIX_WIDTH);
    let clipped: String = setting.chars().take(keep).collect();
    !clipped.is_empty() && rows.contains(clipped.as_str())
}

#[test]
fn inspector_setting_rows_apply_each_clicked_option_contract() {
    for page in StoryRequirements::required_pages() {
        if inspector_rows::is_button_page(page) {
            continue;
        }
        for (index, option) in storybook_ui_option_contract::options_for_page(page)
            .iter()
            .enumerate()
        {
            let mut state = StorybookWindowState {
                selected_page: page,
                ..StorybookWindowState::default()
            };
            let row = layout_metrics::inspector_setting_row_hit_rect(index);

            assert!(
                apply_click(&mut state, row.x + 1, row.y + 1),
                "{page} Inspector option `{}` was not clickable",
                option.setting
            );
            assert_eq!(
                1, state.screen_state.settings_revision,
                "{page} Inspector option `{}` did not mutate settings state",
                option.setting
            );
            assert_eq!(
                option.setting, state.screen_state.last_setting,
                "{page} Inspector option mismatch"
            );
            assert_eq!(
                option.after, state.screen_state.last_setting_value,
                "{page} Inspector option value mismatch"
            );
        }
    }
}

#[test]
fn inspector_setting_rows_repaint_preview_for_each_clicked_option_contract() {
    for page in StoryRequirements::required_pages() {
        let mut state = StorybookWindowState {
            selected_page: page,
            ..StorybookWindowState::default()
        };
        let before = render_state(&state);
        for (index, option) in storybook_ui_option_contract::options_for_page(page)
            .iter()
            .enumerate()
        {
            let row = option_hit_rect(page, index, *option);

            assert!(
                apply_click(&mut state, row.x + 1, row.y + 1),
                "{page} Inspector option `{}` was not clickable",
                option.setting
            );
        }
        let after = render_state(&state);
        assert!(
            component_body_pixel_diff(page, &before, &after) > COMPONENT_BODY_REPAINT_THRESHOLD,
            "{page} clicked Inspector option contracts did not repaint preview body",
        );
    }
}

#[test]
fn button_option_controls_match_storybook_option_contract() {
    for page in button_pages() {
        let options = storybook_ui_option_contract::options_for_page(page);
        assert_eq!(
            StorybookButtonOptionControl::all().len(),
            options.len(),
            "{page} button option contract must cover every Inspector control"
        );
        for control in StorybookButtonOptionControl::all() {
            assert!(
                options
                    .iter()
                    .any(|option| option.setting == control.setting_name()),
                "{page} button option contract missing `{}`",
                control.setting_name()
            );
        }
    }
}

#[test]
fn button_inspector_controls_apply_each_button_option_contract() {
    for page in button_pages() {
        let options = storybook_ui_option_contract::options_for_page(page);
        for control in StorybookButtonOptionControl::all() {
            let option = options
                .iter()
                .find(|option| option.setting == control.setting_name())
                .copied();
            assert!(
                option.is_some(),
                "{page} button option contract missing `{}`",
                control.setting_name()
            );
            let Some(option) = option else {
                continue;
            };
            let mut state = StorybookWindowState {
                selected_page: page,
                ..StorybookWindowState::default()
            };
            let row = button_options::control_rect(control);

            assert!(
                apply_click(&mut state, row.x + 1, row.y + 1),
                "{page} button Inspector control `{}` was not clickable",
                control.setting_name()
            );
            assert_eq!(
                1,
                state.screen_state.settings_revision,
                "{page} button Inspector control `{}` did not mutate settings state",
                control.setting_name()
            );
            assert_eq!(
                option.setting, state.screen_state.last_setting,
                "{page} button Inspector control setting mismatch"
            );
            assert_eq!(
                option.after, state.screen_state.last_setting_value,
                "{page} button Inspector control value mismatch"
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

fn option_hit_rect(
    page: &str,
    index: usize,
    option: storybook_ui_option_contract::StorybookUiOptionContract,
) -> layout_metrics::LayoutRect {
    if !button_options::is_button_page(page) {
        return layout_metrics::inspector_setting_row_hit_rect(index);
    }
    let Some(control) = StorybookButtonOptionControl::all()
        .iter()
        .copied()
        .find(|control| control.setting_name() == option.setting)
    else {
        return layout_metrics::inspector_setting_row_hit_rect(index);
    };
    button_options::control_rect(control)
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn component_body_pixel_diff(page: &str, before: &super::Canvas, after: &super::Canvas) -> usize {
    let rect = preview_detail::component_action_hit_rect(page);
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
