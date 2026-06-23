use super::dedicated_tabs;
use super::visual_interaction_tabs_context_support::*;

const GROUP_COLOR_INDEX: usize = 1;
const GROUP_COLLAPSE_INDEX: usize = 2;
const GROUP_UNGROUP_INDEX: usize = 4;
const GROUP_CLOSE_INDEX: usize = 5;

#[test]
fn tabs_group_header_context_menu_toggles_collapse_through_core_action() -> Result<(), String> {
    let mut id_route_state = tabs_state();
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
    assert_eq!("Reference", group_by_id(&id_route_state, "docs")?.title);

    let mut rename_state = tabs_state();
    open_group_context_menu(&mut rename_state, "docs")?;
    click_open_context_command(&mut rename_state, 0)?;
    assert_eq!(
        "group_context_rename",
        rename_state.screen_state.last_action
    );
    assert_eq!(
        "closeable_tab_group_renamed",
        rename_state.screen_state.last_event
    );
    assert_eq!("Reference", group_by_id(&rename_state, "docs")?.title);

    let mut collapse_state = tabs_state();
    open_group_context_menu(&mut collapse_state, "docs")?;
    assert_eq!(
        group_menu_labels(false),
        dedicated_tabs::context_menu_labels_for_test(&collapse_state.screen_state.tabs)
    );
    click_open_context_command(&mut collapse_state, GROUP_COLLAPSE_INDEX)?;
    assert_eq!(
        "group_context_toggle",
        collapse_state.screen_state.last_action
    );
    assert_eq!(
        "closeable_tab_group_collapse_changed",
        collapse_state.screen_state.last_event
    );
    assert!(group_by_id(&collapse_state, "docs")?.collapsed);

    open_group_context_menu(&mut collapse_state, "docs")?;
    assert_eq!(
        group_menu_labels(true),
        dedicated_tabs::context_menu_labels_for_test(&collapse_state.screen_state.tabs)
    );
    click_open_context_command(&mut collapse_state, GROUP_COLLAPSE_INDEX)?;
    assert!(!group_by_id(&collapse_state, "docs")?.collapsed);
    Ok(())
}

#[test]
fn tabs_group_header_context_menu_applies_color_ungroup_and_close() -> Result<(), String> {
    let mut color_state = tabs_state();
    open_group_context_menu(&mut color_state, "docs")?;
    click_open_context_command(&mut color_state, GROUP_COLOR_INDEX)?;
    assert_eq!("group_context_color", color_state.screen_state.last_action);
    assert_eq!(
        "closeable_tab_group_color_changed",
        color_state.screen_state.last_event
    );
    assert_eq!(0x5aa65a, group_by_id(&color_state, "docs")?.color);

    let mut ungroup_state = tabs_state();
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

    let mut close_state = tabs_state();
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
    assert!(close_state.screen_state.tabs.groups.is_empty());
    Ok(())
}

fn group_menu_labels(collapsed: bool) -> Vec<&'static str> {
    let toggle = if collapsed {
        "グループを展開"
    } else {
        "グループを折りたたむ"
    };
    vec![
        "グループ名を変更",
        "グループ色を変更",
        toggle,
        "グループを移動",
        "グループ解除",
        "グループを閉じる",
    ]
}
