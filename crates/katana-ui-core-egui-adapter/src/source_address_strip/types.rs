use crate::text_surface::{EguiTextSurfaceError, TextSurfacePaintStyle, TextSurfaceRasterStyle};
use katana_ui_core::molecule::structured::source_address_strip::SourceAddressSubmission;
use katana_ui_core::render_model::UiRect;
use katana_ui_core::theme::{FontFamily, FontToken};
use serde::{Deserialize, Serialize};

const RGBA_CHANNEL_COUNT: usize = 4;
const DEFAULT_FONT_SIZE: f32 = 14.0;
const DEFAULT_FONT_WEIGHT: u16 = 400;
const DEFAULT_INPUT_LINE_HEIGHT_PX: f32 = 20.0;
const DEFAULT_LIGHT_RGBA: [u8; RGBA_CHANNEL_COUNT] = [220, 220, 220, 255];
const DEFAULT_BACKGROUND_RGBA: [u8; RGBA_CHANNEL_COUNT] = [30, 30, 30, 255];
const DEFAULT_SELECTION_RGBA: [u8; RGBA_CHANNEL_COUNT] = [70, 100, 140, 180];
const DEFAULT_BUTTON_RGBA: [u8; RGBA_CHANNEL_COUNT] = [55, 55, 55, 255];
const DEFAULT_DISABLED_BUTTON_RGBA: [u8; RGBA_CHANNEL_COUNT] = [35, 35, 35, 255];
const DEFAULT_BUTTON_PADDING_PX: u32 = 8;
const DEFAULT_INPUT_WIDTH_PX: u32 = 260;
const DEFAULT_INPUT_HEIGHT_PX: u32 = 28;

#[derive(Debug)]
pub enum EguiSourceAddressStripError {
    TextSurface(EguiTextSurfaceError),
    Raster(katana_ui_core_text_raster::PlatformTextRasterError),
    FrameNotProduced,
    PaintPlanNotProduced,
}

impl From<EguiTextSurfaceError> for EguiSourceAddressStripError {
    fn from(value: EguiTextSurfaceError) -> Self {
        Self::TextSurface(value)
    }
}

impl From<katana_ui_core_text_raster::PlatformTextRasterError> for EguiSourceAddressStripError {
    fn from(value: katana_ui_core_text_raster::PlatformTextRasterError) -> Self {
        Self::Raster(value)
    }
}

