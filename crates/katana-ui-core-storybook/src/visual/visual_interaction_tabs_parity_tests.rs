use super::visual_interaction_test_support::require_some;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{dedicated_tabs, preview_detail};

const PAGE: &str = "tabs";

#[test]
fn tabs_pinned_icon_click_directly_unpins_tab() -> Result<(), String> {
    let mut state = tabs_state();
    let component = preview_detail::component_action_hit_rect(PAGE);
    let pin_icon = require_some(
        dedicated_tabs::pin_icon_rect_for_test(&state.screen_state.tabs, "readme.md"),
        "pinned tab icon rect",
    )?;

    assert!(apply_click(
        &mut state,
        component.x + pin_icon.x + pin_icon.width / 2,
        component.y + pin_icon.y + pin_icon.height / 2,
    ));
    let readme = tab_by_id(&state, "readme.md")?;
    assert!(!readme.pinned);
    assert_eq!("tab_pin_icon_unpin", state.screen_state.last_action);
    assert_eq!("closeable_tab_pin_changed", state.screen_state.last_event);
    assert_eq!("tabs.pin", state.screen_state.last_setting);
    assert_eq!("direct-icon", state.screen_state.last_setting_value);
    assert_eq!(
        "tabs.pinned=false closeable=true",
        state.screen_state.state_label
    );
    Ok(())
}

#[test]
fn tabs_pinned_tabs_render_before_group_block() -> Result<(), String> {
    let state = tabs_state();
    let group = require_some(
        dedicated_tabs::group_rect_for_test(&state.screen_state.tabs, "docs"),
        "docs group rect",
    )?;
    let pinned = require_some(
        dedicated_tabs::tab_rect_for_test(&state.screen_state.tabs, "readme.md"),
        "pinned tab rect",
    )?;

    assert!(pinned.x < group.x);
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
