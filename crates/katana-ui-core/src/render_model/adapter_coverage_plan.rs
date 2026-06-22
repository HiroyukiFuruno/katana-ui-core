use super::{UiNode, UiNodeKind, UiTree};
use serde::{Deserialize, Serialize};

pub const REQUIRED_CONSUMER_NODE_KINDS: &[UiNodeKind] = &[
    UiNodeKind::Text,
    UiNodeKind::Icon,
    UiNodeKind::ImageSurface,
    UiNodeKind::Button,
    UiNodeKind::SvgButton,
    UiNodeKind::TextButton,
    UiNodeKind::IconTextButton,
    UiNodeKind::Input,
    UiNodeKind::TextArea,
    UiNodeKind::SearchBox,
    UiNodeKind::SelectBox,
    UiNodeKind::ComboBox,
    UiNodeKind::MenuButton,
    UiNodeKind::SelectionList,
    UiNodeKind::CloseableTabStrip,
    UiNodeKind::CloseableTab,
    UiNodeKind::Toolbar,
    UiNodeKind::Accordion,
    UiNodeKind::ContextMenu,
    UiNodeKind::ModalOverlay,
    UiNodeKind::Panel,
    UiNodeKind::Row,
    UiNodeKind::Column,
    UiNodeKind::ScrollArea,
    UiNodeKind::SplitPane,
];

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiAdapterCoveragePlan {
    pub input_count: usize,
    pub text_area_count: usize,
    pub tab_container_count: usize,
    pub selection_count: usize,
    pub split_pane_count: usize,
    pub scroll_area_count: usize,
    pub modal_count: usize,
    pub required_consumer_node_kind_count: usize,
    pub missing_required_consumer_node_kinds: Vec<UiNodeKind>,
    pub unsupported_node_count: usize,
}

impl UiAdapterCoveragePlan {
    #[must_use]
    pub const fn required_consumer_node_kinds() -> &'static [UiNodeKind] {
        REQUIRED_CONSUMER_NODE_KINDS
    }

    #[must_use]
    pub fn collect_from_tree(tree: &UiTree) -> Self {
        let mut plan = Self::default();
        let mut present_kinds = Vec::new();
        plan.collect_node(tree.root(), &mut present_kinds);
        plan.required_consumer_node_kind_count = REQUIRED_CONSUMER_NODE_KINDS.len();
        plan.missing_required_consumer_node_kinds = REQUIRED_CONSUMER_NODE_KINDS
            .iter()
            .copied()
            .filter(|required| !present_kinds.contains(required))
            .collect();
        plan
    }

    #[must_use]
    pub const fn with_unsupported_count(mut self, value: usize) -> Self {
        self.unsupported_node_count = value;
        self
    }

    #[must_use]
    pub fn consumer_shell_ready(&self) -> bool {
        self.input_count > 0
            && self.text_area_count > 0
            && self.tab_container_count > 0
            && self.selection_count > 0
            && self.split_pane_count > 0
            && self.scroll_area_count > 0
            && self.modal_count > 0
            && self.missing_required_consumer_node_kinds.is_empty()
            && self.unsupported_node_count == 0
    }

    fn collect_node(&mut self, node: &UiNode, present_kinds: &mut Vec<UiNodeKind>) {
        remember_kind(present_kinds, node.kind());
        match node.kind() {
            UiNodeKind::Input | UiNodeKind::SearchBox => self.input_count += 1,
            UiNodeKind::TextArea => self.text_area_count += 1,
            UiNodeKind::Tabs | UiNodeKind::CloseableTabStrip => self.tab_container_count += 1,
            UiNodeKind::Checkbox
            | UiNodeKind::Radio
            | UiNodeKind::Toggle
            | UiNodeKind::SegmentedToggle
            | UiNodeKind::SelectBox
            | UiNodeKind::ComboBox
            | UiNodeKind::MenuButton
            | UiNodeKind::SelectionList => self.selection_count += 1,
            UiNodeKind::SplitPane => self.split_pane_count += 1,
            UiNodeKind::ScrollArea => self.scroll_area_count += 1,
            UiNodeKind::Modal | UiNodeKind::ModalOverlay => self.modal_count += 1,
            _ => {}
        }
        for child in node.children() {
            self.collect_node(child, present_kinds);
        }
    }
}

fn remember_kind(present_kinds: &mut Vec<UiNodeKind>, kind: UiNodeKind) {
    if !present_kinds.contains(&kind) {
        present_kinds.push(kind);
    }
}
