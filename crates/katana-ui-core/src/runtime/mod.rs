use crate::accessibility::AccessibilityLabel;
use crate::window::{WindowConfig, WindowId};
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppHandle {
    app_id: String,
    window_ids: Vec<WindowId>,
}

impl AppHandle {
    #[must_use]
    pub fn new(app_id: impl Into<String>, window_ids: Vec<WindowId>) -> Self {
        Self {
            app_id: app_id.into(),
            window_ids,
        }
    }

    #[must_use]
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    #[must_use]
    pub fn window_ids(&self) -> &[WindowId] {
        &self.window_ids
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppLifecycle {
    Created,
    Started,
    Suspended,
    Resumed,
    Stopped,
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
