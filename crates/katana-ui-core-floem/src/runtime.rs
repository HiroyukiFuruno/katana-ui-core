use crate::{FloemAdapter, FloemRenderPlan};
use katana_ui_core::adapter_contract::WidgetAdapter;
use katana_ui_core::render_model::RenderContext;
use katana_ui_core::runtime::{AppConfig, AppHandle, AppLifecycle, RuntimeAdapter};
use katana_ui_core::surface::PaintRequest;
use katana_ui_core::theme::ThemeId;
use katana_ui_core::window::{WindowCommand, WindowConfig};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FloemRuntimeAdapter {
    lifecycle: Vec<AppLifecycle>,
}

impl FloemRuntimeAdapter {
    #[must_use]
    pub fn lifecycle(&self) -> &[AppLifecycle] {
        &self.lifecycle
    }
}

impl RuntimeAdapter for FloemRuntimeAdapter {
    fn run(&mut self, config: AppConfig, windows: Vec<WindowConfig>) -> AppHandle {
        self.lifecycle.push(AppLifecycle::Started);
        AppHandle::new(
            config.app_id,
            windows.into_iter().map(WindowConfig::into_id).collect(),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FloemWindowAction {
    SetTitle(String),
    Resize(f32, f32),
    Move(f32, f32),
    Focus,
    Minimize,
    Maximize,
    Restore,
    Close,
    Fullscreen(bool),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FloemWindowBridge;

impl FloemWindowBridge {
    #[must_use]
    pub fn map_command(self, command: &WindowCommand) -> FloemWindowAction {
        match command {
            WindowCommand::SetTitle { title, .. } => FloemWindowAction::SetTitle(title.clone()),
            WindowCommand::SetSize { size, .. } => {
                FloemWindowAction::Resize(size.width, size.height)
            }
            WindowCommand::SetPosition { x, y, .. } => FloemWindowAction::Move(*x, *y),
            WindowCommand::Focus { .. } => FloemWindowAction::Focus,
            WindowCommand::Minimize { .. } => FloemWindowAction::Minimize,
            WindowCommand::Maximize { .. } => FloemWindowAction::Maximize,
            WindowCommand::Restore { .. } => FloemWindowAction::Restore,
            WindowCommand::Close { .. } => FloemWindowAction::Close,
            WindowCommand::Fullscreen { enabled, .. } => FloemWindowAction::Fullscreen(*enabled),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FloemSurfaceBridge;

impl FloemSurfaceBridge {
    #[must_use]
    pub fn paint(self, request: &PaintRequest, theme_id: ThemeId) -> FloemRenderPlan {
        let metrics = request.metrics();
        let context = RenderContext::new(theme_id, metrics.logical_width, metrics.logical_height);
        FloemAdapter.render_tree(request.tree(), &context)
    }
}

#[cfg(test)]
mod tests {
    use super::{FloemRuntimeAdapter, FloemSurfaceBridge, FloemWindowAction, FloemWindowBridge};
    use katana_ui_core::atom::Text;
    use katana_ui_core::layout::Row;
    use katana_ui_core::render_model::UiTree;
    use katana_ui_core::runtime::{AppConfig, Application, RuntimeAdapter};
    use katana_ui_core::surface::{PaintRequest, SurfaceMetrics};
    use katana_ui_core::theme::ThemeId;
    use katana_ui_core::window::{WindowCommand, WindowConfig, WindowId, WindowSize};

    #[test]
    fn runtime_returns_kuc_app_handle() {
        let mut adapter = FloemRuntimeAdapter::default();
        let handle = adapter.run(AppConfig::new("kuc"), vec![WindowConfig::new("Main")]);

        assert_eq!("kuc", handle.app_id());
        assert_eq!(1, handle.window_ids().len());
    }

    #[test]
    fn application_runs_through_floem_runtime_adapter() {
        let handle = Application::new(AppConfig::new("kuc"))
            .window(WindowConfig::new("Main"))
            .run_with(FloemRuntimeAdapter::default());

        assert_eq!("kuc", handle.app_id());
    }

    #[test]
    fn window_bridge_maps_neutral_command() {
        let action = FloemWindowBridge.map_command(&WindowCommand::SetSize {
            window_id: WindowId::new("main"),
            size: WindowSize::new(320.0, 240.0),
        });

        assert_eq!(FloemWindowAction::Resize(320.0, 240.0), action);
    }

    #[test]
    fn surface_bridge_paints_neutral_tree() {
        let request = PaintRequest::new(
            WindowId::new("main"),
            SurfaceMetrics::new(800.0, 600.0, 1.0, 96.0),
        )
        .with_tree(UiTree::new(Row::new().child(Text::new("Preview"))));
        let plan = FloemSurfaceBridge.paint(&request, ThemeId::new("light"));

        assert_eq!(2, plan.node_count());
    }
}
