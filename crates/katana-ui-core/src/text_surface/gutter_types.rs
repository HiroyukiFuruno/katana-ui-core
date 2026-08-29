use crate::render_model::UiIconProps;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSurfaceGutterRangeStartAnchor {
    ContainingLine,
    FollowingLine,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceAutomaticGutterRangeOverride {
    pub byte_start: usize,
    pub byte_end: usize,
    pub start_anchor: TextSurfaceGutterRangeStartAnchor,
    pub marker_id: String,
    pub priority: i32,
    pub accessibility_label: String,
    pub accessibility_description: Option<String>,
    pub visual_role: String,
    pub icon: Option<UiIconProps>,
}

/// Opaque KUC-issued identity for an automatic gutter row.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TextSurfaceGutterRowId(String);

impl TextSurfaceGutterRowId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn for_logical_row(logical_row: usize) -> Self {
        Self(format!("kuc-gutter-row-{logical_row}"))
    }
}

/// Sparse metadata for a KUC-issued automatic row.
///
/// Consumer input intentionally has no label, logical row or geometry field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceAutomaticGutterOverride {
    pub marker_id: Option<String>,
    pub accessibility_label: String,
    pub accessibility_description: Option<String>,
    pub visual_role: String,
}

/// Controlled-consumer automatic gutter data.
///
/// `row_id` values originate only from KUC frame records. KUC derives the gutter's labels,
/// bounds and raster-measured width; none of those values enter this DTO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceAutomaticGutterPresentation {
    pub overrides: Vec<(TextSurfaceGutterRowId, TextSurfaceAutomaticGutterOverride)>,
    pub range_overrides: Vec<TextSurfaceAutomaticGutterRangeOverride>,
    pub hovered_rows: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceGutterRow {
    pub logical_row: usize,
    pub display_label: String,
    pub marker_id: Option<String>,
    pub accessibility_label: String,
    pub accessibility_description: Option<String>,
    pub visual_role: String,
    pub icon: Option<UiIconProps>,
}

/// Legacy gutter props retained for source-compatible consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceGutter {
    pub width: u32,
    pub rows: Vec<TextSurfaceGutterRow>,
    pub automatic_numbered: bool,
    pub(crate) controlled_automatic: Option<TextSurfaceAutomaticGutterPresentation>,
}
