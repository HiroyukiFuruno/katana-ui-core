use super::{
    FileTree, FileTreeAction, FileTreeHitRect, FileTreeHitTestInput, FileTreeItem, FileTreeState,
};
use crate::render_model::UiCursor;

#[test]
fn hit_test_returns_directory_toggle_for_directory_row() {
    let items = sample_items();
    let action = FileTree::hit_test(
        &items,
        FileTreeHitTestInput {
            pointer_x: 24,
            pointer_y: 25,
            scroll_offset_y: 0,
        },
    );

    assert_eq!(
        FileTreeAction::ToggleDirectory {
            directory_id: "katana".to_string()
        },
        action
    );
}

#[test]
fn hit_test_ignores_file_tree_label_row() {
    let items = sample_items();
    let action = FileTree::hit_test(
        &items,
        FileTreeHitTestInput {
            pointer_x: 24,
            pointer_y: 1,
            scroll_offset_y: 0,
        },
    );

    assert_eq!(FileTreeAction::None, action);
}

#[test]
fn hit_test_returns_file_selection_for_file_row() {
    let items = sample_items();
    let action = FileTree::hit_test(
        &items,
        FileTreeHitTestInput {
            pointer_x: 24,
            pointer_y: 97,
            scroll_offset_y: 0,
        },
    );

    assert_eq!(
        FileTreeAction::SelectFile {
            file_id: "katana/nested/b.md".to_string()
        },
        action
    );
}

#[test]
fn hit_test_accounts_for_scroll_offset() {
    let items = sample_items();
    let action = FileTree::hit_test(
        &items,
        FileTreeHitTestInput {
            pointer_x: 24,
            pointer_y: 1,
            scroll_offset_y: 96,
        },
    );

    assert_eq!(
        FileTreeAction::SelectFile {
            file_id: "katana/nested/b.md".to_string()
        },
        action
    );
}

#[test]
fn hit_target_returns_directory_row_contract() -> Result<(), String> {
    let items = sample_items();
    let target = FileTree::hit_target_with_state(
        &items,
        &FileTreeState::default(),
        FileTreeHitTestInput {
            pointer_x: 24,
            pointer_y: 25,
            scroll_offset_y: 0,
        },
        240,
    )
    .ok_or_else(|| "directory row should expose a KUC hit target".to_string())?;

    assert_eq!("katana", target.item_id);
    assert_eq!(
        FileTreeAction::ToggleDirectory {
            directory_id: "katana".to_string()
        },
        target.action
    );
    assert_eq!(UiCursor::Pointer, target.cursor);
    assert_eq!(
        FileTreeHitRect {
            x: 0,
            y: 22,
            width: 240,
            height: 22,
        },
        target.rect
    );
    Ok(())
}

#[test]
fn hit_target_returns_file_row_contract() -> Result<(), String> {
    let items = sample_items();
    let target = FileTree::hit_target_with_state(
        &items,
        &FileTreeState::default(),
        FileTreeHitTestInput {
            pointer_x: 24,
            pointer_y: 97,
            scroll_offset_y: 0,
        },
        240,
    )
    .ok_or_else(|| "file row should expose a KUC hit target".to_string())?;

    assert_eq!("katana/nested/b.md", target.item_id);
    assert_eq!(
        FileTreeAction::SelectFile {
            file_id: "katana/nested/b.md".to_string()
        },
        target.action
    );
    assert_eq!(UiCursor::Pointer, target.cursor);
    assert_eq!(
        FileTreeHitRect {
            x: 0,
            y: 88,
            width: 240,
            height: 22,
        },
        target.rect
    );
    Ok(())
}

#[test]
fn hit_target_accounts_for_scroll_offset() -> Result<(), String> {
    let items = sample_items();
    let target = FileTree::hit_target_with_state(
        &items,
        &FileTreeState::default(),
        FileTreeHitTestInput {
            pointer_x: 24,
            pointer_y: 1,
            scroll_offset_y: 96,
        },
        240,
    )
    .ok_or_else(|| "scrolled file row should expose a KUC hit target".to_string())?;

    assert_eq!("katana/nested/b.md", target.item_id);
    assert_eq!(
        FileTreeHitRect {
            x: 0,
            y: 0,
            width: 240,
            height: 22,
        },
        target.rect
    );
    Ok(())
}

#[test]
fn hit_target_ignores_file_tree_label_row() {
    let items = sample_items();
    let target = FileTree::hit_target_with_state(
        &items,
        &FileTreeState::default(),
        FileTreeHitTestInput {
            pointer_x: 24,
            pointer_y: 1,
            scroll_offset_y: 0,
        },
        240,
    );

    assert_eq!(None, target);
}

#[test]
fn hit_test_with_state_skips_files_inside_collapsed_directory() {
    let items = sample_items();
    let state = FileTreeState::default().collapsed("katana/nested");
    let action = FileTree::hit_test_with_state(
        &items,
        &state,
        FileTreeHitTestInput {
            pointer_x: 24,
            pointer_y: 97,
            scroll_offset_y: 0,
        },
    );

    assert_eq!(FileTreeAction::None, action);
}

#[test]
fn cursor_for_hit_uses_tree_view_row_cursor_contract() {
    let items = sample_items();
    let cursor = FileTree::cursor_for_hit_with_state(
        &items,
        &FileTreeState::default(),
        FileTreeHitTestInput {
            pointer_x: 24,
            pointer_y: 25,
            scroll_offset_y: 0,
        },
    );

    assert_eq!(UiCursor::Pointer, cursor);
}

fn sample_items() -> Vec<FileTreeItem> {
    vec![
        FileTreeItem::new("katana/a.md", "katana/a.md"),
        FileTreeItem::new("katana/nested/b.md", "katana/nested/b.md").icon("markdown"),
    ]
}
