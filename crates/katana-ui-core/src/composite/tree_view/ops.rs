use std::rc::Rc;

use super::types::TreeViewItem;

#[derive(Clone)]
pub(crate) struct FlattenedTreeItem {
    pub path: Vec<usize>,
    pub label: String,
    pub icon: Option<crate::primitive::icon::IconSource>,
    pub indent: usize,
    pub expanded: bool,
    pub active: bool,
    pub disabled: bool,
    pub has_children: bool,
    pub on_select: Rc<dyn Fn()>,
    pub on_context: Rc<dyn Fn()>,
    pub on_expand: Rc<dyn Fn()>,
    pub on_collapse: Rc<dyn Fn()>,
}

fn collect_active_path(items: &[TreeViewItem], cursor: &mut Vec<usize>) -> Option<Vec<usize>> {
    for (index, item) in items.iter().enumerate() {
        cursor.push(index);
        if item.active {
            return Some(cursor.clone());
        }

        if let Some(active_path) = collect_active_path(&item.children, cursor) {
            return Some(active_path);
        }

        cursor.pop();
    }

    None
}

fn collect_active_path_forces_open(items: &[TreeViewItem]) -> Option<Vec<usize>> {
    let mut cursor = Vec::new();
    collect_active_path(items, &mut cursor)
}

fn is_force_open_path(path: &[usize], active_path: &[usize]) -> bool {
    if path.len() > active_path.len() {
        return false;
    }
    active_path.starts_with(path)
}

fn clear_active(items: &mut [TreeViewItem]) {
    for item in items {
        item.active = false;
        clear_active(&mut item.children);
    }
}

fn set_expanded_recursive(items: &mut [TreeViewItem], expanded: bool) {
    for item in items {
        if !item.children.is_empty() && !item.disabled {
            item.expanded = expanded;
        }
        set_expanded_recursive(&mut item.children, expanded);
    }
}

fn item_by_path_mut<'a>(
    items: &'a mut [TreeViewItem],
    path: &[usize],
) -> Option<&'a mut TreeViewItem> {
    let (head, tail) = path.split_first()?;
    let child = items.get_mut(*head)?;
    if tail.is_empty() {
        return Some(child);
    }

    item_by_path_mut(&mut child.children, tail)
}

fn has_children(item: &TreeViewItem) -> bool {
    !item.children.is_empty()
}

fn effective_indent(item: &TreeViewItem, path: &[usize]) -> usize {
    if item.indent > 0 {
        item.indent
    } else {
        path.len().saturating_sub(1)
    }
}

fn should_open(
    item: &TreeViewItem,
    path: &[usize],
    active_path: &Option<Vec<usize>>,
    force_open: bool,
) -> bool {
    if item.expanded {
        return true;
    }

    if !force_open {
        return false;
    }

    active_path
        .as_ref()
        .is_some_and(|active| is_force_open_path(path, active))
}

fn flatten_recursive(
    items: &[TreeViewItem],
    active_path: &Option<Vec<usize>>,
    force_open: bool,
    path: &mut Vec<usize>,
    output: &mut Vec<FlattenedTreeItem>,
) {
    for (index, item) in items.iter().enumerate() {
        path.push(index);
        let has_children = has_children(item);
        output.push(FlattenedTreeItem {
            path: path.clone(),
            label: item.label.clone(),
            icon: item.icon.clone(),
            indent: effective_indent(item, path),
            expanded: item.expanded,
            active: item.active,
            disabled: item.disabled,
            has_children,
            on_select: item.on_select.clone(),
            on_context: item.on_context.clone(),
            on_expand: item.on_expand.clone(),
            on_collapse: item.on_collapse.clone(),
        });

        let open = has_children && should_open(item, path, active_path, force_open);
        if open {
            flatten_recursive(&item.children, active_path, force_open, path, output);
        }

        path.pop();
    }
}

/// Flatten tree items for visible rendering.
pub(crate) struct TreeViewOps;

impl TreeViewOps {
    /// Flatten tree items for visible rendering.
    pub(crate) fn flatten_visible_items(
        items: &[TreeViewItem],
        force_open: bool,
    ) -> Vec<FlattenedTreeItem> {
        let active_path = if force_open {
            collect_active_path_forces_open(items)
        } else {
            None
        };

        let mut output = Vec::new();
        let mut path = Vec::new();
        flatten_recursive(items, &active_path, force_open, &mut path, &mut output);
        output
    }

    /// Toggle expanded/collapsed state for a node by path.
    pub(crate) fn toggle_expand(items: &mut [TreeViewItem], path: &[usize]) -> Option<bool> {
        let item = item_by_path_mut(items, path)?;
        if item.disabled || item.children.is_empty() {
            return None;
        }

        item.expanded = !item.expanded;
        Some(item.expanded)
    }

    /// Select one item and clear active flags from other nodes.
    pub(crate) fn set_active(items: &mut [TreeViewItem], path: &[usize]) -> bool {
        if path.is_empty() {
            return false;
        };

        clear_active(items);

        let Some(selected) = item_by_path_mut(items, path) else {
            return false;
        };

        if selected.disabled {
            return false;
        }

        selected.active = true;
        true
    }

    /// Expand all expandable items.
    pub(crate) fn expand_all(items: &mut [TreeViewItem]) {
        set_expanded_recursive(items, true);
    }

    /// Collapse all expandable items.
    pub(crate) fn collapse_all(items: &mut [TreeViewItem]) {
        set_expanded_recursive(items, false);
    }
}
