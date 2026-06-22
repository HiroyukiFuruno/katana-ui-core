use super::dedicated_closeable_tab_strip;
use super::visual_interaction_closeable_tab_strip_context_tests::{
    click_open_context_command, closeable_state, open_tab_context_menu,
};

#[test]
fn closeable_tab_strip_context_menu_without_existing_groups_uses_direct_new_group_action()
-> Result<(), String> {
    let mut state = closeable_state();
    state.screen_state.tabs.groups.clear();
    open_tab_context_menu(&mut state, "scratch.md")?;
    let command_index = {
        let labels =
            dedicated_closeable_tab_strip::context_menu_labels_for_test(&state.screen_state.tabs);
        assert!(labels.contains(&"新しいグループを作成"));
        assert!(!labels.contains(&"グループに追加"));
        label_index(&labels, "新しいグループを作成")?
    };
    click_open_context_command(&mut state, command_index)?;

    assert_eq!("tab_context_new_group", state.screen_state.last_action);
    assert_eq!("closeable_tab_group_changed", state.screen_state.last_event);
    assert_eq!(
        Some("context-group"),
        state
            .screen_state
            .tabs
            .active_tab()
            .ok_or_else(|| "active tab is missing".to_string())?
            .group_id
            .as_deref()
    );
    Ok(())
}

fn label_index(labels: &[&str], expected: &str) -> Result<usize, String> {
    labels
        .iter()
        .position(|label| *label == expected)
        .ok_or_else(|| expected.to_string())
}
