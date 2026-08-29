use super::accessibility::TextSurfaceAccessibilityLabels;
use super::annotation::TextSurfaceAnnotation;
use super::focus_request::TextSurfaceFocusRequest;
use super::gutter::{TextSurfaceAutomaticGutterPresentation, TextSurfaceGutter};
use super::scroll_request_types::TextSurfaceScrollRequest;
use crate::atom::TextArea;
use crate::render_model::UiTextSpan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfacePoint {
    pub x: i32,
    pub y: i32,
}

impl TextSurfacePoint {
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceViewport {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scroll_x: i32,
    pub scroll_y: i32,
}

impl TextSurfaceViewport {
    #[must_use]
    pub const fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            scroll_x: 0,
            scroll_y: 0,
        }
    }

    #[must_use]
    pub const fn scroll_offset(mut self, x: i32, y: i32) -> Self {
        self.scroll_x = x;
        self.scroll_y = y;
        self
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSurfaceViewportSizing {
    #[default]
    Fixed,
    AdapterMeasured,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceProps {
    pub text_area: TextArea,
    pub spans: Vec<UiTextSpan>,
    pub annotations: Vec<TextSurfaceAnnotation>,
    pub gutter: Option<TextSurfaceGutter>,
    pub viewport: TextSurfaceViewport,
    pub viewport_sizing: TextSurfaceViewportSizing,
    pub accessibility_label: String,
    pub accessibility_actions: TextSurfaceAccessibilityLabels,
    pub context_target_label: Option<String>,
    pub disabled_reason: Option<String>,
    pub scroll_request: Option<TextSurfaceScrollRequest>,
    pub focus_request: Option<TextSurfaceFocusRequest>,
}

/// Controlled presentation owned by a consumer but rendered and interacted with by KUC.
///
/// All offsets are UTF-8 byte offsets. Synchronization never produces an interaction event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfacePresentation {
    pub value: String,
    pub selection_start: usize,
    pub selection_end: usize,
    pub spans: Vec<UiTextSpan>,
    pub annotations: Vec<TextSurfaceAnnotation>,
    pub automatic_gutter: Option<TextSurfaceAutomaticGutterPresentation>,
    pub accessibility_label: String,
    pub accessibility_actions: TextSurfaceAccessibilityLabels,
    pub context_target_label: Option<String>,
    pub disabled_reason: Option<String>,
    pub readonly: bool,
    pub disabled: bool,
    pub ime_enabled: bool,
    pub scroll_request: Option<TextSurfaceScrollRequest>,
    pub focus_request: Option<TextSurfaceFocusRequest>,
}

impl TextSurfacePresentation {
    #[must_use]
    pub fn from_props(props: &TextSurfaceProps) -> Self {
        let state = props.text_area.state();
        Self {
            value: state.value.clone(),
            selection_start: state.selection.start,
            selection_end: state.selection.end,
            spans: props.spans.clone(),
            annotations: props.annotations.clone(),
            automatic_gutter: None,
            accessibility_label: props.accessibility_label.clone(),
            accessibility_actions: props.accessibility_actions.clone(),
            context_target_label: props.context_target_label.clone(),
            disabled_reason: props.disabled_reason.clone(),
            readonly: state.readonly,
            disabled: state.disabled,
            ime_enabled: props.text_area.options().ime_enabled,
            scroll_request: None,
            focus_request: None,
        }
    }
}

impl TextSurfaceProps {
    #[must_use]
    pub fn new(text_area: TextArea, spans: Vec<UiTextSpan>, viewport: TextSurfaceViewport) -> Self {
        Self {
            text_area,
            spans,
            annotations: Vec::new(),
            gutter: None,
            viewport,
            viewport_sizing: TextSurfaceViewportSizing::default(),
            accessibility_label: String::new(),
            accessibility_actions: TextSurfaceAccessibilityLabels::new(),
            context_target_label: None,
            disabled_reason: None,
            scroll_request: None,
            focus_request: None,
        }
    }

    #[must_use]
    pub const fn adapter_measured_viewport(mut self) -> Self {
        self.viewport_sizing = TextSurfaceViewportSizing::AdapterMeasured;
        self
    }

    #[must_use]
    pub fn annotation(mut self, value: TextSurfaceAnnotation) -> Self {
        self.annotations.push(value);
        self
    }

    #[must_use]
    pub fn gutter(mut self, value: TextSurfaceGutter) -> Self {
        self.gutter = Some(value);
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.accessibility_label = value.into();
        self
    }

    #[must_use]
    pub fn accessibility_actions(mut self, value: TextSurfaceAccessibilityLabels) -> Self {
        self.accessibility_actions = value;
        self
    }

    #[must_use]
    pub fn context_target_label(mut self, value: impl Into<String>) -> Self {
        self.context_target_label = Some(value.into());
        self
    }

    #[must_use]
    pub fn disabled_reason(mut self, value: impl Into<String>) -> Self {
        self.disabled_reason = Some(value.into());
        self
    }
}
