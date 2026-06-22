use super::dedicated_closeable_tab_strip;
use super::visual_interaction_closeable_tab_strip_context_tests::{
    click_open_context_command, closeable_state, has_tab, open_group_context_menu,
};

const GROUP_COLOR_INDEX: usize = 1;
const GROUP_COLLAPSE_INDEX: usize = 2;
const GROUP_MOVE_INDEX: usize = 3;
const GROUP_UNGROUP_INDEX: usize = 4;
const GROUP_CLOSE_INDEX: usize = 5;

#[test]
fn closeable_tab_strip_group_header_context_menu_uses_real_core_commands() -> Result<(), String> {
    let mut state = closeable_state();
    open_group_context_menu(&mut state, "docs")?;

    assert_eq!("group_context_menu", state.screen_state.last_action);
    assert_eq!(
        "closeable_tab_group_context_menu_opened",
        state.screen_state.last_event
    );
    assert_eq!(
        group_menu_labels(),
        dedicated_closeable_tab_strip::context_menu_labels_for_test(&state.screen_state.tabs)
    );

    click_open_context_command(&mut state, 0)?;
    assert_eq!("group_context_rename", state.screen_state.last_action);
    assert_eq!("closeable_tab_group_renamed", state.screen_state.last_event);
    assert!(
        state
            .screen_state
            .tabs
            .groups
            .iter()
            .any(|group| group.id == "docs" && group.title == "Reference")
    );

    open_group_context_menu(&mut state, "docs")?;
    click_open_context_command(&mut state, GROUP_COLLAPSE_INDEX)?;
    assert_eq!("group_context_toggle", state.screen_state.last_action);
    assert_eq!(
        "closeable_tab_group_collapse_changed",
        state.screen_state.last_event
    );
    assert!(
        state
            .screen_state
            .tabs
            .groups
            .iter()
            .any(|group| group.id == "docs" && group.collapsed)
    );
    Ok(())
}

#[test]
fn closeable_tab_strip_group_context_menu_applies_color_and_close() -> Result<(), String> {
    let mut color_state = closeable_state();
    open_group_context_menu(&mut color_state, "docs")?;
    click_open_context_command(&mut color_state, GROUP_COLOR_INDEX)?;
    assert_eq!("group_context_color", color_state.screen_state.last_action);
    assert_eq!(
        "closeable_tab_group_color_changed",
        color_state.screen_state.last_event
    );
    assert!(
        color_state
            .screen_state
            .tabs
            .groups
            .iter()
            .any(|group| group.id == "docs" && group.color == 0x5aa65a)
    );

    let mut close_state = closeable_state();
    open_group_context_menu(&mut close_state, "docs")?;
    click_open_context_command(&mut close_state, GROUP_CLOSE_INDEX)?;
    assert_eq!(
        "group_context_close_group",
        close_state.screen_state.last_action
    );
    assert_eq!(
        "closeable_tab_group_removed",
        close_state.screen_state.last_event
    );
    assert!(!has_tab(&close_state, "editor.rs"));
    assert!(!has_tab(&close_state, "preview.rs"));
    Ok(())
}

#[test]
fn closeable_tab_strip_group_header_context_menu_moves_ungroups_and_uses_rendered_item_ids()
-> Result<(), String> {
    let mut id_route_state = closeable_state();
    open_group_context_menu(&mut id_route_state, "docs")?;
    id_route_state
        .screen_state
        .tabs
        .context_menu
        .as_mut()
        .ok_or_else(|| "group context menu is missing".to_string())?
        .items[1]
        .id = "rename".to_string();
    click_open_context_command(&mut id_route_state, 1)?;
    assert_eq!(
        "group_context_rename",
        id_route_state.screen_state.last_action
    );
    assert_eq!(
        "closeable_tab_group_renamed",
        id_route_state.screen_state.last_event
    );

    let mut move_state = closeable_state();
    move_state
        .screen_state
        .tabs
        .groups
        .push(super::screen_state_tabs::TabsScreenGroup {
            id: "review".to_string(),
            title: "Review".to_string(),
            color: 0x5aa65a,
            collapsed: false,
        });
    open_group_context_menu(&mut move_state, "docs")?;
    click_open_context_command(&mut move_state, GROUP_MOVE_INDEX)?;
    assert_eq!("group_context_move", move_state.screen_state.last_action);
    assert_eq!(
        "closeable_tab_group_reordered",
        move_state.screen_state.last_event
    );
    assert_eq!(
        vec!["review", "docs"],
        move_state
            .screen_state
            .tabs
            .groups
            .iter()
            .map(|group| group.id.as_str())
            .collect::<Vec<_>>()
    );

    let mut ungroup_state = closeable_state();
    open_group_context_menu(&mut ungroup_state, "docs")?;
    click_open_context_command(&mut ungroup_state, GROUP_UNGROUP_INDEX)?;
    assert_eq!(
        "group_context_ungroup",
        ungroup_state.screen_state.last_action
    );
    assert_eq!(
        "closeable_tab_group_removed",
        ungroup_state.screen_state.last_event
    );
    assert!(ungroup_state.screen_state.tabs.groups.is_empty());
    assert!(
        ungroup_state
            .screen_state
            .tabs
            .tabs
            .iter()
            .all(|tab| tab.group_id.is_none())
    );
    Ok(())
}

fn group_menu_labels() -> Vec<&'static str> {
    vec![
        "グループ名を変更",
        "グループ色を変更",
        "グループを折りたたむ",
        "グループを移動",
        "グループ解除",
        "グループを閉じる",
    ]
}
