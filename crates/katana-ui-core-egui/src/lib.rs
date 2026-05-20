pub mod ime_contract;

use katana_ui_core::adapter_contract::{NativeDragDropBridge, WidgetAdapter};
use katana_ui_core::event::UiEvent;
use katana_ui_core::interaction::drag_and_drop::{DragData, DropEffect};
use katana_ui_core::render_model::UiNodeId;
use katana_ui_core::render_model::{RenderContext, UiNodeKind, UiTree};
use katana_ui_core::runtime::{AppConfig, AppHandle, AppLifecycle, RuntimeAdapter};
use katana_ui_core::surface::PaintRequest;
use katana_ui_core::window::{WindowCommand, WindowConfig};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EguiCompatAdapter;

impl WidgetAdapter for EguiCompatAdapter {
    type Output = EguiRenderPlan;

    fn render_tree(&self, tree: &UiTree, _context: &RenderContext) -> Self::Output {
        EguiRenderPlan {
            root_kind: tree.root().kind(),
            child_count: tree.root().children().len(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EguiRenderPlan {
    pub root_kind: UiNodeKind,
    pub child_count: usize,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EguiRuntimeAdapter {
    lifecycle: Vec<AppLifecycle>,
}

impl RuntimeAdapter for EguiRuntimeAdapter {
    fn run(&mut self, config: AppConfig, windows: Vec<WindowConfig>) -> AppHandle {
        self.lifecycle.push(AppLifecycle::Started);
        AppHandle::new(
            config.app_id,
            windows.into_iter().map(WindowConfig::into_id).collect(),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EguiWindowAction {
    Command(String),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EguiWindowBridge;

impl EguiWindowBridge {
    #[must_use]
    pub fn map_command(self, command: &WindowCommand) -> EguiWindowAction {
        EguiWindowAction::Command(format!("{command:?}"))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EguiSurfaceBridge;

impl EguiSurfaceBridge {
    #[must_use]
    pub fn paint(self, request: &PaintRequest) -> EguiRenderPlan {
        EguiRenderPlan {
            root_kind: request.tree().root().kind(),
            child_count: request.tree().root().children().len(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EguiNativeDragDropBridge;

impl EguiNativeDragDropBridge {
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

#[cfg(test)]
mod tests {
    use super::{
        EguiCompatAdapter, EguiNativeDragDropBridge, EguiRuntimeAdapter, EguiSurfaceBridge,
    };
    use katana_ui_core::adapter_contract::WidgetAdapter;
    use katana_ui_core::atom::{Button, Text};
    use katana_ui_core::event::{DragEvent, UiEvent};
    use katana_ui_core::interaction::drag_and_drop::{DragData, DropEffect, OS_URL_TAG};
    use katana_ui_core::layout::{Column, Row};
    use katana_ui_core::render_model::UiNodeId;
    use katana_ui_core::render_model::{RenderContext, UiNodeKind, UiTree};
    use katana_ui_core::runtime::{AppConfig, RuntimeAdapter};
    use katana_ui_core::surface::{PaintRequest, SurfaceMetrics};
    use katana_ui_core::theme::ThemeId;
    use katana_ui_core::window::{WindowConfig, WindowId};

    #[test]
    fn maps_text_button_row_column_skeleton() {
        let tree = UiTree::new(
            Column::new()
                .child(Row::new().child(Text::new("Title")))
                .child(Button::new("Save")),
        );
        let plan = EguiCompatAdapter.render_tree(
            &tree,
            &RenderContext::new(ThemeId::new("light"), 320.0, 240.0),
        );

        assert_eq!(UiNodeKind::Column, plan.root_kind);
        assert_eq!(2, plan.child_count);
    }

    #[test]
    fn runtime_and_surface_skeleton_use_kuc_types() {
        let mut runtime = EguiRuntimeAdapter::default();
        let handle = runtime.run(AppConfig::new("egui"), vec![WindowConfig::new("Main")]);
        let request = PaintRequest::new(
            WindowId::new("main"),
            SurfaceMetrics::new(320.0, 240.0, 1.0, 96.0),
        )
        .with_tree(UiTree::new(Text::new("Preview")));

        assert_eq!("egui", handle.app_id());
        assert_eq!(
            UiNodeKind::Text,
            EguiSurfaceBridge.paint(&request).root_kind
        );
    }

    #[test]
    fn native_dnd_stub_maps_start_drop_and_cancel() {
        let bridge = EguiNativeDragDropBridge;
        let data = DragData::new(OS_URL_TAG, serde_json::json!("https://example.test"));
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
