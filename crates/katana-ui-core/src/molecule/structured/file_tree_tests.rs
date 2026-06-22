use super::{FileTree, FileTreeAction, FileTreeItem, FileTreeState};
use crate::interaction::{UiAction, UiActionSource};
use crate::render_model::{UiHostActionPlan, UiNodeKind, UiTreeNodeKind};

#[test]
fn render_builds_nested_tree_view() {
    let tree = FileTree::render(
        &[
            FileTreeItem::new("katana/a.md", "katana/a.md"),
            FileTreeItem::new("katana/nested/b.md", "katana/nested/b.md").icon("markdown"),
        ],
        "katana/nested/b.md",
        240,
        480,
    );

    let scroll_area = tree.root();
    assert_eq!(UiNodeKind::ScrollArea, scroll_area.kind());
    assert_eq!(1, scroll_area.children().len());
    let tree_view = &scroll_area.children()[0];
    assert_eq!(UiNodeKind::TreeView, tree_view.kind());

    let nodes = &tree_view.props().tree.nodes;
    assert!(
        nodes
            .iter()
            .any(|node| node.kind == UiTreeNodeKind::Directory
                && node.id == "katana"
                && node.depth == 0)
    );
    assert!(
        nodes
            .iter()
            .any(|node| node.kind == UiTreeNodeKind::Directory
                && node.id == "katana/nested"
                && node.depth == 1)
    );
    assert!(nodes.iter().any(|node| node.kind == UiTreeNodeKind::File
        && node.id == "katana/nested/b.md"
        && node.depth == 2
        && node.icon == "markdown"
        && node.selected
        && node.active));
    assert!(tree_view.props().tree.line_display);
    assert_eq!("folder", tree_view.props().tree.directory_icon);
    assert_eq!("file", tree_view.props().tree.file_icon);
}

#[test]
fn render_with_state_passes_hovered_item_to_tree_view_props() {
    let items = sample_items();
    let state = FileTreeState::default().hovered("katana/nested/b.md");
    let tree = FileTree::render_with_state_and_offset(&items, "katana/a.md", 240, 480, 0, &state);
    let tree_view = &tree.root().children()[0];

    assert_eq!("katana/nested/b.md", tree_view.props().tree.hovered_id);
}

#[test]
fn selected_item_id_reads_adapter_value_action() {
    let actions = vec![UiAction::SetValue {
        target: "files".into(),
        value: "katana/sample.md".to_string(),
        source: UiActionSource::Generic,
        progress: None,
        color_drag: None,
    }];

    assert_eq!(
        Some("katana/sample.md"),
        FileTree::selected_item_id(&actions)
    );
}

#[test]
fn action_from_host_plan_maps_tree_file_row_to_file_selection() -> Result<(), String> {
    let tree = FileTree::render(&sample_items(), "katana/a.md", 240, 480);
    let tree_view = &tree.root().children()[0];
    let plan = UiHostActionPlan::collect_from_root(tree_view)
        .into_iter()
        .find(|plan| {
            plan.tree_row_action_target()
                .is_some_and(|target| target.node_id == "katana/nested/b.md")
        })
        .ok_or_else(|| "file row host action missing".to_string())?;

    assert_eq!(
        Some(FileTreeAction::SelectFile {
            file_id: "katana/nested/b.md".to_string(),
        }),
        FileTree::action_from_host_plan(&plan)
    );
    Ok(())
}

#[test]
fn action_from_host_plan_maps_tree_directory_row_to_directory_toggle() -> Result<(), String> {
    let tree = FileTree::render(&sample_items(), "katana/a.md", 240, 480);
    let tree_view = &tree.root().children()[0];
    let plan = UiHostActionPlan::collect_from_root(tree_view)
        .into_iter()
        .find(|plan| {
            plan.tree_row_action_target()
                .is_some_and(|target| target.node_id == "katana/nested")
        })
        .ok_or_else(|| "directory row host action missing".to_string())?;

    assert_eq!(
        Some(FileTreeAction::ToggleDirectory {
            directory_id: "katana/nested".to_string(),
        }),
        FileTree::action_from_host_plan(&plan)
    );
    Ok(())
}

#[test]
fn render_with_offset_applies_scroll_area_offset() {
    let items = (0..32)
        .map(|index| FileTreeItem::new(format!("katana/{index}.md"), format!("katana/{index}.md")))
        .collect::<Vec<_>>();

    let tree = FileTree::render_with_offset(&items, "katana/31.md", 240, 120, 240);

    let scroll_area = tree.root();
    assert_eq!(UiNodeKind::ScrollArea, scroll_area.kind());
    assert_eq!(240, scroll_area.props().scroll_area.offset_y);
    assert!(
        scroll_area.props().scroll_area.content_height
            > scroll_area.props().scroll_area.viewport_height
    );
}

#[test]
fn content_height_with_state_matches_rendered_scroll_extent() {
    let items = sample_items();
    let state = FileTreeState::default().collapsed("katana/nested");
    let tree = FileTree::render_with_state_and_offset(&items, "katana/a.md", 240, 120, 0, &state);

    assert_eq!(
        tree.root().props().scroll_area.content_height,
        FileTree::content_height_with_state(&items, 120, &state)
    );
}

#[test]
fn render_with_state_hides_files_inside_collapsed_directory() {
    let items = sample_items();
    let state = FileTreeState::default().collapsed("katana/nested");
    let tree =
        FileTree::render_with_state_and_offset(&items, "katana/nested/b.md", 240, 480, 0, &state);
    let nodes = &tree.root().children()[0].props().tree.nodes;

    assert!(nodes.iter().any(|node| node.id == "katana/nested"
        && node.kind == UiTreeNodeKind::Directory
        && !node.expanded));
    assert!(!nodes.iter().any(|node| node.id == "katana/nested/b.md"));
}

#[test]
fn state_toggle_directory_collapses_and_expands_directory() {
    let mut state = FileTreeState::default();

    state.toggle_directory("katana/nested");
    assert!(state.is_collapsed("katana/nested"));

    state.toggle_directory("katana/nested");
    assert!(!state.is_collapsed("katana/nested"));
}

fn sample_items() -> Vec<FileTreeItem> {
    vec![
        FileTreeItem::new("katana/a.md", "katana/a.md"),
        FileTreeItem::new("katana/nested/b.md", "katana/nested/b.md").icon("markdown"),
    ]
}
