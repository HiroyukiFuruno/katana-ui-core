use super::window_interaction::{StorybookWindowState, apply_click};
use super::{dedicated_closeable_tab_strip, preview_detail};

const PAGE: &str = "closeable-tab-strip";
const CLOSE_INDEX: usize = 0;
const CLOSE_OTHERS_INDEX: usize = 1;
const CLOSE_ALL_INDEX: usize = 2;
const CLOSE_RIGHT_INDEX: usize = 3;
const CLOSE_LEFT_INDEX: usize = 4;
const PIN_INDEX: usize = 5;
const MOVE_TO_GROUP_INDEX: usize = 6;
const NEW_GROUP_INDEX: usize = 7;
const DOCS_GROUP_INDEX: usize = 8;
const RESTORE_CLOSED_INDEX: usize = 9;

#[test]
fn closeable_tab_strip_tab_context_menu_applies_workspace_tab_commands() -> Result<(), String> {
    let mut close_state = closeable_state();
    click_tab_context_command(&mut close_state, "terminal", CLOSE_INDEX)?;
    assert_eq!("tab_context_close", close_state.screen_state.last_action);
    assert!(!has_tab(&close_state, "terminal"));

    let mut others_state = closeable_state();
    click_tab_context_command(&mut others_state, "editor.rs", CLOSE_OTHERS_INDEX)?;
    assert_eq!(
        "tab_context_close_others",
        others_state.screen_state.last_action
    );
    assert_eq!(vec!["readme.md", "editor.rs"], tab_order(&others_state));

    let mut right_state = closeable_state();
    click_tab_context_command(&mut right_state, "editor.rs", CLOSE_RIGHT_INDEX)?;
    assert_eq!(
        "tab_context_close_right",
        right_state.screen_state.last_action
    );
    assert_eq!(vec!["readme.md", "editor.rs"], tab_order(&right_state));

    let mut left_state = closeable_state();
    click_tab_context_command(&mut left_state, "terminal", CLOSE_LEFT_INDEX)?;
    assert_eq!(
        "tab_context_close_left",
        left_state.screen_state.last_action
    );
    assert_eq!(vec!["readme.md", "terminal"], tab_order(&left_state));

    let mut all_state = closeable_state();
    click_tab_context_command(&mut all_state, "scratch.md", CLOSE_ALL_INDEX)?;
    assert_eq!("tab_context_close_all", all_state.screen_state.last_action);
    assert_eq!(vec!["readme.md"], tab_order(&all_state));

    let mut pin_state = closeable_state();
    click_tab_context_command(&mut pin_state, "scratch.md", PIN_INDEX)?;
    assert_eq!("tab_context_pin", pin_state.screen_state.last_action);
    assert!(active_tab(&pin_state)?.pinned);

    let mut new_group_state = closeable_state();
    click_tab_context_command(&mut new_group_state, "scratch.md", NEW_GROUP_INDEX)?;
    assert_eq!(
        "tab_context_new_group",
        new_group_state.screen_state.last_action
    );
    assert_eq!(
        Some("context-group"),
        active_tab(&new_group_state)?.group_id.as_deref()
    );

    let mut existing_group_state = closeable_state();
    click_tab_context_command(&mut existing_group_state, "scratch.md", DOCS_GROUP_INDEX)?;
    assert_eq!(
        "tab_context_move_group",
        existing_group_state.screen_state.last_action
    );
    assert_eq!(
        Some("docs"),
        active_tab(&existing_group_state)?.group_id.as_deref()
    );
    Ok(())
}

#[test]
fn closeable_tab_strip_context_menu_moves_to_selected_existing_group() -> Result<(), String> {
    let mut state = closeable_state();
    state
        .screen_state
        .tabs
        .groups
        .push(super::screen_state_tabs::TabsScreenGroup {
            id: "review".to_string(),
            title: "Review".to_string(),
            color: 0x5aa65a,
            collapsed: false,
        });
    open_tab_context_menu(&mut state, "scratch.md")?;
    let labels =
        dedicated_closeable_tab_strip::context_menu_labels_for_test(&state.screen_state.tabs);
    assert_eq!("グループに追加", labels[MOVE_TO_GROUP_INDEX]);
    let review_index = labels
        .iter()
        .position(|label| *label == "Review")
        .ok_or_else(|| "Review group menu item is missing".to_string())?;
    click_open_context_command(&mut state, review_index)?;

    assert_eq!("tab_context_move_group", state.screen_state.last_action);
    assert_eq!("closeable_tab_group_changed", state.screen_state.last_event);
    assert_eq!(Some("review"), active_tab(&state)?.group_id.as_deref());
    Ok(())
}

#[test]
fn closeable_tab_strip_context_menu_hides_group_commands_for_ungroupable_tab() -> Result<(), String>
{
    let mut state = closeable_state();
    let scratch = state
        .screen_state
        .tabs
        .tabs
        .iter_mut()
        .find(|tab| tab.id == "scratch.md")
        .ok_or_else(|| "scratch tab is missing".to_string())?;
    scratch.groupable = false;
    open_tab_context_menu(&mut state, "scratch.md")?;

    let labels =
        dedicated_closeable_tab_strip::context_menu_labels_for_test(&state.screen_state.tabs);
    assert!(!labels.contains(&"新しいグループを作成"));
    assert!(!labels.contains(&"グループに追加"));
    Ok(())
}

