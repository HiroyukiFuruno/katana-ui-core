use super::{layout_metrics, panel_screen_state, preview_detail, window_interaction};

const PANEL_PAGE: &str = "panel";
const NESTED_PRESET: usize = 3;
const CHILD_CLICK_INSET: usize = 12;
const PREVIEW_SLOT_Y: usize = 64;

#[test]
fn panel_inspector_active_panel_buttons_scope_scrollbar_setting() {
    let mut state = panel_window_state();
    let details = layout_metrics::panel_active_details_rect();
    let off = layout_metrics::panel_scrollbar_off_rect();

    assert!(window_interaction::apply_click(
        &mut state,
        details.x + 1,
        details.y + 1
    ));
    assert!(window_interaction::apply_click(
        &mut state,
        off.x + 1,
        off.y + 1
    ));

    assert_eq!(
        panel_screen_state::PanelChildKey::Details,
        state.screen_state.panel.active_panel
    );
    assert!(child_state(&state, panel_screen_state::PanelChildKey::Preview).scrollbar_visible);
    assert!(!child_state(&state, panel_screen_state::PanelChildKey::Details).scrollbar_visible);
}

#[test]
fn panel_wheel_over_child_panel_updates_only_that_panel_state() {
    let mut state = panel_window_state();

    assert!(window_interaction::apply_scroll_delta_at_for_test(
        &mut state,
        panel_child_x(484),
        panel_child_y(),
        -1.0
    ));

    assert_eq!(
        panel_screen_state::PanelChildKey::Details,
        state.screen_state.panel.active_panel
    );
    assert_eq!(
        36 + layout_metrics::SCROLL_STEP as u32,
        child_state(&state, panel_screen_state::PanelChildKey::Details).scroll_y
    );
    assert_eq!(
        72,
        child_state(&state, panel_screen_state::PanelChildKey::Preview).scroll_y
    );
}

fn panel_window_state() -> window_interaction::StorybookWindowState {
    window_interaction::StorybookWindowState {
        selected_page: PANEL_PAGE,
        preset_index: NESTED_PRESET,
        ..window_interaction::StorybookWindowState::default()
    }
}

fn child_state(
    state: &window_interaction::StorybookWindowState,
    panel: panel_screen_state::PanelChildKey,
) -> panel_screen_state::PanelChildState {
    state.screen_state.panel.child(panel)
}

fn panel_child_x(slot_x: usize) -> usize {
    preview_detail::HERO_PREVIEW_X_FOR_TEST + slot_x + CHILD_CLICK_INSET
}

fn panel_child_y() -> usize {
    preview_detail::HERO_PREVIEW_Y_FOR_TEST + PREVIEW_SLOT_Y + CHILD_CLICK_INSET
}
