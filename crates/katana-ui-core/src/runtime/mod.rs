use crate::accessibility::AccessibilityLabel;
use crate::surface::PaintRequest;
use crate::window::{WindowConfig, WindowEvent, WindowId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub app_id: String,
    pub persistence_path: String,
    pub locale: String,
    pub accessibility_label: AccessibilityLabel,
}

impl AppConfig {
    #[must_use]
    pub fn new(app_id: impl Into<String>) -> Self {
        let app_id = app_id.into();
        Self {
            accessibility_label: AccessibilityLabel::new(app_id.clone()),
            app_id,
            persistence_path: String::new(),
            locale: "en-US".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppHandle {
    app_id: String,
    window_ids: Vec<WindowId>,
    runtime_report: RuntimeRunReport,
}

impl AppHandle {
    #[must_use]
    pub fn new(app_id: impl Into<String>, window_ids: Vec<WindowId>) -> Self {
        Self {
            app_id: app_id.into(),
            window_ids,
            runtime_report: RuntimeRunReport::default(),
        }
    }

    #[must_use]
    pub fn with_runtime_report(mut self, runtime_report: RuntimeRunReport) -> Self {
        self.runtime_report = runtime_report;
        self
    }

    #[must_use]
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    #[must_use]
    pub fn window_ids(&self) -> &[WindowId] {
        &self.window_ids
    }

    #[must_use]
    pub const fn runtime_report(&self) -> &RuntimeRunReport {
        &self.runtime_report
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppLifecycle {
    Created,
    Started,
    Suspended,
    Resumed,
    Stopped,
    ShuttingDown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RuntimeRunReport {
    lifecycle_events: Vec<AppLifecycle>,
    window_events: Vec<WindowEvent>,
    paint_requests: Vec<PaintRequest>,
    redraw_requested: bool,
    shutdown_requested: bool,
}

impl RuntimeRunReport {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn lifecycle(mut self, lifecycle: AppLifecycle) -> Self {
        self.lifecycle_events.push(lifecycle);
        self
    }

    #[must_use]
    pub fn window_event(mut self, event: WindowEvent) -> Self {
        self.window_events.push(event);
        self
    }

    #[must_use]
    pub fn paint_request(mut self, request: PaintRequest) -> Self {
        self.paint_requests.push(request);
        self
    }

    #[must_use]
    pub const fn request_redraw(mut self) -> Self {
        self.redraw_requested = true;
        self
    }

    #[must_use]
    pub const fn request_shutdown(mut self) -> Self {
        self.shutdown_requested = true;
        self
    }

    #[must_use]
    pub fn lifecycle_events(&self) -> &[AppLifecycle] {
        &self.lifecycle_events
    }

    #[must_use]
    pub fn window_events(&self) -> &[WindowEvent] {
        &self.window_events
    }

    #[must_use]
    pub fn paint_requests(&self) -> &[PaintRequest] {
        &self.paint_requests
    }

    #[must_use]
    pub const fn redraw_requested(&self) -> bool {
        self.redraw_requested
    }

    #[must_use]
    pub const fn shutdown_requested(&self) -> bool {
        self.shutdown_requested
    }
}

pub trait RuntimeAdapter {
    fn run(&mut self, config: AppConfig, windows: Vec<WindowConfig>) -> AppHandle;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Application {
    config: AppConfig,
    windows: Vec<WindowConfig>,
}

impl Application {
    #[must_use]
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            windows: Vec::new(),
        }
    }

    #[must_use]
    pub fn builder(config: AppConfig) -> ApplicationBuilder {
        ApplicationBuilder::new(config)
    }

    #[must_use]
    pub fn window(mut self, config: WindowConfig) -> Self {
        self.windows.push(config);
        self
    }

    #[must_use]
    pub fn run_with(mut self, mut adapter: impl RuntimeAdapter) -> AppHandle {
        adapter.run(self.config, std::mem::take(&mut self.windows))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationBuilder {
    config: AppConfig,
    windows: Vec<WindowConfig>,
}

impl ApplicationBuilder {
    #[must_use]
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            windows: Vec::new(),
        }
    }

    #[must_use]
    pub fn window(mut self, config: WindowConfig) -> Self {
        self.windows.push(config);
        self
    }

    #[must_use]
    pub fn run_with(mut self, mut adapter: impl RuntimeAdapter) -> AppHandle {
        adapter.run(self.config, std::mem::take(&mut self.windows))
    }
}
