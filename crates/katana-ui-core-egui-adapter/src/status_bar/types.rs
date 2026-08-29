use katana_ui_core::molecule::StatusBarEvent;
use katana_ui_core::render_model::{RGBA_CHANNEL_COUNT, UiRect};
use katana_ui_core::theme::{FontFamily, FontToken};
use katana_ui_core_text_raster::PlatformTextRasterError;
use serde::{Deserialize, Serialize};

const DEFAULT_FONT_SIZE_PX: f32 = 13.0;
const DEFAULT_FONT_WEIGHT: u16 = 400;
const DEFAULT_BACKGROUND_RGB: u8 = 37;
const DEFAULT_BORDER_RGB: u8 = 67;
const DEFAULT_NEUTRAL_TEXT_RGB: u8 = 222;
const DEFAULT_OPACITY: u8 = 255;
const DEFAULT_SEGMENT_PADDING_PX: u32 = 8;
const DEFAULT_SEGMENT_GAP_PX: u32 = 6;
const DEFAULT_HEIGHT_PX: u32 = 28;
const DEFAULT_RGBA: [u8; RGBA_CHANNEL_COUNT] = [
    DEFAULT_BACKGROUND_RGB,
    DEFAULT_BACKGROUND_RGB,
    DEFAULT_BACKGROUND_RGB,
    DEFAULT_OPACITY,
];
const DEFAULT_BORDER_RGBA: [u8; RGBA_CHANNEL_COUNT] = [
    DEFAULT_BORDER_RGB,
    DEFAULT_BORDER_RGB,
    DEFAULT_BORDER_RGB,
    DEFAULT_OPACITY,
];
const DEFAULT_NEUTRAL_TEXT_RGBA: [u8; RGBA_CHANNEL_COUNT] = [
    DEFAULT_NEUTRAL_TEXT_RGB,
    DEFAULT_NEUTRAL_TEXT_RGB,
    DEFAULT_NEUTRAL_TEXT_RGB,
    DEFAULT_OPACITY,
];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusBarRenderStyle {
    pub font: FontToken,
    pub background_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub border_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub neutral_text_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub segment_padding_px: u32,
    pub segment_gap_px: u32,
    pub height_px: u32,
}

impl StatusBarRenderStyle {
    #[must_use]
    pub fn standard() -> Self {
        Self {
            font: FontToken {
                name: "system-ui".to_owned(),
                family: FontFamily::Proportional,
                size: DEFAULT_FONT_SIZE_PX,
                weight: DEFAULT_FONT_WEIGHT,
            },
            background_rgba: DEFAULT_RGBA,
            border_rgba: DEFAULT_BORDER_RGBA,
            neutral_text_rgba: DEFAULT_NEUTRAL_TEXT_RGBA,
            segment_padding_px: DEFAULT_SEGMENT_PADDING_PX,
            segment_gap_px: DEFAULT_SEGMENT_GAP_PX,
            height_px: DEFAULT_HEIGHT_PX,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBarPaintTexture {
    pub identity: String,
    pub width: u32,
    pub height: u32,
    pub rgba_pixels: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusBarPaintOperationKind {
    Fill {
        bounds: UiRect,
        color_rgba: [u8; RGBA_CHANNEL_COUNT],
    },
    Texture {
        bounds: UiRect,
        texture: StatusBarPaintTexture,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBarPaintOperation {
    pub clip_bounds: UiRect,
    pub kind: StatusBarPaintOperationKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBarPaintPlan {
    pub surface_bounds: UiRect,
    pub operations: Vec<StatusBarPaintOperation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBarLabelRasterEvidence {
    pub label_fingerprint: String,
    pub width: u32,
    pub height: u32,
    pub chromatic_pixel_count: usize,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EguiStatusBarOutput {
    pub(crate) events: Vec<StatusBarEvent>,
    pub(crate) paint_plan: StatusBarPaintPlan,
}

impl EguiStatusBarOutput {
    #[must_use]
    pub fn events(&self) -> &[StatusBarEvent] {
        &self.events
    }
}

#[derive(Debug)]
pub enum EguiStatusBarError {
    Raster(PlatformTextRasterError),
    PaintPlanNotProduced,
}

impl From<PlatformTextRasterError> for EguiStatusBarError {
    fn from(value: PlatformTextRasterError) -> Self {
        Self::Raster(value)
    }
}
impl std::fmt::Display for EguiStatusBarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raster(e) => write!(f, "status-bar raster failed: {e}"),
            Self::PaintPlanNotProduced => f.write_str("status-bar did not produce a paint plan"),
        }
    }
}
impl std::error::Error for EguiStatusBarError {}
