use floem::views::{button, h_stack_from_iter, label, v_stack_from_iter};
use floem::{IntoView, View};
use katana_ui_core::adapter_contract::WidgetAdapter;
use katana_ui_core::render_model::{RenderContext, UiNode, UiNodeKind, UiTree};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FloemViewAdapter;

impl WidgetAdapter for FloemViewAdapter {
    type Output = Box<dyn View>;

    fn render_tree(&self, tree: &UiTree, _context: &RenderContext) -> Self::Output {
        Self::render_node(tree.root())
    }
}

impl FloemViewAdapter {
    #[must_use]
    pub fn render_node(node: &UiNode) -> Box<dyn View> {
        match node.kind() {
            UiNodeKind::Button
            | UiNodeKind::TextButton
            | UiNodeKind::SvgButton
            | UiNodeKind::IconTextButton
            | UiNodeKind::MenuButton => Self::button_node(node),
            UiNodeKind::Row | UiNodeKind::Toolbar | UiNodeKind::SplitPane => Self::row_node(node),
            _ => Self::column_node(node),
        }
    }

    fn label_node(node: &UiNode) -> Box<dyn View> {
        let text = Self::node_label(node);
        label(move || text.clone()).into_any()
    }

    fn button_node(node: &UiNode) -> Box<dyn View> {
        let text = Self::node_label(node);
        button(label(move || text.clone())).into_any()
    }

    fn row_node(node: &UiNode) -> Box<dyn View> {
        let children = Self::children_or_label(node);
        h_stack_from_iter(children).into_any()
    }

    fn column_node(node: &UiNode) -> Box<dyn View> {
        let children = Self::children_or_label(node);
        v_stack_from_iter(children).into_any()
    }

    fn children_or_label(node: &UiNode) -> Vec<Box<dyn View>> {
        if node.children().is_empty() {
            vec![Self::label_node(node)]
        } else {
            node.children().iter().map(Self::render_node).collect()
        }
    }

    fn node_label(node: &UiNode) -> String {
        format!("{:?}: {}", node.kind(), node.props().label)
    }
}

#[cfg(test)]
mod tests {
    use super::FloemViewAdapter;
    use katana_ui_core::adapter_contract::WidgetAdapter;
    use katana_ui_core::atom::{Button, Input, Text};
    use katana_ui_core::layout::{Column, Row, SplitPane};
    use katana_ui_core::molecule::{Tabs, Toolbar};
    use katana_ui_core::render_model::{RenderContext, UiTree};
    use katana_ui_core::theme::ThemeId;

    #[test]
    fn renders_primary_target_widgets_to_floem_view() {
        let tree = UiTree::new(
            Column::new()
                .child(Text::new("Title"))
                .child(Button::new("Save"))
                .child(Input::new("Name"))
                .child(Row::new().child(Text::new("Row")))
                .child(Tabs::new("Tabs").child(Text::new("Tab")))
                .child(Toolbar::new("Toolbar").child(Button::new("Run")))
                .child(SplitPane::new().child(Text::new("Left"))),
        );
        let context = RenderContext::new(ThemeId::new("light"), 800.0, 600.0);
        let _view = FloemViewAdapter.render_tree(&tree, &context);
    }
}
