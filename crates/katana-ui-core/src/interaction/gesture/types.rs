use crate::render_model::{UiRect, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSurfaceGestureCapabilities {
    pub pointer_pan: bool,
    pub smooth_scroll_pan: bool,
    pub zoom: bool,
    pub fullscreen: bool,
}

impl UiSurfaceGestureCapabilities {
    #[must_use]
    pub const fn pointer_pan(mut self, value: bool) -> Self {
        self.pointer_pan = value;
        self
    }

    #[must_use]
    pub const fn smooth_scroll_pan(mut self, value: bool) -> Self {
        self.smooth_scroll_pan = value;
        self
    }

    #[must_use]
    pub const fn zoom(mut self, value: bool) -> Self {
        self.zoom = value;
        self
    }

    #[must_use]
    pub const fn fullscreen(mut self, value: bool) -> Self {
        self.fullscreen = value;
        self
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSurfacePoint {
    pub x: i32,
    pub y: i32,
}

impl UiSurfacePoint {
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UiSurfaceGestureInput {
    PointerDown {
        pointer_id: u64,
        position: UiSurfacePoint,
    },
    PointerMove {
        pointer_id: u64,
        position: UiSurfacePoint,
    },
    PointerUp {
        pointer_id: u64,
        position: UiSurfacePoint,
    },
    SmoothScroll {
        position: UiSurfacePoint,
        delta_x: f32,
        delta_y: f32,
    },
    Zoom {
        multiplier: f32,
        center: UiSurfacePoint,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UiSurfaceGestureCommand {
    PanBy {
        delta_x: f32,
        delta_y: f32,
    },
    ZoomBy {
        multiplier: f32,
        center: UiSurfacePoint,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiSurfaceGestureEvent {
    pub target: UiStateId,
    pub input: UiSurfaceGestureInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiSurfaceHostEvent {
    FullscreenChanged { target: UiStateId, fullscreen: bool },
}

#[derive(Debug, Clone, PartialEq)]
pub enum UiSurfaceGestureOverride {
    UseDefault,
    Command(UiSurfaceGestureCommand),
    Ignore,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UiSurfaceGestureOutcome {
    pub target: Option<UiStateId>,
    pub event: Option<UiSurfaceGestureEvent>,
    pub command: Option<UiSurfaceGestureCommand>,
    pub captured: bool,
}

impl UiSurfaceGestureOutcome {
    pub(super) fn unhandled(target: Option<UiStateId>) -> Self {
        Self {
            target,
            event: None,
            command: None,
            captured: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiGestureSurface {
    pub target: UiStateId,
    pub bounds: UiRect,
    pub capabilities: UiSurfaceGestureCapabilities,
    pub fullscreen: bool,
}

impl UiGestureSurface {
    #[must_use]
    pub fn new(target: impl Into<UiStateId>, bounds: UiRect) -> Self {
        Self {
            target: target.into(),
            bounds,
            capabilities: UiSurfaceGestureCapabilities::default(),
            fullscreen: false,
        }
    }

    #[must_use]
    pub const fn capabilities(mut self, value: UiSurfaceGestureCapabilities) -> Self {
        self.capabilities = value;
        self
    }

    #[must_use]
    pub const fn fullscreen(mut self, value: bool) -> Self {
        self.fullscreen = value;
        self
    }

    #[must_use]
    pub fn hit(&self, point: UiSurfacePoint) -> bool {
        super::contains(self.bounds, point)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ActivePointer {
    pub(super) pointer_id: u64,
    pub(super) target: UiStateId,
    pub(super) last: UiSurfacePoint,
}

#[derive(Debug, Default)]
pub struct UiSurfaceGestureController {
    pub(super) surfaces: Vec<UiGestureSurface>,
    pub(super) active_pointer: Option<ActivePointer>,
}
