mod atoms;
mod layouts;
mod molecules;
mod panel_interaction;
mod panel_operations;
mod panel_report;

use crate::requirements::StoryRequirements;
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::render_model::{UiNode, UiStateId, UiTree};
use std::collections::BTreeSet;

pub use panel_interaction::StorybookPanelInteractionReport;
pub(crate) use panel_operations::StorybookOperationSequences;
pub use panel_report::{StorybookPanelReport, StorybookStyleSheet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryExample {
    pub page: &'static str,
    pub tree: UiTree,
    pub minimum_nodes: usize,
    pub callback_logs: Vec<UiCallbackLog>,
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
            callback_logs: Vec::new(),
        }
    }

    pub(super) fn interactive_story(
        page: &'static str,
        root: impl Into<UiNode>,
        callback_logs: Vec<UiCallbackLog>,
    ) -> StoryExample {
        StoryExample {
            page,
            tree: UiTree::new(root),
            minimum_nodes: StoryRequirements::minimum_nodes_for(page),
            callback_logs,
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

#[cfg(test)]
mod tests {
    use super::StoryCatalog;
    use katana_ui_core::render_model::{UiNodeKind, UiVisualRole};

    #[test]
    fn atom_examples_use_typed_props_without_type_classes() {
        let examples = StoryCatalog.examples();
        let atoms = examples.iter().filter(|it| {
            matches!(
                it.tree.root().kind(),
                UiNodeKind::Text
                    | UiNodeKind::Icon
                    | UiNodeKind::Button
                    | UiNodeKind::Input
                    | UiNodeKind::Checkbox
                    | UiNodeKind::Radio
                    | UiNodeKind::Badge
                    | UiNodeKind::Divider
                    | UiNodeKind::Spacer
                    | UiNodeKind::KeyCap
                    | UiNodeKind::LoadingDots
                    | UiNodeKind::Spinner
                    | UiNodeKind::ProgressBar
                    | UiNodeKind::ColorSwatch
                    | UiNodeKind::Toggle
                    | UiNodeKind::SlideControl
                    | UiNodeKind::SvgButton
                    | UiNodeKind::TextButton
                    | UiNodeKind::IconTextButton
            )
        });

        for example in atoms {
            let props = example.tree.root().props();
            assert!(props.style_classes.is_empty(), "{}", example.page);
        }
        let key_cap = examples.iter().find(|it| it.page == "key-cap");
        assert!(key_cap.is_some(), "key-cap story is required");
        let key_cap_props = key_cap.map(|it| it.tree.root().props());
        assert_eq!(
            Some(UiVisualRole::Shortcut),
            key_cap_props.map(|it| it.visual_role)
        );
        assert_eq!(Some("code"), key_cap_props.map(|it| it.font_role.as_str()));
    }

    #[test]
    fn interactive_atom_examples_expose_callback_logs() {
        let examples = StoryCatalog.examples();
        let log_pages: Vec<&str> = examples
            .iter()
            .filter(|it| !it.callback_logs.is_empty())
            .map(|it| it.page)
            .collect();

        assert!(log_pages.contains(&"button"));
        assert!(log_pages.contains(&"text-input"));
        assert!(log_pages.contains(&"checkbox"));
        assert!(log_pages.contains(&"toggle"));
    }
}
