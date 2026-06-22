use super::screen_state_tabs::TabsScreenAction;
use super::visual_interaction_test_support::{component_body_pixel_diff, require_some};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{dedicated_tabs, preview_detail, render};

const DARK_THEME: &str = "dark";
const PAGE: &str = "tabs";
const PRIMARY_INSTANCE: &str = "tabs.primary";
const SECONDARY_INSTANCE: &str = "tabs.secondary";

#[test]
fn tabs_window_interaction_keeps_instance_state_isolated() -> Result<(), String> {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    state.select_instance(PRIMARY_INSTANCE);
    click_control(&mut state, TabsScreenAction::AddTab)?;
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert!(primary.tabs.tabs.iter().any(|tab| tab.id == "notes.md"));

    state.select_instance(SECONDARY_INSTANCE);
    assert!(
        !state
            .screen_state
            .tabs
            .tabs
            .iter()
            .any(|tab| tab.id == "notes.md")
    );
    click_control(&mut state, TabsScreenAction::TogglePinActive)?;
    let secondary = state.screen_state.clone();
    let secondary_canvas = render_state(&state);
    assert!(!secondary.tabs.tabs.iter().any(|tab| tab.id == "notes.md"));
    assert!(secondary.tabs.active_tab().is_some_and(|tab| tab.pinned));

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.tabs.tabs, state.screen_state.tabs.tabs);
    assert_eq!(
        primary.tabs.active_tab_id,
        state.screen_state.tabs.active_tab_id
    );
    assert!(
        state
            .screen_state
            .tabs
            .tabs
            .iter()
            .any(|tab| tab.id == "notes.md")
    );
    assert!(
        !state
            .screen_state
            .tabs
            .active_tab()
            .is_some_and(|tab| tab.pinned)
    );
    assert!(
        component_body_pixel_diff(PAGE, &primary_canvas, &secondary_canvas) > 80,
        "tabs instance-local state must produce distinct rendered bodies"
    );

    Ok(())
}

fn click_control(state: &mut StorybookWindowState, action: TabsScreenAction) -> Result<(), String> {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let rect = require_some(
        dedicated_tabs::control_rect_for_test(action),
        "tabs control rect",
    )?;

    assert!(apply_click(
        state,
        component.x + rect.x + 1,
        component.y + rect.y + 1
    ));
    Ok(())
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}
