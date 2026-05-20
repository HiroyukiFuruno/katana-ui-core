use super::command_launcher_results::CommandPaletteRenderer;
use super::items::{ArrayEditorItem, CommandItem, TreeNode, TreeNodeKind};
use super::types::{StructuredTypedModel, TreeLineStyle};
use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult, VirtualRange, VirtualizationConfig};
use crate::molecule::DisclosureTriggerArea;
use crate::molecule::state::MoleculeState;
use crate::molecule::virtualization;
use crate::render_model::{
    UiNode, UiNodeKind, UiTreeLineStyle, UiTreeNodeKind, UiTreeNodeProps, UiTreeProps,
    UiTreeToggleTriggerArea,
};
use serde::{Deserialize, Serialize};

macro_rules! structured_molecule {
    ($name:ident, $item:ty, $kind:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            pub(super) label: String,
            pub(super) state: MoleculeState,
            pub(super) items: Vec<$item>,
            pub(super) model: StructuredTypedModel,
            pub(super) children: Vec<UiNode>,
        }

        impl $name {
            #[must_use]
            pub fn new(label: impl Into<String>) -> Self {
                Self {
                    label: label.into(),
                    state: MoleculeState::new($kind),
                    items: Vec::new(),
                    model: StructuredTypedModel::default(),
                    children: Vec::new(),
                }
            }

            #[must_use]
            pub fn item(mut self, item: $item) -> Self {
                self.items.push(item);
                self.state.item_count = self.items.len();
                self
            }

            #[must_use]
            pub fn child(mut self, child: impl Into<UiNode>) -> Self {
                self.children.push(child.into());
                self
            }

            #[must_use]
            pub fn open(mut self, value: bool) -> Self {
                self.state.open = value;
                self
            }

            #[must_use]
            pub fn selected_index(mut self, value: usize) -> Self {
                self.state.has_selection = true;
                self.state.selected_index = value;
                self
            }

            #[must_use]
            pub fn item_count(mut self, value: usize) -> Self {
                self.state.item_count = value;
                self
            }

            #[must_use]
            pub fn value(mut self, value: impl Into<String>) -> Self {
                self.state.value = value.into();
                self
            }
        }
    };
}

structured_molecule!(TreeView, TreeNode, UiNodeKind::TreeView);
structured_molecule!(CommandPalette, CommandItem, UiNodeKind::CommandPalette);
structured_molecule!(
    DynamicArrayEditor,
    ArrayEditorItem,
    UiNodeKind::DynamicArrayEditor
);

impl From<TreeView> for UiNode {
    fn from(value: TreeView) -> Self {
        let model = value.model.clone();
        let range = value.virtual_range_model();
        let mut node = value
            .state
            .node(UiNodeKind::TreeView, value.label)
            .interaction(virtualization::interaction(
                value.state.interaction(),
                range.as_ref(),
            ))
            .tree(tree_props(model, value.items, range.as_ref()));
        if !value.model.font_role.is_empty() {
            node = node.font_role(value.model.font_role);
        }
        if !value.model.theme_id.is_empty() {
            node = node.theme_id(value.model.theme_id);
        }
        for child in value.children {
            node = node.child(child);
        }
        node
    }
}

impl From<CommandPalette> for UiNode {
    fn from(value: CommandPalette) -> Self {
        CommandPaletteRenderer::render(value)
    }
}

impl From<DynamicArrayEditor> for UiNode {
    fn from(value: DynamicArrayEditor) -> Self {
        structured_node(
            value
                .state
                .node(UiNodeKind::DynamicArrayEditor, value.label),
            value.children,
        )
    }
}

fn structured_node(mut node: UiNode, children: Vec<UiNode>) -> UiNode {
    for child in children {
        node = node.child(child);
    }
    node
}

fn tree_props(
    model: StructuredTypedModel,
    items: Vec<TreeNode>,
    range: Option<&VirtualRange>,
) -> UiTreeProps {
    UiTreeProps {
        active_id: model.active_id,
        line_display: model.line_display,
        line_style: tree_line_style(model.line_style),
        line_width: model.line_width,
        icons_visible: model.icons_visible,
        directory_icon: model.directory_icon,
        file_icon: model.file_icon,
        font_role: model.font_role,
        theme_id: model.theme_id,
        empty_area_context_menu: model.empty_area_context_menu,
        default_open: model.default_open,
        toggle_icon: model.toggle_icon,
        toggle_trigger_area: trigger_area(model.toggle_trigger_area),
        nodes: virtualization::slice_by_range(items, range)
            .into_iter()
            .map(tree_node_props)
            .collect(),
    }
}

fn tree_node_props(node: TreeNode) -> UiTreeNodeProps {
    UiTreeNodeProps::new(
        node.id,
        node.label,
        node.depth,
        match node.kind {
            TreeNodeKind::File => UiTreeNodeKind::File,
            TreeNodeKind::Directory => UiTreeNodeKind::Directory,
        },
    )
    .expanded(node.expanded)
    .selected(node.selected)
    .active(node.active)
}

fn tree_line_style(value: TreeLineStyle) -> UiTreeLineStyle {
    match value {
        TreeLineStyle::Solid => UiTreeLineStyle::Solid,
        TreeLineStyle::Dotted => UiTreeLineStyle::Dotted,
        TreeLineStyle::Dashed => UiTreeLineStyle::Dashed,
    }
}

fn trigger_area(value: DisclosureTriggerArea) -> UiTreeToggleTriggerArea {
    match value {
        DisclosureTriggerArea::IconOnly => UiTreeToggleTriggerArea::IconOnly,
        DisclosureTriggerArea::IconAndText => UiTreeToggleTriggerArea::IconAndText,
        DisclosureTriggerArea::WholeElement => UiTreeToggleTriggerArea::WholeElement,
        DisclosureTriggerArea::TextOnly => UiTreeToggleTriggerArea::TextOnly,
    }
}

impl ComponentAction for TreeView {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        if !matches!(action, UiAction::Press { .. }) {
            return self.state.apply_action(action, false);
        }
        let before = self.state.interaction();
        if action.target() != &self.state.state_id || self.state.disabled {
            return UiActionResult::ignored(self.state.state_id.clone(), before);
        }
        self.state.open = !self.state.open;
        UiActionResult::handled(
            self.state.state_id.clone(),
            action,
            before,
            self.state.interaction(),
        )
    }
}

impl TreeView {
    #[must_use]
    pub fn virtualization(mut self, value: VirtualizationConfig) -> Self {
        self.model.virtualization = Some(value);
        self
    }

    #[must_use]
    pub fn virtual_range_model(&self) -> Option<VirtualRange> {
        virtualization::range(&self.model.virtualization, self.items.len())
    }
}

impl CommandPalette {
    #[must_use]
    pub fn virtual_range_model(&self) -> Option<VirtualRange> {
        self.command_virtual_range_model()
    }
}

impl ComponentAction for CommandPalette {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        self.state.apply_action(action, false)
    }
}

impl ComponentAction for DynamicArrayEditor {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        self.state.apply_action(action, false)
    }
}
