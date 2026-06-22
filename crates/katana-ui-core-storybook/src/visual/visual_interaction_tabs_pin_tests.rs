use super::visual_interaction_test_support::{component_body_pixel_diff, require_some};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{dedicated_tabs, preview_detail, render};

const PAGE: &str = "tabs";
const BODY_DIFF_THRESHOLD: usize = 80;

#[test]
fn tabs_pin_icon_left_click_unpins_through_core_action() -> Result<(), String> {
    let mut state = tabs_state();
    let component = preview_detail::component_action_hit_rect(PAGE);
    let pin_icon = require_some(
        dedicated_tabs::pin_icon_rect_for_test(&state.screen_state.tabs, "readme.md"),
        "pin icon rect",
    )?;
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );

    assert!(tab_by_id(&state, "readme.md")?.pinned);
    assert!(apply_click(
        &mut state,
        component.x + pin_icon.x + 1,
        component.y + pin_icon.y + 1
    ));
    assert_eq!("tab_pin_icon_unpin", state.screen_state.last_action);
    assert_eq!("closeable_tab_pin_changed", state.screen_state.last_event);
    assert_eq!("tabs.pin", state.screen_state.last_setting);
    assert_eq!(
        "tabs.pinned=false closeable=true",
        state.screen_state.state_label
    );
    assert!(!tab_by_id(&state, "readme.md")?.pinned);

    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
    Ok(())
}

fn tabs_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn tab_by_id<'a>(
    state: &'a StorybookWindowState,
    tab_id: &str,
) -> Result<&'a super::screen_state_tabs::TabsScreenTab, String> {
    require_some(
        state
            .screen_state
            .tabs
            .tabs
            .iter()
            .find(|tab| tab.id == tab_id),
        "tab exists",
    )
}