#[test]
fn closeable_tab_strip_context_menu_keeps_pinned_tabs_fixed_until_unpinned() -> Result<(), String> {
    let mut close_blocked = closeable_state();
    open_tab_context_menu(&mut close_blocked, "readme.md")?;
    let pinned_labels = dedicated_closeable_tab_strip::context_menu_labels_for_test(
        &close_blocked.screen_state.tabs,
    );
    assert!(!pinned_labels.contains(&"新しいグループを作成"));
    assert!(!pinned_labels.contains(&"グループに追加"));
    click_tab_context_command(&mut close_blocked, "readme.md", CLOSE_INDEX)?;
    assert!(has_tab(&close_blocked, "readme.md"));
    assert_eq!(
        "closeable_tab_context_close_blocked",
        close_blocked.screen_state.last_event
    );

    let mut unpinned = closeable_state();
    click_tab_context_command(&mut unpinned, "readme.md", PIN_INDEX)?;
    assert!(!active_tab(&unpinned)?.pinned);
    click_tab_context_command(&mut unpinned, "readme.md", CLOSE_INDEX)?;
    assert!(!has_tab(&unpinned, "readme.md"));
    Ok(())
}

#[test]
fn closeable_tab_strip_context_menu_click_uses_rendered_item_id() -> Result<(), String> {
    let mut state = closeable_state();
    open_tab_context_menu(&mut state, "scratch.md")?;
    state
        .screen_state
        .tabs
        .context_menu
        .as_mut()
        .ok_or_else(|| "context menu is missing".to_string())?
        .items[0]
        .id = "pin".to_string();
    click_open_context_command(&mut state, 0)?;

    assert_eq!("tab_context_pin", state.screen_state.last_action);
    assert_eq!("closeable_tab_pin_changed", state.screen_state.last_event);
    assert!(active_tab(&state)?.pinned);
    Ok(())
}

#[test]
fn closeable_tab_strip_context_menu_pin_removes_group_membership() -> Result<(), String> {
    let mut state = closeable_state();
    click_tab_context_command(&mut state, "editor.rs", PIN_INDEX)?;

    assert_eq!("tab_context_pin", state.screen_state.last_action);
    assert_eq!("closeable_tab_group_changed", state.screen_state.last_event);
    assert!(active_tab(&state)?.pinned);
    assert_eq!(None, active_tab(&state)?.group_id.as_deref());
    Ok(())
}

#[test]
fn closeable_tab_strip_context_menu_restores_last_closed_tab() -> Result<(), String> {
    let mut state = closeable_state();
    click_tab_context_command(&mut state, "terminal", CLOSE_INDEX)?;
    assert!(!has_tab(&state, "terminal"));

    open_tab_context_menu(&mut state, "scratch.md")?;
    click_open_context_command(&mut state, RESTORE_CLOSED_INDEX)?;

    assert_eq!("tab_context_restore_closed", state.screen_state.last_action);
    assert_eq!("closeable_tab_restored", state.screen_state.last_event);
    assert!(has_tab(&state, "terminal"));
    assert_eq!("terminal", active_tab(&state)?.id);
    Ok(())
}

pub(super) fn closeable_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn click_tab_context_command(
    state: &mut StorybookWindowState,
    tab_id: &str,
    command_index: usize,
) -> Result<(), String> {
    open_tab_context_menu(state, tab_id)?;
    click_open_context_command(state, command_index)
}

pub(super) fn open_tab_context_menu(
    state: &mut StorybookWindowState,
    tab_id: &str,
) -> Result<(), String> {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let tab = dedicated_closeable_tab_strip::tab_rect_for_test(&state.screen_state.tabs, tab_id)
        .ok_or_else(|| format!("tab rect is missing for {tab_id}"))?;

    assert!(super::window_interaction::apply_context_click_for_test(
        state,
        component.x + tab.x + 1,
        component.y + tab.y + 1,
    ));
    Ok(())
}

pub(super) fn open_group_context_menu(
    state: &mut StorybookWindowState,
    group_id: &str,
) -> Result<(), String> {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let group =
        dedicated_closeable_tab_strip::group_rect_for_test(&state.screen_state.tabs, group_id)
            .ok_or_else(|| "group rect is missing".to_string())?;

    assert!(super::window_interaction::apply_context_click_for_test(
        state,
        component.x + group.x + 1,
        component.y + group.y + 1,
    ));
    Ok(())
}

pub(super) fn has_tab(state: &StorybookWindowState, tab_id: &str) -> bool {
    state
        .screen_state
        .tabs
        .tabs
        .iter()
        .any(|tab| tab.id == tab_id)
}

fn tab_order(state: &StorybookWindowState) -> Vec<&str> {
    state
        .screen_state
        .tabs
        .tabs
        .iter()
        .map(|tab| tab.id.as_str())
        .collect()
}

fn active_tab(
    state: &StorybookWindowState,
) -> Result<&super::screen_state_tabs::TabsScreenTab, String> {
    state
        .screen_state
        .tabs
        .active_tab()
        .ok_or_else(|| "active tab is missing".to_string())
}

pub(super) fn click_open_context_command(
    state: &mut StorybookWindowState,
    command_index: usize,
) -> Result<(), String> {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let labels =
        dedicated_closeable_tab_strip::context_menu_labels_for_test(&state.screen_state.tabs);
    let menu = dedicated_closeable_tab_strip::context_menu_rect_for_test(&state.screen_state.tabs)
        .ok_or_else(|| "context menu rect is missing".to_string())?;
    let row_height = menu.height / labels.len();

    assert!(apply_click(
        state,
        component.x + menu.x + 1,
        component.y + menu.y + command_index * row_height + 1,
    ));
    Ok(())
}
