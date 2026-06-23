use super::preset_labels::StoryPresetLabels;
use crate::requirements::StoryRequirements;
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::render_model::{UiNode, UiTree};
use std::collections::BTreeSet;

const GENERIC_PRESET_LABELS: &[&str] = &["default", "interactive", "edge", "theme"];
const PASSIVE_CONTRACT_PAGES: &[&str] = &[
    "theme-tokens",
    "panel",
    "text",
    "icon",
    "badge",
    "divider",
    "spacer",
    "key-cap",
    "row",
    "column",
    "stack",
    "grid",
    "scroll-area",
    "align-center",
    "list",
    "menu",
    "tabs",
    "toolbar",
    "form-field",
    "breadcrumb",
    "side-menu",
    "status-bar",
    "command-palette",
    "dynamic-array-editor",
    "drag-and-drop",
    "closeable-tab-strip",
    "hover-card",
    "empty-state",
];

#[derive(Debug, Clone, PartialEq)]
pub struct StoryExample {
    pub page: &'static str,
    pub tree: UiTree,
    pub minimum_nodes: usize,
    pub callback_logs: Vec<UiCallbackLog>,
    pub contract: StoryPageContract,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoryPageContract {
    pub preview: bool,
    pub settings: bool,
    pub state_summary: bool,
    pub event_history: bool,
    pub action_history: bool,
    pub preset_tabs: bool,
    pub requirement_status: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryCatalogReport {
    pub stories: usize,
    pub validated: usize,
    pub state_conflicts: usize,
    pub structure_failures: usize,
    pub missing_required_pages: usize,
    pub page_contract_failures: usize,
    pub nodes: usize,
}

impl StoryCatalogReport {
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "stories={} validated={} state_conflicts={} structure_failures={} missing_required_pages={} page_contract_failures={} nodes={}",
            self.stories,
            self.validated,
            self.state_conflicts,
            self.structure_failures,
            self.missing_required_pages,
            self.page_contract_failures,
            self.nodes
        )
    }
}

impl StoryPageContract {
    #[must_use]
    pub fn from_tree(
        page: &str,
        tree: &UiTree,
        minimum_nodes: usize,
        callback_logs: &[UiCallbackLog],
    ) -> Self {
        let root = tree.root();
        Self {
            preview: node_count(root) >= minimum_nodes,
            settings: has_settings_contract(page, root),
            state_summary: !root.props().state_id.as_str().is_empty(),
            event_history: has_event_contract(page, callback_logs),
            action_history: has_action_contract(page, callback_logs),
            preset_tabs: has_preset_contract(page),
            requirement_status: StoryRequirements::required_pages().contains(&page),
        }
    }

    #[must_use]
    pub fn is_complete(self) -> bool {
        self.preview
            && self.settings
            && self.state_summary
            && self.event_history
            && self.action_history
            && self.preset_tabs
            && self.requirement_status
    }
}

fn node_count(node: &UiNode) -> usize {
    1 + node.children().iter().map(node_count).sum::<usize>()
}

fn has_settings_contract(page: &str, root: &UiNode) -> bool {
    !page.is_empty()
        && !root.props().state_id.as_str().is_empty()
        && !StoryPresetLabels::for_page(page)
            .iter()
            .any(|it| it.is_empty())
}

fn has_event_contract(page: &str, callback_logs: &[UiCallbackLog]) -> bool {
    callback_logs.iter().any(has_target_state_log) || is_passive_contract_page(page)
}

fn has_action_contract(page: &str, callback_logs: &[UiCallbackLog]) -> bool {
    callback_logs.iter().any(|it| !it.action.is_empty()) || is_passive_contract_page(page)
}

fn has_target_state_log(log: &UiCallbackLog) -> bool {
    !log.target.as_str().is_empty() && !log.before.is_empty() && !log.after.is_empty()
}

fn has_preset_contract(page: &str) -> bool {
    let labels = StoryPresetLabels::for_page(page);
    let unique: BTreeSet<&str> = labels.iter().copied().collect();
    labels.len() == unique.len() && labels.iter().all(|it| !GENERIC_PRESET_LABELS.contains(it))
}

fn is_passive_contract_page(page: &str) -> bool {
    PASSIVE_CONTRACT_PAGES.contains(&page)
}
