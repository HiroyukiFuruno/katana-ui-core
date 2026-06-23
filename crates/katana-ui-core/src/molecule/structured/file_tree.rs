use super::{
    TreeNode, TreeNodeKind, TreeView, TreeViewAction, TreeViewHitRect, TreeViewHitTestInput,
};
use crate::interaction::UiAction;
use crate::layout::{ScrollArea, ScrollAxis};
use crate::render_model::{UiCursor, UiHostActionPlan, UiTree, UiTreeRowActionKind};

#[path = "file_tree_model.rs"]
mod file_tree_model;
#[path = "file_tree_types.rs"]
mod file_tree_types;

pub use file_tree_types::{
    FileTree, FileTreeAction, FileTreeHitRect, FileTreeHitTarget, FileTreeHitTestInput,
    FileTreeItem, FileTreeState,
};

const MIN_VIEWPORT_EXTENT: u32 = 1;

impl FileTree {
    #[must_use]
    pub fn render(items: &[FileTreeItem], selected_id: &str, width: u32, height: u32) -> UiTree {
        Self::render_with_offset(items, selected_id, width, height, 0)
    }

    #[must_use]
    pub fn render_with_offset(
        items: &[FileTreeItem],
        selected_id: &str,
        width: u32,
        height: u32,
        offset_y: u32,
    ) -> UiTree {
        Self::render_with_state_and_offset(
            items,
            selected_id,
            width,
            height,
            offset_y,
            &FileTreeState::default(),
        )
    }

    #[must_use]
    pub fn render_with_state_and_offset(
        items: &[FileTreeItem],
        selected_id: &str,
        width: u32,
        height: u32,
        offset_y: u32,
        state: &FileTreeState,
    ) -> UiTree {
        UiTree::new(
            ScrollArea::new()
                .axis(ScrollAxis::Vertical)
                .viewport(viewport_extent(width), viewport_extent(height))
                .content_extent(viewport_extent(width), content_height(items, height, state))
                .offset(0, offset_y)
                .child(file_tree_model::tree_view(items, selected_id, state)),
        )
    }

    #[must_use]
    pub fn selected_item_id(actions: &[UiAction]) -> Option<&str> {
        actions.iter().find_map(selected_value)
    }

    #[must_use]
    pub fn action_from_host_plan(plan: &UiHostActionPlan) -> Option<FileTreeAction> {
        let target = plan.tree_row_action_target()?;
        Some(match target.action_kind {
            UiTreeRowActionKind::Select => FileTreeAction::SelectFile {
                file_id: target.node_id,
            },
            UiTreeRowActionKind::Toggle => FileTreeAction::ToggleDirectory {
                directory_id: target.node_id,
            },
            UiTreeRowActionKind::Focus => FileTreeAction::FocusItem {
                item_id: target.node_id,
            },
        })
    }

    #[must_use]
    pub fn content_height_with_state(
        items: &[FileTreeItem],
        viewport_height: u32,
        state: &FileTreeState,
    ) -> u32 {
        content_height(items, viewport_height, state)
    }

    #[must_use]
    pub fn hit_test(items: &[FileTreeItem], input: FileTreeHitTestInput) -> FileTreeAction {
        Self::hit_test_with_state(items, &FileTreeState::default(), input)
    }

    #[must_use]
    pub fn hit_test_with_state(
        items: &[FileTreeItem],
        state: &FileTreeState,
        input: FileTreeHitTestInput,
    ) -> FileTreeAction {
        let tree = file_tree_model::tree_view(items, "", state);
        file_tree_action_from_tree_action(tree.hit_test(TreeViewHitTestInput {
            pointer_x: input.pointer_x,
            pointer_y: input.pointer_y,
            scroll_offset_y: input.scroll_offset_y,
        }))
    }

    #[must_use]
    pub fn hit_target_with_state(
        items: &[FileTreeItem],
        state: &FileTreeState,
        input: FileTreeHitTestInput,
        viewport_width: u32,
    ) -> Option<FileTreeHitTarget> {
        let tree = file_tree_model::tree_view(items, "", state);
        let target = tree.hit_target(
            TreeViewHitTestInput {
                pointer_x: input.pointer_x,
                pointer_y: input.pointer_y,
                scroll_offset_y: input.scroll_offset_y,
            },
            viewport_width,
        )?;
        let action = file_tree_action_from_tree_action(target.action);
        let item_id = item_id_for_action(&action)?.to_string();
        Some(FileTreeHitTarget {
            item_id,
            rect: file_tree_rect_from_tree_rect(target.rect),
            cursor: tree_row_cursor(items, state),
            action,
        })
    }

