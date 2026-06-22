use super::preview_detail;
use super::window_interaction::{StorybookWindowState, apply_hover_at};

const HOVER_EVENT_PAGES: &[&str] = &[
    "accordion",
    "code-diff",
    "collapsible-panel",
    "virtualization",
    "diagnostics-list",
    "empty-state",
    "tree-view",
    "drag-and-drop",
    "panel",
    "row",
    "column",
    "stack",
    "grid",
    "align-center",
    "scroll-area",
    "split-pane",
    "theme-tokens",
    "color-picker-rgba",
    "combo-box",
    "command-palette",
    "dynamic-array-editor",
    "toast-stack-manager",
    "notification-toast",
    "tooltip",
    "popover",
    "hover-card",
    "search-control-strip",
    "segmented-toggle",
    "shortcut-combo",
    "shortcut-cheatsheet",
    "skeleton-cluster",
    "motion",
    "window-control-button-group",
    "startup-state-panel",
    "attachment-chip",
    "chip-group",
    "side-menu",
    "select-box",
    "selection-list",
    "settings-list",
    "modal-overlay",
];

#[test]
fn repeated_hover_at_same_target_is_idempotent_for_event_pages() {
    for page in HOVER_EVENT_PAGES {
        let mut state = StorybookWindowState {
            selected_page: page,
            ..StorybookWindowState::default()
        };
        let target = if *page == "tooltip" {
            super::dedicated_tooltip::anchor_hit_rect(state.preset_index)
        } else {
            preview_detail::component_action_hit_rect(page)
        };
        let x = target.x + 1;
        let y = target.y + 1;

        assert!(apply_hover_at(&mut state, x, y), "{page} first hover");
        let action_count = state.screen_state.action_count;
        let last_action = state.screen_state.last_action.to_string();
        let last_event = state.screen_state.last_event.to_string();
        let state_label = state.screen_state.state_label.to_string();

        assert!(apply_hover_at(&mut state, x, y), "{page} repeated hover");

        assert_eq!(action_count, state.screen_state.action_count, "{page}");
        assert_eq!(last_action, state.screen_state.last_action, "{page}");
        assert_eq!(last_event, state.screen_state.last_event, "{page}");
        assert_eq!(state_label, state.screen_state.state_label, "{page}");
    }
}

#[test]
fn repeated_tree_view_hover_after_scroll_count_change_does_not_emit_again() {
    let mut state = StorybookWindowState {
        selected_page: "tree-view",
        ..StorybookWindowState::default()
    };
    let target = preview_detail::component_action_hit_rect("tree-view");
    let x = target.x + 1;
    let y = target.y + 1;

    assert!(apply_hover_at(&mut state, x, y));
    state.screen_state.action_count += 1;
    let action_count = state.screen_state.action_count;

    assert!(apply_hover_at(&mut state, x, y));

    assert_eq!(action_count, state.screen_state.action_count);
}
