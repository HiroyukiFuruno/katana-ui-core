use super::{TreeNodeKind, TreeView};
use crate::molecule::DisclosureTriggerArea;

#[path = "tree_view_hit_test_types.rs"]
mod hit_test_types;

pub use hit_test_types::{
    TreeViewAction, TreeViewHitRect, TreeViewHitTarget, TreeViewHitTestInput,
};

const ROW_HEIGHT: u32 = 22;
const INDENT_WIDTH: u32 = 12;
const DISCLOSURE_WIDTH: u32 = 16;
const DISCLOSURE_GAP: u32 = 4;
const ICON_WIDTH: u32 = 16;
const ICON_GAP: u32 = 6;

impl TreeView {
    #[must_use]
    pub const fn row_height() -> u32 {
        ROW_HEIGHT
    }

    #[must_use]
    pub const fn indent_width() -> u32 {
        INDENT_WIDTH
    }

    #[must_use]
    pub const fn disclosure_width() -> u32 {
        DISCLOSURE_WIDTH
    }

    #[must_use]
    pub const fn disclosure_gap() -> u32 {
        DISCLOSURE_GAP
    }

    #[must_use]
    pub const fn icon_width() -> u32 {
        ICON_WIDTH
    }

    #[must_use]
    pub const fn icon_gap() -> u32 {
        ICON_GAP
    }

    #[must_use]
    pub fn hit_test(&self, input: TreeViewHitTestInput) -> TreeViewAction {
        self.hit_target(input, u32::MAX)
            .map(|target| target.action)
            .unwrap_or(TreeViewAction::None)
    }

    #[must_use]
    pub fn hit_target(
        &self,
        input: TreeViewHitTestInput,
        viewport_width: u32,
    ) -> Option<TreeViewHitTarget> {
        let absolute_y = input.pointer_y.saturating_add(input.scroll_offset_y);
        let row_index = self.node_row_index(absolute_y)?;
        let node = self.items.get(row_index)?;
        let action = match node.kind {
            TreeNodeKind::Directory => self.directory_action(&node.id, node.depth, input.pointer_x),
            TreeNodeKind::File => TreeViewAction::SelectNode {
                node_id: node.id.clone(),
            },
        };
        Some(TreeViewHitTarget {
            node_id: node.id.clone(),
            rect: self.row_rect(row_index, input.scroll_offset_y, viewport_width),
            action,
        })
    }

    #[must_use]
    pub fn row_count(&self) -> usize {
        self.items.len() + usize::from(self.has_label_row())
    }

    fn node_row_index(&self, absolute_y: u32) -> Option<usize> {
        let row_index = absolute_y / ROW_HEIGHT;
        if self.has_label_row() {
            if row_index == 0 {
                return None;
            }
            return Some(row_index.saturating_sub(1) as usize);
        }
        Some(row_index as usize)
    }

    fn row_rect(
        &self,
        node_row_index: usize,
        scroll_offset_y: u32,
        viewport_width: u32,
    ) -> TreeViewHitRect {
        let row_index = node_row_index.saturating_add(usize::from(self.has_label_row()));
        let row_top = (row_index as u32).saturating_mul(ROW_HEIGHT);
        TreeViewHitRect {
            x: 0,
            y: row_top.saturating_sub(scroll_offset_y),
            width: viewport_width,
            height: ROW_HEIGHT,
        }
    }

    fn has_label_row(&self) -> bool {
        !self.label.trim().is_empty()
    }

    fn directory_action(&self, node_id: &str, depth: usize, pointer_x: u32) -> TreeViewAction {
        if self.accepts_directory_toggle(depth, pointer_x) {
            return TreeViewAction::ToggleNode {
                node_id: node_id.to_string(),
            };
        }
        TreeViewAction::FocusNode {
            node_id: node_id.to_string(),
        }
    }

    fn accepts_directory_toggle(&self, depth: usize, pointer_x: u32) -> bool {
        match self.model.toggle_trigger_area {
            DisclosureTriggerArea::WholeElement => true,
            DisclosureTriggerArea::IconOnly => {
                self.model.icons_visible && in_range(pointer_x, content_x(depth), DISCLOSURE_WIDTH)
            }
            DisclosureTriggerArea::IconAndText => pointer_x >= content_x(depth),
            DisclosureTriggerArea::TextOnly => pointer_x >= self.label_x(depth),
        }
    }

