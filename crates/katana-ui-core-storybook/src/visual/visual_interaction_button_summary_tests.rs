use super::preview;
use super::render;
use super::screen_state::StorybookScreenState;
use super::window_interaction::StorybookWindowState;
use crate::DEFAULT_STORYBOOK_PAGE;

const DARK_THEME: &str = "dark";
const BUTTON_PAGE: &str = "button";
const DEFAULT_PRESET: usize = 0;
const SUMMARY_SETTING_INDEX: usize = 2;
const SUMMARY_TOOLTIP_DIFF_THRESHOLD: usize = 100;
const SUMMARY_TOOLTIP_SCAN_HEIGHT: usize = 40;
const SUMMARY_TOOLTIP_SCAN_WIDTH: usize = 360;

#[test]
fn summary_ellipsis_exposes_full_value_with_tooltip_state() {
    let mut screen_state = StorybookScreenState {
        last_setting: "label",
        last_setting_value: "保存する長いラベルを確認する",
        ..Default::default()
    };
    let scenario = button_summary_scenario_with_state(screen_state.clone());
    let full = preview::summary_full_samples_for_test(scenario);
    let visible = preview::summary_visible_samples_for_test(scenario);

    assert_eq!(
        "setting label=保存する長いラベルを確認する",
        full[SUMMARY_SETTING_INDEX]
    );
    assert_ne!(full[SUMMARY_SETTING_INDEX], visible[SUMMARY_SETTING_INDEX]);

    let hidden = render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        BUTTON_PAGE,
        DEFAULT_PRESET,
        screen_state.clone(),
    );
    screen_state.hovered_summary_index = Some(SUMMARY_SETTING_INDEX);
    let shown = render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        BUTTON_PAGE,
        DEFAULT_PRESET,
        screen_state,
    );

    assert!(summary_tooltip_pixel_diff(&hidden, &shown) > SUMMARY_TOOLTIP_DIFF_THRESHOLD);
}

fn button_summary_scenario_with_state(
    screen_state: StorybookScreenState,
) -> super::render_context::ScenarioContext<'static> {
    let screen_state = Box::leak(Box::new(screen_state));
    super::render_context::ScenarioContext {
        selected_page: BUTTON_PAGE,
        preset_index: DEFAULT_PRESET,
        preset_tab_scroll_x: 0,
        tree_expansion: Default::default(),
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
        screen_state,
    }
}

fn summary_tooltip_pixel_diff(before: &super::Canvas, after: &super::Canvas) -> usize {
    let rect = preview::summary_control_rect_for_test(SUMMARY_SETTING_INDEX);
    let start_y = rect.bottom();
    let end_y = start_y + SUMMARY_TOOLTIP_SCAN_HEIGHT;
    let end_x = (rect.x + SUMMARY_TOOLTIP_SCAN_WIDTH).min(before.width());
    let mut diff = 0;
    for current_y in start_y..end_y {
        for current_x in rect.x..end_x {
            let index = current_y * before.width() + current_x;
            if before.pixels()[index] != after.pixels()[index] {
                diff += 1;
            }
        }
    }
    diff
}

#[test]
fn default_window_state_uses_representative_input_page() {
    let state = StorybookWindowState::default();

    assert_eq!(DEFAULT_STORYBOOK_PAGE, state.selected_page);
}
