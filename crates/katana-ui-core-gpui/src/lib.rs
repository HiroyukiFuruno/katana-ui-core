pub mod ime_contract;

use katana_ui_core::adapter_contract::{
    NativeDragDropBridge, WidgetAdapter, WindowControlDispatchRequest,
};
use katana_ui_core::event::UiEvent;
use katana_ui_core::interaction::drag_and_drop::{DragData, DropEffect};
use katana_ui_core::render_model::UiNodeId;
use katana_ui_core::render_model::{RenderContext, UiImageSurfaceRenderPlan, UiNodeKind, UiTree};
use katana_ui_core::runtime::{AppConfig, AppHandle, AppLifecycle, RuntimeAdapter};
use katana_ui_core::surface::PaintRequest;
use katana_ui_core::window::{WindowCommand, WindowConfig};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GpuiCompatAdapter;

impl WidgetAdapter for GpuiCompatAdapter {
    type Output = GpuiRenderPlan;

    fn render_tree(&self, tree: &UiTree, _context: &RenderContext) -> Self::Output {
        GpuiRenderPlan {
            root_kind: tree.root().kind(),
            child_count: tree.root().children().len(),
            image_surfaces: UiImageSurfaceRenderPlan::collect_from_tree(tree),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpuiRenderPlan {
    pub root_kind: UiNodeKind,
    pub child_count: usize,
    pub image_surfaces: Vec<UiImageSurfaceRenderPlan>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GpuiRuntimeAdapter {
    lifecycle: Vec<AppLifecycle>,
}

impl RuntimeAdapter for GpuiRuntimeAdapter {
    fn run(&mut self, config: AppConfig, windows: Vec<WindowConfig>) -> AppHandle {
        self.lifecycle.push(AppLifecycle::Started);
        AppHandle::new(
            config.app_id,
            windows.into_iter().map(WindowConfig::into_id).collect(),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GpuiWindowAction {
    Command(String),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GpuiWindowBridge;

impl GpuiWindowBridge {
    #[must_use]
    pub fn map_command(self, command: &WindowCommand) -> GpuiWindowAction {
        GpuiWindowAction::Command(format!("{command:?}"))
    }

    #[must_use]
    pub fn map_window_control(self, request: &WindowControlDispatchRequest) -> GpuiWindowAction {
        self.map_command(&request.command())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GpuiSurfaceBridge;

impl GpuiSurfaceBridge {
    #[must_use]
    pub fn paint(self, request: &PaintRequest) -> GpuiRenderPlan {
        GpuiRenderPlan {
            root_kind: request.tree().root().kind(),
            child_count: request.tree().root().children().len(),
            image_surfaces: UiImageSurfaceRenderPlan::collect_from_tree(request.tree()),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GpuiNativeDragDropBridge;

impl GpuiNativeDragDropBridge {
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
        GpuiCompatAdapter, GpuiNativeDragDropBridge, GpuiRuntimeAdapter, GpuiSurfaceBridge,
        GpuiWindowAction, GpuiWindowBridge,
    };
    use katana_ui_core::adapter_contract::{WidgetAdapter, WindowControlDispatchRequest};
    use katana_ui_core::atom::{Button, ImageSurface, Text};
    use katana_ui_core::event::{DragEvent, UiEvent};
    use katana_ui_core::interaction::drag_and_drop::{DragData, DropEffect, OS_TEXT_TAG};
    use katana_ui_core::layout::{Column, Row};
    use katana_ui_core::molecule::selection::window_control_button_group::{
        WindowControlButtonGroupEvent, WindowControlKind,
    };
    use katana_ui_core::render_model::UiNodeId;
    use katana_ui_core::render_model::{RenderContext, UiNodeKind, UiTree};
    use katana_ui_core::render_model::{
        UiImageSurfaceHighlight, UiImageSurfaceValidationError, UiRect,
    };
    use katana_ui_core::runtime::{AppConfig, RuntimeAdapter};
    use katana_ui_core::surface::{PaintRequest, SurfaceMetrics};
    use katana_ui_core::theme::ThemeId;
    use katana_ui_core::window::{WindowCommand, WindowConfig, WindowId};

    #[test]
    fn maps_text_button_row_column_skeleton() {
        let tree = UiTree::new(
            Column::new()
                .child(Row::new().child(Text::new("Title")))
                .child(Button::new("Save")),
        );
        let plan = GpuiCompatAdapter.render_tree(
            &tree,
            &RenderContext::new(ThemeId::new("light"), 320.0, 240.0),
        );

        assert_eq!(UiNodeKind::Column, plan.root_kind);
        assert_eq!(2, plan.child_count);
    }

    #[test]
    fn runtime_and_surface_skeleton_use_kuc_types() {
        let mut runtime = GpuiRuntimeAdapter::default();
        let handle = runtime.run(AppConfig::new("gpui"), vec![WindowConfig::new("Main")]);
        let request = PaintRequest::new(
            WindowId::new("main"),
            SurfaceMetrics::new(320.0, 240.0, 1.0, 96.0),
        )
        .with_tree(UiTree::new(Text::new("Preview")));

        assert_eq!("gpui", handle.app_id());
        assert_eq!(
            UiNodeKind::Text,
            GpuiSurfaceBridge.paint(&request).root_kind
        );
    }

    #[test]
    fn surface_bridge_preserves_image_surface_contract() -> Result<(), UiImageSurfaceValidationError>
    {
        let request = PaintRequest::new(
            WindowId::new("main"),
            SurfaceMetrics::new(320.0, 240.0, 1.0, 96.0),
        )
        .with_tree(UiTree::new(
            ImageSurface::from_rgba("Preview", "surface-sha", 1, 1, vec![12, 24, 36, 255])?
                .highlight_rect(UiImageSurfaceHighlight::search_hit(
                    UiRect::new(1, 2, 3, 4),
                    "search hit",
                )),
        ));
        let plan = GpuiSurfaceBridge.paint(&request);

        assert_eq!(UiNodeKind::ImageSurface, plan.root_kind);
        assert_eq!(1, plan.image_surfaces.len());
        assert_eq!("surface-sha", plan.image_surfaces[0].fingerprint);
        assert_eq!(4, plan.image_surfaces[0].rgba_byte_len);
        assert_eq!(
            UiRect::new(1, 2, 3, 4),
            plan.image_surfaces[0].highlight_rects[0].rect
        );
        Ok(())
    }

    #[test]
    fn native_dnd_stub_maps_start_drop_and_cancel() {
        let bridge = GpuiNativeDragDropBridge;
        let data = DragData::new(OS_TEXT_TAG, serde_json::json!("plain text"));
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

    #[test]
    fn window_control_stub_receives_window_commands() -> Result<(), String> {
        let request = WindowControlDispatchRequest::from_event(
            WindowControlButtonGroupEvent::ControlPressed {
                which: WindowControlKind::Restore,
            },
            WindowId::new("main"),
        )
        .ok_or_else(|| "window control press must dispatch".to_string())?;
        let expected = WindowCommand::Restore {
            window_id: WindowId::new("main"),
        };

        assert_eq!(
            GpuiWindowAction::Command(format!("{expected:?}")),
            GpuiWindowBridge.map_window_control(&request)
        );
        Ok(())
    }
}
