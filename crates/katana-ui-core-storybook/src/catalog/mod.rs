mod atoms;
mod layouts;
mod molecules;

use crate::requirements::StoryRequirements;
use katana_ui_core::render_model::{UiNode, UiStateId, UiTree};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryExample {
    pub page: &'static str,
    pub tree: UiTree,
    pub minimum_nodes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryCatalogReport {
    pub stories: usize,
    pub validated: usize,
    pub state_conflicts: usize,
    pub structure_failures: usize,
    pub missing_required_pages: usize,
    pub nodes: usize,
}

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
        StoryCatalogReport {
            stories: examples.len(),
            validated: examples.len() - state_conflicts - structure_failures,
            state_conflicts,
            structure_failures,
            missing_required_pages,
            nodes: examples
                .iter()
                .map(|it| Self::node_count(it.tree.root()))
                .sum(),
        }
    }

    pub(super) fn story(page: &'static str, root: impl Into<UiNode>) -> StoryExample {
        StoryExample {
            page,
            tree: UiTree::new(root),
            minimum_nodes: StoryRequirements::minimum_nodes_for(page),
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

impl StoryCatalogReport {
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "stories={} validated={} state_conflicts={} structure_failures={} missing_required_pages={} nodes={}",
            self.stories,
            self.validated,
            self.state_conflicts,
            self.structure_failures,
            self.missing_required_pages,
            self.nodes
        )
    }
}