    fn label_x(&self, depth: usize) -> u32 {
        let start = content_x(depth);
        if !self.model.icons_visible {
            return start;
        }
        start
            .saturating_add(DISCLOSURE_WIDTH)
            .saturating_add(DISCLOSURE_GAP)
            .saturating_add(ICON_WIDTH)
            .saturating_add(ICON_GAP)
    }
}

fn content_x(depth: usize) -> u32 {
    (depth as u32).saturating_mul(INDENT_WIDTH)
}

fn in_range(value: u32, start: u32, width: u32) -> bool {
    value >= start && value < start.saturating_add(width)
}

#[cfg(test)]
mod tests {
    use super::{TreeViewAction, TreeViewHitTestInput};
    use crate::molecule::{TreeNode, TreeView};

    #[test]
    fn hit_test_ignores_label_row() {
        let tree = sample_tree();

        let action = tree.hit_test(TreeViewHitTestInput {
            pointer_x: 0,
            pointer_y: 1,
            scroll_offset_y: 0,
        });

        assert_eq!(TreeViewAction::None, action);
    }

    #[test]
    fn hit_test_returns_directory_toggle_for_directory_node() {
        let tree = sample_tree();

        let action = tree.hit_test(TreeViewHitTestInput {
            pointer_x: 0,
            pointer_y: 25,
            scroll_offset_y: 0,
        });

        assert_eq!(
            TreeViewAction::ToggleNode {
                node_id: "src".to_string()
            },
            action
        );
    }

    #[test]
    fn hit_test_returns_file_selection_for_file_node() {
        let tree = sample_tree();

        let action = tree.hit_test(TreeViewHitTestInput {
            pointer_x: 0,
            pointer_y: 49,
            scroll_offset_y: 0,
        });

        assert_eq!(
            TreeViewAction::SelectNode {
                node_id: "src/lib.rs".to_string()
            },
            action
        );
    }

    #[test]
    fn hit_target_returns_action_and_rendered_row_rect() -> Result<(), String> {
        let tree = sample_tree();

        let target = tree
            .hit_target(
                TreeViewHitTestInput {
                    pointer_x: 0,
                    pointer_y: 49,
                    scroll_offset_y: 0,
                },
                240,
            )
            .ok_or_else(|| "file row target".to_string())?;

        assert_eq!("src/lib.rs", target.node_id);
        assert_eq!(
            TreeViewAction::SelectNode {
                node_id: "src/lib.rs".to_string()
            },
            target.action
        );
        assert_eq!(0, target.rect.x);
        assert_eq!(44, target.rect.y);
        assert_eq!(240, target.rect.width);
        assert_eq!(TreeView::row_height(), target.rect.height);
        Ok(())
    }

    #[test]
    fn hit_target_accounts_for_scroll_offset() -> Result<(), String> {
        let tree = sample_tree();

        let target = tree
            .hit_target(
                TreeViewHitTestInput {
                    pointer_x: 0,
                    pointer_y: 5,
                    scroll_offset_y: 39,
                },
                240,
            )
            .ok_or_else(|| "scrolled file row target".to_string())?;

        assert_eq!("src/lib.rs", target.node_id);
        assert_eq!(5, target.rect.y);
        Ok(())
    }

    #[test]
    fn row_height_matches_katana_explorer_contract() {
        assert_eq!(22, TreeView::row_height());
    }

    #[test]
    fn icon_only_trigger_area_does_not_toggle_from_label_text() {
        let tree = sample_tree()
            .icons_visible(true)
            .toggle_trigger_area(crate::molecule::DisclosureTriggerArea::IconOnly);

        let action = tree.hit_test(TreeViewHitTestInput {
            pointer_x: 64,
            pointer_y: 25,
            scroll_offset_y: 0,
        });

        assert_eq!(
            TreeViewAction::FocusNode {
                node_id: "src".to_string()
            },
            action
        );
    }

    #[test]
    fn icon_only_trigger_area_toggles_from_disclosure_icon() {
        let tree = sample_tree()
            .icons_visible(true)
            .toggle_trigger_area(crate::molecule::DisclosureTriggerArea::IconOnly);

        let action = tree.hit_test(TreeViewHitTestInput {
            pointer_x: 4,
            pointer_y: 25,
            scroll_offset_y: 0,
        });

        assert_eq!(
            TreeViewAction::ToggleNode {
                node_id: "src".to_string()
            },
            action
        );
    }

    fn sample_tree() -> TreeView {
        TreeView::new("Files")
            .item(TreeNode::new("src", "src", 0).directory())
            .item(TreeNode::new("src/lib.rs", "lib.rs", 1).file())
    }
}
