use serde::{Deserialize, Serialize};

mod coordinate;
#[cfg(test)]
mod coordinate_tests;
mod placement;
#[cfg(test)]
mod placement_tests;

pub use coordinate::{
    WindowCanvasPoint, WindowInputNormalizer, WindowSurfacePoint, WindowSurfaceSize,
};
pub use placement::{
    DisplayBounds, ModalWindowPlacement, ModalWindowPlacementError, ModalWindowPlan, WindowPoint,
    WindowRect,
};

const DEFAULT_WINDOW_WIDTH: f32 = 1024.0;
const DEFAULT_WINDOW_HEIGHT: f32 = 768.0;
const MIN_WINDOW_WIDTH: f32 = 320.0;
const MIN_WINDOW_HEIGHT: f32 = 240.0;
const MAX_WINDOW_WIDTH: f32 = 7680.0;
const MAX_WINDOW_HEIGHT: f32 = 4320.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WindowId(String);

impl WindowId {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

impl WindowSize {
    #[must_use]
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowConfig {
    id: WindowId,
    pub title: String,
    pub size: WindowSize,
    pub min_size: WindowSize,
    pub max_size: WindowSize,
    pub fullscreen: bool,
    pub decorations: bool,
    pub icon: Vec<u8>,
}

impl WindowConfig {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        let title = title.into();
        Self {
            id: WindowId::new(format!("window:{title}")),
            title,
            size: WindowSize::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
            min_size: WindowSize::new(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT),
            max_size: WindowSize::new(MAX_WINDOW_WIDTH, MAX_WINDOW_HEIGHT),
            fullscreen: false,
            decorations: true,
            icon: Vec::new(),
        }
    }

    #[must_use]
    pub fn into_id(self) -> WindowId {
        self.id
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WindowEvent {
    Created(WindowId),
    Focused(WindowId),
    Closed(WindowId),
    DisplayChanged(DisplayInfo),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WindowCommand {
    SetTitle {
        window_id: WindowId,
        title: String,
    },
    SetSize {
        window_id: WindowId,
        size: WindowSize,
    },
    SetPosition {
        window_id: WindowId,
        x: f32,
        y: f32,
    },
    Focus {
        window_id: WindowId,
    },
    Minimize {
        window_id: WindowId,
    },
    Maximize {
        window_id: WindowId,
    },
    Restore {
        window_id: WindowId,
    },
    Close {
        window_id: WindowId,
    },
    Fullscreen {
        window_id: WindowId,
        enabled: bool,
    },
}

impl WindowCommand {
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match self {
            Self::SetTitle { title, .. } => Some(title.as_str()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowManager {
    windows: Vec<WindowConfig>,
}

impl WindowManager {
    #[must_use]
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
        }
    }

    pub fn create(&mut self, config: WindowConfig) -> WindowId {
        let id = config.id.clone();
        self.windows.push(config);
        id
    }

    #[must_use]
    pub fn windows(&self) -> &[WindowConfig] {
        &self.windows
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisplayInfo {
    pub name: String,
    pub scale_factor: f32,
    pub width: f32,
    pub height: f32,
}
