use super::{FileTreeItem, FileTreeState, TreeNode, TreeView};
use std::collections::BTreeSet;

pub(super) fn tree_view(
    items: &[FileTreeItem],
    selected_id: &str,
    state: &FileTreeState,
) -> TreeView {
    let mut tree = TreeView::new("Files")
        .open(true)
        .icons_visible(true)
        .line_display(true)
        .directory_icon("folder")
        .file_icon("file")
        .active(selected_id);
    if let Some(hovered_id) = state.hovered_item_id() {
        tree = tree.hovered(hovered_id);
    }
    let mut directories = BTreeSet::new();
    for item in items {
        tree = append_item(tree, item, selected_id, &mut directories, state);
    }
    tree
}

fn append_item(
    mut tree: TreeView,
    item: &FileTreeItem,
    selected_id: &str,
    directories: &mut BTreeSet<String>,
    state: &FileTreeState,
) -> TreeView {
    for directory in directory_nodes(item) {
        let directory_id = directory.id.clone();
        if directories.insert(directory_id.clone()) {
            let expanded = !state.is_collapsed(&directory_id);
            tree = tree.item(directory.expanded(expanded));
        }
        if state.is_collapsed(&directory_id) {
            return tree;
        }
    }
    tree.item(file_node(item, item.id == selected_id))
}

fn directory_nodes(item: &FileTreeItem) -> Vec<TreeNode> {
    let parts = item.label.split('/').collect::<Vec<_>>();
    let mut nodes = Vec::new();
    for depth in 0..parts.len().saturating_sub(1) {
        let id = parts[..=depth].join("/");
        nodes.push(TreeNode::new(id, parts[depth], depth).directory());
    }
    nodes
}

fn file_node(item: &FileTreeItem, selected: bool) -> TreeNode {
    let depth = item.label.matches('/').count();
    let name = item.label.rsplit('/').next().unwrap_or(item.label.as_str());
    TreeNode::new(item.id.clone(), name, depth)
        .file()
        .icon(item.icon.clone())
        .selected(selected)
        .active(selected)
}
