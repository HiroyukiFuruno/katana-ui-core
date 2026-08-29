use super::super::{
    StorybookWindowState, apply_align_center_resize_for_audit,
    apply_code_diff_scroll_sync_for_audit, apply_column_resize_for_audit,
    apply_command_palette_escape_for_audit, apply_diagnostics_list_scroll_for_audit,
    apply_drag_and_drop_drag_for_audit, apply_drag_and_drop_resize_for_audit,
    apply_drag_and_drop_scroll_for_audit, apply_grid_resize_for_audit, apply_list_scroll_for_audit,
    apply_panel_resize_for_audit, apply_row_resize_for_audit, apply_scroll_area_drag_for_audit,
    apply_scroll_area_resize_for_audit, apply_scroll_area_scroll_for_audit,
    apply_select_scroll_for_audit, apply_selection_list_scroll_for_audit,
    apply_settings_list_scroll_for_audit, apply_shortcut_cheatsheet_scroll_for_audit,
    apply_side_menu_scroll_for_audit, apply_slide_drag_for_audit, apply_split_pane_drag_for_audit,
    apply_split_pane_resize_for_audit, apply_stack_resize_for_audit,
    apply_theme_tokens_resize_for_audit, apply_tree_view_scroll_for_audit,
    apply_virtualization_scroll_for_audit,
};

type AuditOperation = fn(&mut StorybookWindowState, usize, usize) -> bool;

#[test]
fn audit_operations_reject_wrong_pages_and_points_outside_the_component() {
    for &(page, operation) in audit_operations() {
        let mut wrong_page = StorybookWindowState::default();
        assert!(!operation(&mut wrong_page, 0, 0), "{page}: wrong page");

        let mut outside = state_for(page);
        assert!(!operation(&mut outside, 0, 0), "{page}: outside point");
    }
}

#[test]
fn command_palette_escape_rejects_other_pages_and_closes_the_palette() {
    let mut wrong_page = StorybookWindowState::default();
    assert!(!apply_command_palette_escape_for_audit(&mut wrong_page));

    let mut palette = state_for("command-palette");
    assert!(apply_command_palette_escape_for_audit(&mut palette));
}

fn state_for(selected_page: &'static str) -> StorybookWindowState {
    StorybookWindowState {
        selected_page,
        ..StorybookWindowState::default()
    }
}

fn audit_operations() -> &'static [(&'static str, AuditOperation)] {
    &[
        ("slide-control", apply_slide_drag_for_audit),
        ("drag-and-drop", apply_drag_and_drop_drag_for_audit),
        ("drag-and-drop", apply_drag_and_drop_scroll_for_audit),
        ("drag-and-drop", apply_drag_and_drop_resize_for_audit),
        ("panel", apply_panel_resize_for_audit),
        ("row", apply_row_resize_for_audit),
        ("column", apply_column_resize_for_audit),
        ("stack", apply_stack_resize_for_audit),
        ("grid", apply_grid_resize_for_audit),
        ("align-center", apply_align_center_resize_for_audit),
        ("theme-tokens", apply_theme_tokens_resize_for_audit),
        ("scroll-area", apply_scroll_area_scroll_for_audit),
        ("scroll-area", apply_scroll_area_drag_for_audit),
        ("scroll-area", apply_scroll_area_resize_for_audit),
        ("split-pane", apply_split_pane_drag_for_audit),
        ("split-pane", apply_split_pane_resize_for_audit),
        ("list", apply_list_scroll_for_audit),
        ("select-box", apply_select_scroll_for_audit),
        ("selection-list", apply_selection_list_scroll_for_audit),
        ("tree-view", apply_tree_view_scroll_for_audit),
        ("side-menu", apply_side_menu_scroll_for_audit),
        (
            "shortcut-cheatsheet",
            apply_shortcut_cheatsheet_scroll_for_audit,
        ),
        ("settings-list", apply_settings_list_scroll_for_audit),
        ("diagnostics-list", apply_diagnostics_list_scroll_for_audit),
        ("virtualization", apply_virtualization_scroll_for_audit),
        ("code-diff", apply_code_diff_scroll_sync_for_audit),
    ]
}
