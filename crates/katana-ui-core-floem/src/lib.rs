//! Floem adapter and legacy Floem widget surface for KUC.

pub mod menu_button_contract;
pub mod overlay_lifecycle;
pub mod runtime;
pub mod view;

use katana_ui_core::adapter_contract::{NativeDragDropBridge, WidgetAdapter};
use katana_ui_core::event::UiEvent;
use katana_ui_core::interaction::drag_and_drop::{DragData, DropEffect};
use katana_ui_core::render_model::UiNodeId;
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FloemNativeDragDropBridge;

impl FloemNativeDragDropBridge {
    #[must_use]
    pub fn drag_start(self, source: UiNodeId, data: DragData) -> UiEvent {
        NativeDragDropBridge::drag_start(source, data)
    }

    #[must_use]
    pub fn drop(self, target: UiNodeId, data: DragData, effect: DropEffect) -> UiEvent {
        NativeDragDropBridge::drop(target, data, effect)
    }

    #[must_use]
    pub fn cancel(self, source: UiNodeId) -> Vec<UiEvent> {
        NativeDragDropBridge::cancel(source)
    }
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
    use super::{FloemAdapter, FloemNativeDragDropBridge, UiNodeKind, WidgetAdapter};
    use katana_ui_core::atom::Text;
    use katana_ui_core::event::{DragEvent, UiEvent};
    use katana_ui_core::interaction::drag_and_drop::{DragData, DropEffect, OS_FILE_LIST_TAG};
    use katana_ui_core::render_model::UiNodeId;
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

    #[test]
    fn native_dnd_stub_maps_start_drop_and_cancel() {
        let bridge = FloemNativeDragDropBridge;
        let data = DragData::new(OS_FILE_LIST_TAG, serde_json::json!(["/tmp/a.md"]));
        let start = bridge.drag_start(UiNodeId::new("source"), data.clone());
        let drop = bridge.drop(UiNodeId::new("target"), data, DropEffect::Copy);
        let cancel = bridge.cancel(UiNodeId::new("source"));

        assert!(matches!(start, UiEvent::Drag(DragEvent::DragStart { .. })));
        assert!(matches!(drop, UiEvent::Drag(DragEvent::Drop { .. })));
        assert!(matches!(
            cancel.as_slice(),
            [
                UiEvent::Drag(DragEvent::DragCancel { .. }),
                UiEvent::Drag(DragEvent::DragEnd {
                    committed: false,
                    ..
                })
            ]
        ));
    }
}