impl std::fmt::Display for EguiSourceAddressStripError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TextSurface(e) => write!(f, "source-address text surface failed: {e}"),
            Self::Raster(e) => write!(f, "source-address raster failed: {e}"),
            Self::FrameNotProduced => f.write_str("source-address did not produce an input frame"),
            Self::PaintPlanNotProduced => {
                f.write_str("source-address did not produce a paint plan")
            }
        }
    }
}
impl std::error::Error for EguiSourceAddressStripError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceAddressLabelRasterEvidence {
    pub label_fingerprint: String,
    pub width: u32,
    pub height: u32,
    pub chromatic_pixel_count: usize,
    pub sha256: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceAddressPaintTexture {
    pub identity: String,
    pub width: u32,
    pub height: u32,
    pub rgba_pixels: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceAddressPaintOperationKind {
    Input(crate::text_surface::TextSurfacePaintOperationKind),
    Fill {
        bounds: UiRect,
        color_rgba: [u8; RGBA_CHANNEL_COUNT],
    },
    Texture {
        bounds: UiRect,
        texture: SourceAddressPaintTexture,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceAddressPaintOperation {
    pub clip_bounds: UiRect,
    pub kind: SourceAddressPaintOperationKind,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceAddressPaintPlan {
    pub surface_bounds: UiRect,
    pub operations: Vec<SourceAddressPaintOperation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceAddressRasterEvidenceReceipt {
    pub(super) input_paint_plan_hash: String,
    pub(super) input_has_text_texture: bool,
    pub(super) label_rasters: Vec<SourceAddressLabelRasterEvidence>,
}
impl SourceAddressRasterEvidenceReceipt {
    #[must_use]
    pub fn input_paint_plan_hash(&self) -> &str {
        &self.input_paint_plan_hash
    }
    #[must_use]
    pub const fn input_has_text_texture(&self) -> bool {
        self.input_has_text_texture
    }
    #[must_use]
    pub fn label_rasters(&self) -> &[SourceAddressLabelRasterEvidence] {
        &self.label_rasters
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceAddressRenderStyle {
    pub input_raster: TextSurfaceRasterStyle,
    pub input_paint: TextSurfacePaintStyle,
    pub label_font: FontToken,
    pub label_color_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub button_background_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub button_disabled_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub button_padding_px: u32,
    pub input_width_px: u32,
    pub input_height_px: u32,
}
impl Default for SourceAddressRenderStyle {
    fn default() -> Self {
        let font = FontToken {
            name: "system-ui".to_string(),
            family: FontFamily::Proportional,
            size: DEFAULT_FONT_SIZE,
            weight: DEFAULT_FONT_WEIGHT,
        };
        Self {
            input_raster: TextSurfaceRasterStyle::new(
                font.clone(),
                DEFAULT_LIGHT_RGBA,
                DEFAULT_INPUT_LINE_HEIGHT_PX,
            ),
            input_paint: TextSurfacePaintStyle {
                background_rgba: DEFAULT_BACKGROUND_RGBA,
                gutter_background_rgba: DEFAULT_BACKGROUND_RGBA,
                gutter_paints: Vec::new(),
                selection_rgba: DEFAULT_SELECTION_RGBA,
                preedit_rgba: DEFAULT_LIGHT_RGBA,
                caret_rgba: DEFAULT_LIGHT_RGBA,
                annotation_paints: Vec::new(),
            },
            label_font: font,
            label_color_rgba: DEFAULT_LIGHT_RGBA,
            button_background_rgba: DEFAULT_BUTTON_RGBA,
            button_disabled_rgba: DEFAULT_DISABLED_BUTTON_RGBA,
            button_padding_px: DEFAULT_BUTTON_PADDING_PX,
            input_width_px: DEFAULT_INPUT_WIDTH_PX,
            input_height_px: DEFAULT_INPUT_HEIGHT_PX,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceAddressFrameEventClass {
    DraftChanged,
    EnabledChanged,
    Focused,
    Blurred,
    HistoryOpened,
    HistoryClosed,
    CandidatesOpened,
    CandidatesClosed,
    HistorySelected,
    CandidateSelected,
    Submitted,
}
pub trait SourceAddressSubmissionForwarder {
    type Error;
    fn forward_submission(
        &mut self,
        submission: SourceAddressSubmission,
    ) -> Result<(), Self::Error>;
}

pub struct EguiSourceAddressStripOutput {
    pub(crate) event_classes: Vec<SourceAddressFrameEventClass>,
    pub(crate) submissions: Vec<SourceAddressSubmission>,
}
impl std::fmt::Debug for EguiSourceAddressStripOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EguiSourceAddressStripOutput")
            .field("event_class_count", &self.event_classes.len())
            .field("submission_count", &self.submissions.len())
            .finish()
    }
}
impl EguiSourceAddressStripOutput {
    #[must_use]
    pub fn event_classes(&self) -> &[SourceAddressFrameEventClass] {
        &self.event_classes
    }
    pub(crate) fn take_submissions(&mut self) -> Vec<SourceAddressSubmission> {
        std::mem::take(&mut self.submissions)
    }
    pub fn forward_submissions_once<F>(self, forwarder: &mut F) -> Result<(), F::Error>
    where
        F: SourceAddressSubmissionForwarder,
    {
        for submission in self.submissions {
            forwarder.forward_submission(submission)?;
        }
        Ok(())
    }
}
