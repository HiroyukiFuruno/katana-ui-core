mod atom_interactions;
mod atom_motion_interactions;
mod atoms;
#[cfg(test)]
mod color_picker_story_tests;
mod layouts;
mod molecules;
mod panel_interaction;
mod panel_operations;
mod panel_report;
mod preset_labels;
#[cfg(test)]
mod search_control_strip_story_tests;
#[cfg(test)]
mod tests;
mod types;
use crate::requirements::StoryRequirements;
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::render_model::{UiNode, UiStateId, UiTree};
pub use panel_interaction::StoryDetailContent;
pub use panel_interaction::StorybookPanelInteractionReport;
pub(crate) use panel_operations::StorybookOperationSequences;
pub(crate) use panel_report::StorybookPanelReportFields;
pub use panel_report::{StorybookPanelReport, StorybookStyleSheet};
pub(crate) use preset_labels::StoryPresetLabels;
use std::collections::BTreeSet;
pub use types::{StoryCatalogReport, StoryExample, StoryPageContract};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StoryCatalog;

impl StoryCatalog {
    #[must_use]
    pub fn examples(self) -> Vec<StoryExample> {
        let mut examples = Vec::new();
        examples.extend(atoms::examples());
        examples.extend(molecules::examples());
        examples.extend(layouts::examples());
        examples
    }

    #[must_use]
    pub fn verify(self) -> StoryCatalogReport {
        let examples = self.examples();
        self.verify_examples(&examples)
    }

    #[must_use]
    pub fn verify_examples(self, examples: &[StoryExample]) -> StoryCatalogReport {
        let state_conflicts = examples
            .iter()
            .filter(|it| Self::has_state_conflict(&it.tree))
            .count();
        let structure_failures = examples
            .iter()
            .filter(|it| Self::node_count(it.tree.root()) < it.minimum_nodes)
            .count();
        let present: BTreeSet<&str> = examples.iter().map(|it| it.page).collect();
        let missing_required_pages = StoryRequirements::required_pages()
            .iter()
            .filter(|it| !present.contains(**it))
            .count();
        let page_contract_failures = examples
            .iter()
            .filter(|it| !it.contract.is_complete())
            .count();
        StoryCatalogReport {
            stories: examples.len(),
            validated: examples.len()
                - state_conflicts
                - structure_failures
                - page_contract_failures,
            state_conflicts,
            structure_failures,
            missing_required_pages,
            page_contract_failures,
            nodes: examples
                .iter()
                .map(|it| Self::node_count(it.tree.root()))
                .sum(),
        }
    }

    pub(super) fn story(page: &'static str, root: impl Into<UiNode>) -> StoryExample {
        let tree = UiTree::new(root);
        let minimum_nodes = StoryRequirements::minimum_nodes_for(page);
        let callback_logs = Vec::new();
        let contract =
            StoryPageContract::from_tree(page, &tree, minimum_nodes, callback_logs.as_slice());
        StoryExample {
            page,
            tree,
            minimum_nodes,
            callback_logs,
            contract,
        }
    }

    pub(super) fn interactive_story(
        page: &'static str,
        root: impl Into<UiNode>,
        callback_logs: Vec<UiCallbackLog>,
    ) -> StoryExample {
        let tree = UiTree::new(root);
        let minimum_nodes = StoryRequirements::minimum_nodes_for(page);
        let contract =
            StoryPageContract::from_tree(page, &tree, minimum_nodes, callback_logs.as_slice());
        StoryExample {
            page,
            tree,
            minimum_nodes,
            callback_logs,
            contract,
        }
    }

    fn has_state_conflict(tree: &UiTree) -> bool {
        let mut ids = Vec::new();
        Self::collect_state_ids(tree.root(), &mut ids);
        let unique: BTreeSet<&str> = ids.iter().map(UiStateId::as_str).collect();
        unique.len() != ids.len()
    }

    fn collect_state_ids(node: &UiNode, ids: &mut Vec<UiStateId>) {
        ids.push(node.props().state_id.clone());
        for child in node.children() {
            Self::collect_state_ids(child, ids);
        }
    }

    fn node_count(node: &UiNode) -> usize {
        1 + node.children().iter().map(Self::node_count).sum::<usize>()
    }
}
