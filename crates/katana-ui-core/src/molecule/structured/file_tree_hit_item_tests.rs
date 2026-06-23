use super::{FileTree, FileTreeAction, FileTreeHitTestInput, FileTreeItem, FileTreeState};
use crate::render_model::UiCursor;

#[test]
fn hit_target_for_item_returns_file_selection_contract() -> Result<(), String> {
    let items = sample_items();
    let target = FileTree::hit_target_for_item_with_state(
        &items,
        &FileTreeState::default(),
        "katana/a.md",
        0,
        240,
    )
    .ok_or_else(|| "file target".to_string())?;
    let center_x = target.rect.x + target.rect.width / 2;
    let center_y = target.rect.y + target.rect.height / 2;

    assert_eq!("katana/a.md", target.item_id);
    assert_eq!(UiCursor::Pointer, target.cursor);
    assert_eq!(
        FileTreeAction::SelectFile {
            file_id: "katana/a.md".to_string(),
        },
        target.action
    );
    assert_eq!(
        target.action,
        FileTree::hit_test_with_state(
            &items,
            &FileTreeState::default(),
            FileTreeHitTestInput {
                pointer_x: center_x,
                pointer_y: center_y,
                scroll_offset_y: 0,
            },
        )
    );
    Ok(())
}

#[test]
fn hit_target_for_item_returns_directory_toggle_contract() -> Result<(), String> {
    let items = sample_items();
    let target = FileTree::hit_target_for_item_with_state(
        &items,
        &FileTreeState::default(),
        "katana",
        0,
        240,
    )
    .ok_or_else(|| "directory target".to_string())?;
    let center_x = target.rect.x + target.rect.width / 2;
    let center_y = target.rect.y + target.rect.height / 2;

    assert_eq!("katana", target.item_id);
    assert_eq!(UiCursor::Pointer, target.cursor);
    assert_eq!(
        FileTreeAction::ToggleDirectory {
            directory_id: "katana".to_string(),
        },
        target.action
    );
    assert_eq!(
        target.action,
        FileTree::hit_test_with_state(
            &items,
            &FileTreeState::default(),
            FileTreeHitTestInput {
                pointer_x: center_x,
                pointer_y: center_y,
                scroll_offset_y: 0,
            },
        )
    );
    Ok(())
}

fn sample_items() -> Vec<FileTreeItem> {
    vec![
        FileTreeItem::new("katana/a.md", "katana/a.md"),
        FileTreeItem::new("katana/nested/b.md", "katana/nested/b.md").icon("markdown"),
    ]
}
