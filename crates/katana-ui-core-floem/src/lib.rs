//! Floem adapter and legacy Floem widget surface for KUC.

pub mod menu_button_contract;
pub mod overlay_lifecycle;
pub mod runtime;
pub mod view;

use katana_ui_core::adapter_contract::WidgetAdapter;
use katana_ui_core::render_model::{RenderContext, UiNode, UiNodeKind, UiTree};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FloemAdapter;

impl WidgetAdapter for FloemAdapter {
    type Output = FloemRenderPlan;

    fn render_tree(&self, tree: &UiTree, context: &RenderContext) -> Self::Output {
        FloemRenderPlan {
            theme_id: context.theme_id.as_str().to_string(),
            root: FloemNodePlan::from_node(tree.root()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloemRenderPlan {
    pub theme_id: String,
    pub root: FloemNodePlan,
}

impl FloemRenderPlan {
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.root.node_count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloemNodePlan {
    pub kind: UiNodeKind,
    pub label: String,
    pub child_count: usize,
    pub children: Vec<FloemNodePlan>,
}

impl FloemNodePlan {
    #[must_use]
    pub fn from_node(node: &UiNode) -> Self {
        let children: Vec<Self> = node.children().iter().map(Self::from_node).collect();
        Self {
            kind: node.kind(),
            label: node.props().label.clone(),
            child_count: children.len(),
            children,
        }
    }

    #[must_use]
    pub fn node_count(&self) -> usize {
        1 + self.children.iter().map(Self::node_count).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::{FloemAdapter, UiNodeKind, WidgetAdapter};
    use katana_ui_core::atom::Text;
    use katana_ui_core::render_model::{RenderContext, UiTree};
    use katana_ui_core::theme::ThemeId;

    #[test]
    fn maps_text_tree_to_floem_plan() {
        let adapter = FloemAdapter;
        let output = adapter.render_tree(
            &UiTree::new(Text::new("hello")),
            &RenderContext::new(ThemeId::new("light"), 100.0, 100.0),
        );

        assert_eq!(UiNodeKind::Text, output.root.kind);
    }
}
