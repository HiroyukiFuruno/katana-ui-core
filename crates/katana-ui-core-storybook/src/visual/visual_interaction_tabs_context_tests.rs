use super::dedicated_tabs;
use super::visual_interaction_tabs_context_support::*;
use super::visual_interaction_test_support::require_some;

#[test]
fn tabs_context_menu_commands_apply_close_pin_and_group_actions() -> Result<(), String> {
    let mut close_state = tabs_state();
    click_context_command(&mut close_state, "terminal", CLOSE_INDEX)?;
    assert_eq!("tab_context_close", close_state.screen_state.last_action);
    assert!(!has_tab(&close_state, "terminal"));

    let mut others_state = tabs_state();
    click_context_command(&mut others_state, "editor.rs", CLOSE_OTHERS_INDEX)?;
    assert_eq!(
        "tab_context_close_others",
        others_state.screen_state.last_action
    );
    assert_eq!(vec!["readme.md", "editor.rs"], tab_order(&others_state));

    let mut right_state = tabs_state();
    click_context_command(&mut right_state, "editor.rs", CLOSE_RIGHT_INDEX)?;
    assert_eq!(
        "tab_context_close_right",
        right_state.screen_state.last_action
    );
    assert_eq!(vec!["readme.md", "editor.rs"], tab_order(&right_state));

    let mut left_state = tabs_state();
    click_context_command(&mut left_state, "terminal", CLOSE_LEFT_INDEX)?;
    assert_eq!(
        "tab_context_close_left",
        left_state.screen_state.last_action
    );
    assert_eq!(vec!["readme.md", "terminal"], tab_order(&left_state));

    let mut all_state = tabs_state();
    click_context_command(&mut all_state, "scratch.md", CLOSE_ALL_INDEX)?;
    assert_eq!("tab_context_close_all", all_state.screen_state.last_action);
    assert_eq!(vec!["readme.md"], tab_order(&all_state));

    let mut pin_state = tabs_state();
    click_context_command(&mut pin_state, "scratch.md", PIN_INDEX)?;
    assert_eq!("tab_context_pin", pin_state.screen_state.last_action);
    assert_eq!(
        "closeable_tab_pin_changed",
        pin_state.screen_state.last_event
    );
    assert!(active_tab(&pin_state)?.pinned);

    let mut group_state = tabs_state();
    click_context_command(&mut group_state, "scratch.md", NEW_GROUP_INDEX)?;
    assert_eq!(
        "tab_context_new_group",
        group_state.screen_state.last_action
    );
    assert_eq!(
        "closeable_tab_group_changed",
        group_state.screen_state.last_event
    );
    assert_eq!(
        Some("context-group"),
        active_tab(&group_state)?.group_id.as_deref()
    );

    let mut existing_group_state = tabs_state();
    click_context_command(&mut existing_group_state, "scratch.md", DOCS_GROUP_INDEX)?;
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
fn tabs_context_menu_labels_match_workspace_tab_commands() -> Result<(), String> {
    let mut regular_state = tabs_state();
    open_context_menu(&mut regular_state, "scratch.md")?;
    let menu = require_some(
        regular_state.screen_state.tabs.context_menu.as_ref(),
        "context menu state",
    )?;
    assert_eq!(
        vec![
            "閉じる",
            "他のタブを閉じる",
            "すべて閉じる",
            "右側のタブを閉じる",
            "左側のタブを閉じる",
            "ピン留め",
            "グループに追加",
            "新しいグループを作成",
            "Docs",
        ],
        dedicated_tabs::context_menu_labels_for_test(&regular_state.screen_state.tabs)
    );
    assert_eq!("close", menu.items[0].id);
    assert_eq!("move-to-group", menu.items[MOVE_TO_GROUP_INDEX].id);
    assert_eq!(
        "move-to-new-group",
        menu.items[MOVE_TO_GROUP_INDEX].children[0].id
    );
    assert_eq!(
        "move-to-group:docs",
        menu.items[MOVE_TO_GROUP_INDEX].children[1].id
    );

    let mut pinned_state = tabs_state();
    open_context_menu(&mut pinned_state, "readme.md")?;
    let pinned_labels =
        dedicated_tabs::context_menu_labels_for_test(&pinned_state.screen_state.tabs);
    assert!(pinned_labels.contains(&"ピン留めを解除"));
    assert!(!pinned_labels.contains(&"新しいグループを作成"));
    assert!(!pinned_labels.contains(&"グループに追加"));
    Ok(())
}

#[test]
fn tabs_context_menu_without_existing_groups_uses_direct_new_group_action() -> Result<(), String> {
    let mut state = tabs_state();
    state.screen_state.tabs.groups.clear();
    open_context_menu(&mut state, "scratch.md")?;
    let command_index = {
        let labels = dedicated_tabs::context_menu_labels_for_test(&state.screen_state.tabs);
        assert!(labels.contains(&"新しいグループを作成"));
        assert!(!labels.contains(&"グループに追加"));
        label_index(&labels, "新しいグループを作成")?
    };
    click_open_context_command(&mut state, command_index)?;

    assert_eq!("tab_context_new_group", state.screen_state.last_action);
    assert_eq!("closeable_tab_group_changed", state.screen_state.last_event);
    assert_eq!(
        Some("context-group"),
        active_tab(&state)?.group_id.as_deref()
    );
    Ok(())
}

#[test]
fn tabs_context_menu_moves_to_selected_existing_group_not_fixed_default() -> Result<(), String> {
    let mut state = tabs_state();
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
    open_context_menu(&mut state, "scratch.md")?;
    let labels = dedicated_tabs::context_menu_labels_for_test(&state.screen_state.tabs);
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

fn label_index(labels: &[&str], expected: &str) -> Result<usize, String> {
    labels
        .iter()
        .position(|label| *label == expected)
        .ok_or_else(|| expected.to_string())
}

#[test]
fn tabs_context_menu_hides_group_commands_for_ungroupable_tab() -> Result<(), String> {
    let mut state = tabs_state();
    let scratch = state
        .screen_state
        .tabs
        .tabs
        .iter_mut()
        .find(|tab| tab.id == "scratch.md")
        .ok_or_else(|| "scratch tab is missing".to_string())?;
    scratch.groupable = false;
    open_context_menu(&mut state, "scratch.md")?;

    let labels = dedicated_tabs::context_menu_labels_for_test(&state.screen_state.tabs);
    assert!(!labels.contains(&"新しいグループを作成"));
    assert!(!labels.contains(&"グループに追加"));
    Ok(())
}

#[test]
fn tabs_context_menu_click_uses_rendered_item_id_not_parallel_index() -> Result<(), String> {
    let mut state = tabs_state();
    open_context_menu(&mut state, "scratch.md")?;
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
fn tabs_context_menu_pin_removes_group_membership() -> Result<(), String> {
    let mut state = tabs_state();
    click_context_command(&mut state, "editor.rs", PIN_INDEX)?;

    assert_eq!("tab_context_pin", state.screen_state.last_action);
    assert_eq!("closeable_tab_group_changed", state.screen_state.last_event);
    assert!(active_tab(&state)?.pinned);
    assert_eq!(None, active_tab(&state)?.group_id.as_deref());
    assert_eq!(
        vec![
            "readme.md",
            "editor.rs",
            "preview.rs",
            "scratch.md",
            "terminal"
        ],
        tab_order(&state)
    );
    Ok(())
}

#[test]
fn tabs_context_menu_keeps_pinned_tabs_fixed_until_unpinned() -> Result<(), String> {
    let mut close_blocked = tabs_state();
    click_context_command(&mut close_blocked, "readme.md", CLOSE_INDEX)?;
    assert!(has_tab(&close_blocked, "readme.md"));
    assert_eq!(
        "closeable_tab_context_close_blocked",
        close_blocked.screen_state.last_event
    );

    let mut unpinned = tabs_state();
    click_context_command(&mut unpinned, "readme.md", PIN_INDEX)?;
    assert!(!active_tab(&unpinned)?.pinned);
    assert_eq!("tab_context_pin", unpinned.screen_state.last_action);
    click_context_command(&mut unpinned, "readme.md", CLOSE_INDEX)?;
    assert!(!has_tab(&unpinned, "readme.md"));
    Ok(())
}

#[test]
fn tabs_context_menu_restores_last_closed_tab_through_core_action() -> Result<(), String> {
    let mut state = tabs_state();
    click_context_command(&mut state, "terminal", CLOSE_INDEX)?;
    assert!(!has_tab(&state, "terminal"));

    open_context_menu(&mut state, "scratch.md")?;
    assert!(
        dedicated_tabs::context_menu_labels_for_test(&state.screen_state.tabs)
            .contains(&"閉じたタブを復元")
    );
    click_open_context_command(&mut state, RESTORE_CLOSED_INDEX)?;

    assert_eq!("tab_context_restore_closed", state.screen_state.last_action);
    assert_eq!("closeable_tab_restored", state.screen_state.last_event);
    assert!(has_tab(&state, "terminal"));
    assert_eq!("terminal", active_tab(&state)?.id);
    Ok(())
}

#[test]
fn tabs_context_menu_close_right_follows_visual_order_after_pin() -> Result<(), String> {
    let mut state = tabs_state();

    click_context_command(&mut state, "scratch.md", PIN_INDEX)?;
    assert!(active_tab(&state)?.pinned);
    click_context_command(&mut state, "scratch.md", CLOSE_RIGHT_INDEX)?;

    assert_eq!("tab_context_close_right", state.screen_state.last_action);
    assert_eq!(vec!["readme.md", "scratch.md"], tab_order(&state));
    Ok(())
}