    #[must_use]
    pub fn hit_target_for_item_with_state(
        items: &[FileTreeItem],
        state: &FileTreeState,
        item_id: &str,
        scroll_offset_y: u32,
        viewport_width: u32,
    ) -> Option<FileTreeHitTarget> {
        let tree = file_tree_model::tree_view(items, "", state);
        let (row_index, node) = tree
            .items
            .iter()
            .enumerate()
            .find(|(_, node)| node.id == item_id)?;
        Some(hit_target_for_node(
            node,
            row_index.saturating_add(1),
            scroll_offset_y,
            viewport_width,
            tree_row_cursor(items, state),
        ))
    }

    #[must_use]
    pub fn cursor_for_hit_with_state(
        items: &[FileTreeItem],
        state: &FileTreeState,
        input: FileTreeHitTestInput,
    ) -> UiCursor {
        Self::hit_target_with_state(items, state, input, MIN_VIEWPORT_EXTENT)
            .map(|target| target.cursor)
            .unwrap_or(UiCursor::Default)
    }
}

fn hit_target_for_node(
    node: &TreeNode,
    row_index: usize,
    scroll_offset_y: u32,
    viewport_width: u32,
    cursor: UiCursor,
) -> FileTreeHitTarget {
    FileTreeHitTarget {
        item_id: node.id.clone(),
        rect: row_hit_rect(row_index, scroll_offset_y, viewport_width),
        cursor,
        action: action_for_node(node),
    }
}

fn file_tree_action_from_tree_action(action: TreeViewAction) -> FileTreeAction {
    match action {
        TreeViewAction::SelectNode { node_id } => FileTreeAction::SelectFile { file_id: node_id },
        TreeViewAction::ToggleNode { node_id } => FileTreeAction::ToggleDirectory {
            directory_id: node_id,
        },
        TreeViewAction::FocusNode { node_id } => FileTreeAction::FocusItem { item_id: node_id },
        TreeViewAction::HoverNode { .. } => FileTreeAction::None,
        TreeViewAction::None => FileTreeAction::None,
    }
}

fn action_for_node(node: &TreeNode) -> FileTreeAction {
    match node.kind {
        TreeNodeKind::Directory => FileTreeAction::ToggleDirectory {
            directory_id: node.id.clone(),
        },
        TreeNodeKind::File => FileTreeAction::SelectFile {
            file_id: node.id.clone(),
        },
    }
}

fn file_tree_rect_from_tree_rect(rect: TreeViewHitRect) -> FileTreeHitRect {
    FileTreeHitRect {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
    }
}

fn selected_value(action: &UiAction) -> Option<&str> {
    let UiAction::SetValue { value, .. } = action else {
        return None;
    };
    Some(value.as_str())
}

fn item_id_for_action(action: &FileTreeAction) -> Option<&str> {
    match action {
        FileTreeAction::SelectFile { file_id } => Some(file_id),
        FileTreeAction::ToggleDirectory { directory_id } => Some(directory_id),
        FileTreeAction::FocusItem { item_id } => Some(item_id),
        FileTreeAction::None => None,
    }
}

fn row_hit_rect(row_index: usize, scroll_offset_y: u32, viewport_width: u32) -> FileTreeHitRect {
    let row_top = (row_index as u32).saturating_mul(TreeView::row_height());
    visible_row_hit_rect(row_top, scroll_offset_y, viewport_width)
}

fn visible_row_hit_rect(
    row_top: u32,
    scroll_offset_y: u32,
    viewport_width: u32,
) -> FileTreeHitRect {
    FileTreeHitRect {
        x: 0,
        y: row_top.saturating_sub(scroll_offset_y),
        width: viewport_extent(viewport_width),
        height: TreeView::row_height(),
    }
}

fn tree_row_cursor(items: &[FileTreeItem], state: &FileTreeState) -> UiCursor {
    let tree = UiTree::new(file_tree_model::tree_view(items, "", state));
    tree.root().props().tree.row_cursor
}

fn content_height(items: &[FileTreeItem], viewport_height: u32, state: &FileTreeState) -> u32 {
    file_tree_model::tree_view(items, "", state)
        .row_count()
        .try_into()
        .unwrap_or(u32::MAX)
        .saturating_mul(TreeView::row_height())
        .max(viewport_extent(viewport_height))
}

fn viewport_extent(value: u32) -> u32 {
    value.max(MIN_VIEWPORT_EXTENT)
}
