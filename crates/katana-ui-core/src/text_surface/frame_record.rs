use super::accessibility::TextSurfaceAccessibilityTree;
use super::annotation::TextSurfaceAnnotationStyle;
use super::gutter::TextSurfaceGutterRowId;
use super::props::TextSurfaceViewport;
use crate::render_model::{UiIconProps, UiRect};
use crate::text_selection::UiTextSelectionRange;
use serde::{Deserialize, Serialize};

/// Immutable KUC-derived facts for one rendered text-surface frame.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceFrameRecord {
    pub layout_identity: String,
    pub content_bounds: UiRect,
    pub surface_bounds: UiRect,
    pub viewport_bounds: UiRect,
    pub viewport: TextSurfaceViewport,
    /// KUC-derived rows whose layout bounds intersect the current viewport.
    pub visible_logical_rows: Vec<usize>,
    pub caret: usize,
    pub selection_start: usize,
    pub selection_end: usize,
    pub selection: TextSurfaceSelectionFrame,
    pub preedit: Option<TextSurfacePreeditFrame>,
    pub annotations: Vec<TextSurfaceAnnotationFrame>,
    pub gutter: Vec<TextSurfaceGutterFrame>,
    pub accessibility: TextSurfaceAccessibilityTree,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceAnnotationFrame {
    pub id: String,
    pub visual_role: String,
    pub style: TextSurfaceAnnotationStyle,
    pub priority: i32,
    pub tooltip: String,
    pub rects: Vec<UiRect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceGutterFrame {
    pub row_id: TextSurfaceGutterRowId,
    pub logical_row: usize,
    #[serde(default)]
    pub active: bool,
    #[serde(default)]
    pub hovered: bool,
    pub display_label: String,
    pub marker_id: Option<String>,
    pub accessibility_label: String,
    pub accessibility_description: Option<String>,
    pub visual_role: String,
    pub icon: Option<UiIconProps>,
    /// KUC-derived hit, paint and accessibility bounds for an icon-bearing marker.
    ///
    /// `None` preserves legacy non-icon marker behavior: its marker action continues
    /// to occupy the complete row bounds.
    pub marker_bounds: Option<UiRect>,
    pub bounds: UiRect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfacePreeditFrame {
    pub text: String,
    pub range: UiTextSelectionRange,
    pub rects: Vec<UiRect>,
    pub caret: UiRect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceSelectionFrame {
    pub range: UiTextSelectionRange,
    pub rects: Vec<UiRect>,
    pub caret: UiRect,
}
