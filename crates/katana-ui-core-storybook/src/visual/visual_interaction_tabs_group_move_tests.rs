use super::screen_state_tabs::{TabsScreenGroup, TabsScreenTab};
use super::visual_interaction_test_support::require_some;
use super::window_interaction::{StorybookWindowState, apply_click, apply_context_click_for_test};
use super::{dedicated_tabs, preview_detail};

const PAGE: &str = "tabs";
const MOVE_GROUP_LABEL: &str = "グループを移動";
const NOTES_GROUP_COLOR: u32 = 0x4ec9b0;

#[test]
fn tabs_group_header_context_menu_move_reorders_groups_through_core_action() -> Result<(), String> {
    let mut state = tabs_state_with_notes_group();

    assert_eq!(
        vec![
            "readme.md",
            "editor.rs",
            "preview.rs",
            "notes.md",
            "scratch.md",
            "terminal",
        ],
        tab_order(&state)
    );
    open_group_context_menu(&mut state, "docs")?;
    click_open_context_command(&mut state, MOVE_GROUP_LABEL)?;

    assert_eq!("group_context_move", state.screen_state.last_action);
    assert_eq!(
        "closeable_tab_group_reordered",
        state.screen_state.last_event
    );
    assert_eq!("target_index=1", state.screen_state.last_setting_value);
    assert_eq!(vec!["notes", "docs"], group_order(&state));
    assert_eq!(
        vec![
            "readme.md",
            "notes.md",
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
fn tabs_group_header_context_menu_move_wraps_last_group_to_first_through_core_action()
-> Result<(), String> {
    let mut state = tabs_state_with_notes_group();

    open_group_context_menu(&mut state, "notes")?;
    click_open_context_command(&mut state, MOVE_GROUP_LABEL)?;

    assert_eq!("group_context_move", state.screen_state.last_action);
    assert_eq!(
        "closeable_tab_group_reordered",
        state.screen_state.last_event
    );
    assert_eq!("target_index=0", state.screen_state.last_setting_value);
    assert_eq!(vec!["notes", "docs"], group_order(&state));
    assert_eq!(
        vec![
            "readme.md",
            "notes.md",
            "editor.rs",
            "preview.rs",
            "scratch.md",
            "terminal",
        ],
        tab_order(&state)
    );
    Ok(())
}

fn tabs_state_with_notes_group() -> StorybookWindowState {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    state.screen_state.tabs.groups.push(TabsScreenGroup {
        id: "notes".to_string(),
        title: "Notes".to_string(),
        color: NOTES_GROUP_COLOR,
        collapsed: false,
    });
    state
        .screen_state
        .tabs
        .tabs
        .push(TabsScreenTab::new("notes.md", "notes").group_id("notes"));
    state
}

fn open_group_context_menu(state: &mut StorybookWindowState, group_id: &str) -> Result<(), String> {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let group = require_some(
        dedicated_tabs::group_rect_for_test(&state.screen_state.tabs, group_id),
        "group rect",
    )?;
    assert!(apply_context_click_for_test(
        state,
        component.x + group.x + 1,
        component.y + group.y + 1
    ));
    Ok(())
}

fn click_open_context_command(
    state: &mut StorybookWindowState,
    command_label: &str,
) -> Result<(), String> {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let labels = dedicated_tabs::context_menu_labels_for_test(&state.screen_state.tabs);
    let command_index = labels
        .iter()
        .position(|label| *label == command_label)
        .ok_or_else(|| command_label.to_string())?;
    let menu = require_some(
        dedicated_tabs::context_menu_rect_for_test(&state.screen_state.tabs),
        "context menu rect",
    )?;
    let row_height = menu.height / labels.len();
    assert!(apply_click(
        state,
        component.x + menu.x + 1,
        component.y + menu.y + command_index * row_height + 1
    ));
    Ok(())
}

fn tab_order(state: &StorybookWindowState) -> Vec<&str> {
    dedicated_tabs::tab_ids_for_test(&state.screen_state.tabs)
}

fn group_order(state: &StorybookWindowState) -> Vec<&str> {
    state
        .screen_state
        .tabs
        .groups
        .iter()
        .map(|group| group.id.as_str())
        .collect()
}
