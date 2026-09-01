//! Public diagnostics contracts and named visual tokens.

use crate::molecule::DiagnosticsListEvent;
use crate::render_model::UiRect;
use crate::text_raster::PlatformTextRasterError;
use crate::theme::{FontFamily, FontToken};
use serde::{Deserialize, Serialize};

const COLOR_CHANNEL_COUNT: usize = 4;
const COLOR_OPAQUE: u8 = 255;
const FONT_SIZE: f32 = 13.0;
const FONT_WEIGHT: u16 = 400;
const BACKGROUND_CHANNEL: u8 = 37;
const SELECTED_RED: u8 = 54;
const SELECTED_GREEN: u8 = 74;
const SELECTED_BLUE: u8 = 92;
const TEXT_CHANNEL: u8 = 222;
const HEADER_HEIGHT: f32 = 30.0;
const SCOPE_ROW_HEIGHT: f32 = 28.0;
const ROW_HEIGHT: f32 = 34.0;
const PREVIEW_LINE_HEIGHT: f32 = 18.0;
const PREVIEW_PADDING: f32 = 8.0;
const PREVIEW_ADDED_RED: u8 = 35;
const PREVIEW_ADDED_GREEN: u8 = 82;
const PREVIEW_ADDED_BLUE: u8 = 52;
const PREVIEW_REMOVED_RED: u8 = 100;
const PREVIEW_REMOVED_GREEN: u8 = 45;
const PREVIEW_REMOVED_BLUE: u8 = 48;
const PREVIEW_CONTEXT_CHANNEL: u8 = 48;
const VIEWPORT_HEIGHT: f32 = 136.0;
const ACCESSIBILITY_SCROLL_STEP: f32 = 100.0;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsListPaintTexture {
    pub identity: String,
    pub width: u32,
    pub height: u32,
    pub rgba_pixels: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticsListPaintOperationKind {
    Fill {
        bounds: UiRect,
        color_rgba: [u8; COLOR_CHANNEL_COUNT],
    },
    Texture {
        bounds: UiRect,
        texture: DiagnosticsListPaintTexture,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsListPaintOperation {
    pub clip_bounds: UiRect,
    pub kind: DiagnosticsListPaintOperationKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsListPaintPlan {
    pub surface_bounds: UiRect,
    pub operations: Vec<DiagnosticsListPaintOperation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsListRasterEvidence {
    pub text: String,
    pub width: u32,
    pub height: u32,
    pub chromatic_pixel_count: usize,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EguiDiagnosticsListOutput {
    pub(crate) events: Vec<DiagnosticsListEvent>,
    pub(crate) paint_plan: DiagnosticsListPaintPlan,
}

impl EguiDiagnosticsListOutput {
    #[must_use]
    pub fn events(&self) -> &[DiagnosticsListEvent] {
        &self.events
    }
}

#[derive(Debug)]
pub enum EguiDiagnosticsListError {
    Raster(PlatformTextRasterError),
    PaintPlanNotProduced,
}

impl From<PlatformTextRasterError> for EguiDiagnosticsListError {
    fn from(value: PlatformTextRasterError) -> Self {
        Self::Raster(value)
    }
}

impl std::fmt::Display for EguiDiagnosticsListError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raster(error) => write!(formatter, "diagnostics raster failed: {error}"),
            Self::PaintPlanNotProduced => {
                formatter.write_str("diagnostics did not produce a paint plan")
            }
        }
    }
}

impl std::error::Error for EguiDiagnosticsListError {}

#[derive(Debug, Clone)]
pub(crate) struct DiagnosticsListStyle {
    pub(crate) font: FontToken,
    pub(crate) background: [u8; COLOR_CHANNEL_COUNT],
    pub(crate) selected: [u8; COLOR_CHANNEL_COUNT],
    pub(crate) text: [u8; COLOR_CHANNEL_COUNT],
    pub(crate) header_height: f32,
    pub(crate) scope_row_height: f32,
    pub(crate) row_height: f32,
    pub(crate) preview_line_height: f32,
    pub(crate) preview_padding: f32,
    pub(crate) preview_added: [u8; COLOR_CHANNEL_COUNT],
    pub(crate) preview_removed: [u8; COLOR_CHANNEL_COUNT],
    pub(crate) preview_context: [u8; COLOR_CHANNEL_COUNT],
    pub(crate) viewport_height: f32,
    pub(crate) accessibility_scroll_step: f32,
}

impl DiagnosticsListStyle {
    pub(crate) fn standard() -> Self {
        Self {
            font: FontToken {
                name: "system-ui".to_string(),
                family: FontFamily::Proportional,
                size: FONT_SIZE,
                weight: FONT_WEIGHT,
            },
            background: [
                BACKGROUND_CHANNEL,
                BACKGROUND_CHANNEL,
                BACKGROUND_CHANNEL,
                COLOR_OPAQUE,
            ],
            selected: [SELECTED_RED, SELECTED_GREEN, SELECTED_BLUE, COLOR_OPAQUE],
            text: [TEXT_CHANNEL, TEXT_CHANNEL, TEXT_CHANNEL, COLOR_OPAQUE],
            header_height: HEADER_HEIGHT,
            scope_row_height: SCOPE_ROW_HEIGHT,
            row_height: ROW_HEIGHT,
            preview_line_height: PREVIEW_LINE_HEIGHT,
            preview_padding: PREVIEW_PADDING,
            preview_added: [
                PREVIEW_ADDED_RED,
                PREVIEW_ADDED_GREEN,
                PREVIEW_ADDED_BLUE,
                COLOR_OPAQUE,
            ],
            preview_removed: [
                PREVIEW_REMOVED_RED,
                PREVIEW_REMOVED_GREEN,
                PREVIEW_REMOVED_BLUE,
                COLOR_OPAQUE,
            ],
            preview_context: [
                PREVIEW_CONTEXT_CHANNEL,
                PREVIEW_CONTEXT_CHANNEL,
                PREVIEW_CONTEXT_CHANNEL,
                COLOR_OPAQUE,
            ],
            viewport_height: VIEWPORT_HEIGHT,
            accessibility_scroll_step: ACCESSIBILITY_SCROLL_STEP,
        }
    }
}
