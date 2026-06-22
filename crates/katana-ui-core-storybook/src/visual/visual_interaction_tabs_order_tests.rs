use super::screen_state_tabs::{TabsScreenAction, TabsScreenTab};
use super::visual_interaction_test_support::require_some;
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_context_click_for_test, apply_tabs_drag_at_for_test,
    release_tabs_drag_for_test, start_tabs_drag_at_for_test,
};
use super::{dedicated_tabs, preview_detail};

const PAGE: &str = "tabs";
const CLOSE_RIGHT_INDEX: usize = 3;

#[test]
fn tabs_context_menu_reaches_unknown_group_tabs_after_pinned_region() -> Result<(), String> {
    let mut state = tabs_state();
    state
        .screen_state
        .tabs
        .tabs
        .push(TabsScreenTab::new("orphan.md", "orphan").group_id("missing"));

    assert_eq!(
        vec![
            "readme.md",
            "editor.rs",
            "preview.rs",
            "orphan.md",
            "scratch.md",
            "terminal",
        ],
        tab_order(&state)
    );
    click_context_command(&mut state, "orphan.md", CLOSE_RIGHT_INDEX)?;

    assert_eq!("tab_context_close_right", state.screen_state.last_action);
    assert_eq!(
        vec!["readme.md", "editor.rs", "preview.rs", "orphan.md"],
        tab_order(&state)
    );
    Ok(())
}

#[test]
fn tabs_storybook_layout_order_matches_core_visual_order_for_declared_unknown_pinned_ungrouped() {
    let mut state = pinned_pair_state();
    state
        .screen_state
        .tabs
        .tabs
        .push(TabsScreenTab::new("orphan.md", "orphan").group_id("missing"));
    state
        .screen_state
        .tabs
        .tabs
        .retain(|tab| tab.id != "terminal");

    let render_order: Vec<String> = dedicated_tabs::tab_ids_for_test(&state.screen_state.tabs)
        .into_iter()
        .map(str::to_string)
        .collect();

    assert_eq!(state.screen_state.tabs.core_visual_tab_ids(), render_order);
}

#[test]
fn tabs_move_control_uses_visual_index_for_pinned_region() -> Result<(), String> {
    let mut state = pinned_pair_state();

    assert_eq!(
        vec![
            "readme.md",
            "guide.md",
            "editor.rs",
            "preview.rs",
            "scratch.md",
            "terminal",
        ],
        tab_order(&state)
    );
    click_control(&mut state, TabsScreenAction::MoveActiveRight)?;

    assert_eq!("move_tab", state.screen_state.last_action);
    assert_eq!("closeable_tab_reordered", state.screen_state.last_event);
    assert_eq!(
        vec![
            "guide.md",
            "readme.md",
            "editor.rs",
            "preview.rs",
            "scratch.md",
            "terminal",
        ],
        tab_order(&state)
    );
    Ok(())
}

#[test]
fn tabs_drag_moves_pinned_tab_by_visual_order_and_ends_drag() -> Result<(), String> {
    let mut state = pinned_pair_state();
    let component = preview_detail::component_action_hit_rect(PAGE);
    let readme = require_some(
        dedicated_tabs::tab_rect_for_test(&state.screen_state.tabs, "readme.md"),
        "readme tab rect",
    )?;
    let guide = require_some(
        dedicated_tabs::tab_rect_for_test(&state.screen_state.tabs, "guide.md"),
        "guide tab rect",
    )?;

    assert!(start_tabs_drag_at_for_test(
        &mut state,
        component.x + readme.x + readme.width / 2,
        component.y + readme.y + readme.height / 2,
    ));
    assert_eq!("tab_drag_start", state.screen_state.last_action);
    assert_eq!("closeable_tab_drag_started", state.screen_state.last_event);
    assert!(apply_tabs_drag_at_for_test(
        &mut state,
        component.x + guide.x + guide.width - 1,
        component.y + guide.y + guide.height / 2,
    ));
    assert_eq!("tab_drag_move", state.screen_state.last_action);
    assert_eq!("closeable_tab_reordered", state.screen_state.last_event);
    assert_eq!(
        vec![
            "guide.md",
            "readme.md",
            "editor.rs",
            "preview.rs",
            "scratch.md",
            "terminal",
        ],
        tab_order(&state)
    );

    assert!(release_tabs_drag_for_test(&mut state));
    assert_eq!("tab_drag_end", state.screen_state.last_action);
    assert_eq!("closeable_tab_drag_ended", state.screen_state.last_event);
    assert_eq!("tabs.dragging=false", state.screen_state.state_label);
    Ok(())
}

fn click_context_command(
    state: &mut StorybookWindowState,
    tab_id: &str,
    command_index: usize,
) -> Result<(), String> {
    open_context_menu(state, tab_id)?;
    let component = preview_detail::component_action_hit_rect(PAGE);
    let labels = dedicated_tabs::context_menu_labels_for_test(&state.screen_state.tabs);
    let menu = require_some(
        dedicated_tabs::context_menu_rect_for_test(&state.screen_state.tabs),
        "context menu rect",
    )?;
    let row_height = menu.height / labels.len();
    assert!(apply_click(
        state,
        component.x + menu.x + 1,
        component.y + menu.y + command_index * row_height + 1,
    ));
    Ok(())
}

fn open_context_menu(state: &mut StorybookWindowState, tab_id: &str) -> Result<(), String> {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let tab = require_some(
        dedicated_tabs::tab_rect_for_test(&state.screen_state.tabs, tab_id),
        "tab rect",
    )?;
    assert!(apply_context_click_for_test(
        state,
        component.x + tab.x + 1,
        component.y + tab.y + 1,
    ));
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
        component.y + rect.y + 1,
    ));
    Ok(())
}

fn tabs_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn pinned_pair_state() -> StorybookWindowState {
    let mut state = tabs_state();
    state
        .screen_state
        .tabs
        .tabs
        .push(TabsScreenTab::new("guide.md", "guide").pinned(true));
    state.screen_state.tabs.active_tab_id = "readme.md".to_string();
    state
}

fn tab_order(state: &StorybookWindowState) -> Vec<&str> {
    dedicated_tabs::tab_ids_for_test(&state.screen_state.tabs)
}
