use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";
const DYNAMIC_ARRAY_STATES: [OptionExpectation; 4] = [
    OptionExpectation::new(
        "array.rows",
        "3",
        "array_rows_option",
        "array_changed",
        "array.rows=3",
    ),
    OptionExpectation::new(
        "array.add_remove",
        "add+remove",
        "array_add_remove_option",
        "array_changed",
        "array.rows=4",
    ),
    OptionExpectation::new(
        "array.reorder",
        "true",
        "array_reorder_option",
        "array_changed",
        "array.order=2,1,3",
    ),
    OptionExpectation::new(
        "array.theme_row",
        "accent",
        "array_theme_row_option",
        "array_callback",
        "array.theme_row=accent",
    ),
];
const DRAG_AND_DROP_STATES: [OptionExpectation; 4] = [
    OptionExpectation::new(
        "drag.accept_policy",
        "Accept(after)",
        "drag_accept_policy_option",
        "drag_accept_changed",
        "drag.accept_policy=move",
    ),
    OptionExpectation::new(
        "drag.autoscroll",
        "edge=24",
        "drag_autoscroll_option",
        "drag_autoscroll_changed",
        "drag.autoscroll=edge",
    ),
    OptionExpectation::new(
        "drag.keyboard_draggable",
        "true",
        "drag_keyboard_option",
        "drag_keyboard_changed",
        "drag.keyboard_draggable=true",
    ),
    OptionExpectation::new(
        "drag.drop_indicator",
        "after",
        "drag_indicator_option",
        "drag_indicator_changed",
        "drag.drop_indicator=after",
    ),
];

#[test]
fn live_component_inspector_options_mutate_array_and_drag_semantic_state() -> Result<(), String> {
    assert_options("dynamic-array-editor", dynamic_array_states())?;
    assert_options("drag-and-drop", drag_and_drop_states())
}

fn dynamic_array_states() -> &'static [OptionExpectation] {
    &DYNAMIC_ARRAY_STATES
}

fn drag_and_drop_states() -> &'static [OptionExpectation] {
    &DRAG_AND_DROP_STATES
}

fn assert_options(
    page: &'static str,
    expected_states: &'static [OptionExpectation],
) -> Result<(), String> {
    for expected in expected_states {
        let mut state = page_state(page);
        let before = render_state(&state, page);
        click_option(&mut state, page, expected.setting)?;
        let after = render_state(&state, page);

        assert_eq!(expected.setting, state.screen_state.last_setting);
        assert_eq!(expected.value, state.screen_state.last_setting_value);
        assert_eq!(expected.action, state.screen_state.last_action);
        assert_eq!(expected.event, state.screen_state.last_event);
        assert_eq!(expected.state, state.screen_state.state_label);
        assert_live_component_runtime(page, expected.setting, &state);
        assert!(component_body_pixel_diff(page, &before, &after) > 0);
    }
    Ok(())
}

fn assert_live_component_runtime(page: &str, setting: &str, state: &StorybookWindowState) {
    if page == "dynamic-array-editor" {
        assert_dynamic_array_runtime(setting, state);
        return;
    }
    assert_drag_and_drop_runtime(setting, state);
}

fn assert_dynamic_array_runtime(setting: &str, state: &StorybookWindowState) {
    let dynamic_array = &state.screen_state.dynamic_array_editor;
    match setting {
        "array.rows" => {
            assert_eq!(3, dynamic_array.item_count());
            assert_eq!("callback=rows", dynamic_array.callback_event());
        }
        "array.add_remove" => {
            assert_eq!(4, dynamic_array.item_count());
            assert_eq!("callback=add_remove", dynamic_array.callback_event());
        }
        "array.reorder" => {
            assert_eq!("order=2,1,3", dynamic_array.order_label());
            assert_eq!("callback=reorder", dynamic_array.callback_event());
        }
        "array.theme_row" => {
            assert_eq!("callback=theme", dynamic_array.callback_event());
        }
        _ => {}
    }
}

fn assert_drag_and_drop_runtime(_setting: &str, state: &StorybookWindowState) {
    let drag_and_drop = &state.screen_state.drag_and_drop;
    assert!(drag_and_drop.is_dragging());
    assert!(!drag_and_drop.committed());
}

#[derive(Debug, Clone, Copy)]
struct OptionExpectation {
    setting: &'static str,
    value: &'static str,
    action: &'static str,
    event: &'static str,
    state: &'static str,
}

impl OptionExpectation {
    const fn new(
        setting: &'static str,
        value: &'static str,
        action: &'static str,
        event: &'static str,
        state: &'static str,
    ) -> Self {
        Self {
            setting,
            value,
            action,
            event,
            state,
        }
    }
}

fn click_option(state: &mut StorybookWindowState, page: &str, setting: &str) -> Result<(), String> {
    let index = option_index(page, setting)?;
    let row = layout_metrics::inspector_setting_row_hit_rect(index);

    assert!(apply_click(state, row.x + 1, row.y + 1));
    Ok(())
}

fn option_index(page: &str, setting: &str) -> Result<usize, String> {
    storybook_ui_option_contract::options_for_page(page)
        .iter()
        .position(|option| option.setting == setting)
        .ok_or_else(|| format!("missing {page} option `{setting}`"))
}

fn render_state(state: &StorybookWindowState, page: &str) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn page_state(page: &'static str) -> StorybookWindowState {
    StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    }
}
