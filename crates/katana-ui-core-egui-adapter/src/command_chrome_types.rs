use super::command_chrome_artifact::{
    CommandChromeArtifactFrame, EguiCommandChromeFloatingArtifactFrame,
    EguiCommandChromeSearchArtifactFrame,
};
use crate::text_surface::{
    EguiTextSurfaceAdapter, EguiTextSurfaceError, EguiTextSurfaceFrameRecord,
    TextSurfacePaintStyle, TextSurfaceRasterStyle,
};
use crate::texture_cache::RgbaTextureCache;
use katana_ui_core::molecule::RgbaColor;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeText, CommandChromeToolbarEvent,
    FloatingCommandToolbarEvent,
};
use katana_ui_core::render_model::{RGBA_CHANNEL_COUNT, UiRect};
use katana_ui_core::text_surface::{TextSurface, TextSurfaceEvent};
use katana_ui_core::theme::FontToken;
use katana_ui_core_svg_raster::{UiSvgRasterError, UiSvgRasterizer};
use katana_ui_core_text_raster::{PlatformTextRasterError, PlatformTextRasterizer};
use serde::{Deserialize, Serialize};

pub struct EguiCommandChromeAdapter {
    pub(super) text_rasterizer: PlatformTextRasterizer,
    pub(super) svg_rasterizer: UiSvgRasterizer,
    pub(super) textures: RgbaTextureCache,
    pub(super) text_surface_adapter: EguiTextSurfaceAdapter,
    pub(super) search_surfaces: Option<SearchSurfaceState>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandChromeRasterStyle {
    pub font: FontToken,
    pub text_color_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub icon_color: RgbaColor,
    pub line_height_px: f32,
    pub icon_size_px: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandChromePaintStyle {
    pub action_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub hovered_action_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub disabled_action_rgba: [u8; RGBA_CHANNEL_COUNT],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EguiCommandChromeSearchStyle {
    pub input_raster: TextSurfaceRasterStyle,
    pub input_paint: TextSurfacePaintStyle,
    pub input_width_px: u32,
    pub input_height_px: u32,
    pub gap_px: u32,
    pub control_padding_px: u32,
    pub active_control_rgba: [u8; RGBA_CHANNEL_COUNT],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EguiCommandChromeDrawLayer {
    PanelFill,
    ActionFill,
    IconTexture,
    TextTexture,
    FocusRing,
    TooltipFill,
    TooltipTexture,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiCommandChromeActionFrame {
    pub action_id: String,
    pub bounds: UiRect,
    pub secondary_trigger_bounds: Option<UiRect>,
    pub icon_raster_identity: Option<String>,
    pub label_raster_identity: Option<String>,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiCommandChromeDropdownItemFrame {
    pub item_id: String,
    pub bounds: UiRect,
    pub icon_raster_identity: Option<String>,
    pub label_raster_identity: String,
    pub disabled: bool,
    pub selected: bool,
    pub focused: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiCommandChromeDropdownFrame {
    pub action_id: String,
    pub trigger_bounds: UiRect,
    pub bounds: UiRect,
    pub items: Vec<EguiCommandChromeDropdownItemFrame>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiCommandChromeFrameRecord {
    pub bounds: UiRect,
    pub actions: Vec<EguiCommandChromeActionFrame>,
    pub dropdown: Option<EguiCommandChromeDropdownFrame>,
    pub hidden_item_ids: Vec<String>,
    pub focused_action_id: Option<String>,
    pub layers: Vec<EguiCommandChromeDrawLayer>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiCommandChromeOutput {
    pub record: EguiCommandChromeFrameRecord,
    pub events: Vec<CommandChromeToolbarEvent>,
    pub artifact: CommandChromeArtifactFrame,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiCommandChromeFloatingFrameRecord {
    pub surface_id: String,
    pub anchor_bounds: UiRect,
    pub panel_bounds: UiRect,
    pub toolbar: EguiCommandChromeFrameRecord,
    pub tooltip_bounds: Option<UiRect>,
    pub tooltip_raster_identity: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiCommandChromeFloatingOutput {
    pub record: Option<EguiCommandChromeFloatingFrameRecord>,
    pub events: Vec<FloatingCommandToolbarEvent>,
    pub artifact: Option<EguiCommandChromeFloatingArtifactFrame>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiCommandChromeSearchControlFrame {
    pub control_id: String,
    pub bounds: UiRect,
    pub raster_identity: String,
    pub disabled: bool,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiCommandChromeSearchFrameRecord {
    pub bounds: UiRect,
    pub query: EguiTextSurfaceFrameRecord,
    pub replace: Option<EguiTextSurfaceFrameRecord>,
    pub controls: Vec<EguiCommandChromeSearchControlFrame>,
    pub focused_target: Option<String>,
    pub layers: Vec<EguiCommandChromeDrawLayer>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiCommandChromeSearchOutput {
    pub record: EguiCommandChromeSearchFrameRecord,
    pub events: Vec<CommandChromeSearchEvent>,
    pub text_events: Vec<TextSurfaceEvent>,
    pub artifact: EguiCommandChromeSearchArtifactFrame,
}

#[derive(Debug)]
pub enum EguiCommandChromeError {
    Text(PlatformTextRasterError),
    Svg(UiSvgRasterError),
    TextSurface(EguiTextSurfaceError),
    ArtifactSerialization(String),
}

impl From<PlatformTextRasterError> for EguiCommandChromeError {
    fn from(value: PlatformTextRasterError) -> Self {
        Self::Text(value)
    }
}

impl From<UiSvgRasterError> for EguiCommandChromeError {
    fn from(value: UiSvgRasterError) -> Self {
        Self::Svg(value)
    }
}

impl From<EguiTextSurfaceError> for EguiCommandChromeError {
    fn from(value: EguiTextSurfaceError) -> Self {
        Self::TextSurface(value)
    }
}

impl std::fmt::Display for EguiCommandChromeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(error) => write!(formatter, "command chrome text raster failed: {error}"),
            Self::Svg(error) => write!(formatter, "command chrome SVG raster failed: {error:?}"),
            Self::TextSurface(error) => {
                write!(formatter, "command chrome text surface failed: {error}")
            }
            Self::ArtifactSerialization(error) => {
                write!(
                    formatter,
                    "command chrome artifact serialization failed: {error}"
                )
            }
        }
    }
}

pub(super) struct SearchSurfaceState {
    pub(super) strip_state_id: String,
    pub(super) query: TextSurface,
    pub(super) replace: TextSurface,
    pub(super) query_presentation: CommandChromeText,
    pub(super) replace_presentation: CommandChromeText,
    pub(super) input_width_px: u32,
    pub(super) input_height_px: u32,
    pub(super) replace_disabled: bool,
}

impl std::error::Error for EguiCommandChromeError {}

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn command_chrome_error_conversions_and_display_cover_every_variant() {
        let text = EguiCommandChromeError::from(PlatformTextRasterError::EmptyText);
        assert!(
            text.to_string()
                .contains("command chrome text raster failed")
        );

        let svg = EguiCommandChromeError::from(UiSvgRasterError::EmptySource);
        assert!(svg.to_string().contains("command chrome SVG raster failed"));

        let surface = EguiCommandChromeError::from(EguiTextSurfaceError::FrameNotProduced);
        assert!(
            surface
                .to_string()
                .contains("command chrome text surface failed")
        );

        let serialization = EguiCommandChromeError::ArtifactSerialization("opaque".into());
        assert_eq!(
            serialization.to_string(),
            "command chrome artifact serialization failed: opaque"
        );
    }
}

#[derive(Clone)]
pub(super) struct RenderedAction {
    pub(super) bounds: UiRect,
    pub(super) icon: Option<RenderedRaster>,
    pub(super) label: Option<RenderedRaster>,
    pub(super) icon_identity: Option<String>,
    pub(super) label_identity: Option<String>,
}

#[derive(Clone)]
pub(super) struct RenderedRaster {
    pub(super) identity: String,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) physical_width: u32,
    pub(super) physical_height: u32,
    pub(super) pixels: Vec<u8>,
}

impl RenderedRaster {
    pub(super) fn new(
        identity: String,
        width: u32,
        height: u32,
        pixels: Vec<u8>,
        scale: f32,
    ) -> Self {
        Self {
            identity,
            width: physical_to_logical(width, scale),
            height: physical_to_logical(height, scale),
            physical_width: width,
            physical_height: height,
            pixels,
        }
    }
}

pub(super) fn logical_to_physical(value: u32, scale: f32) -> u32 {
    ((value as f32 * scale).round().max(1.0)) as u32
}

fn physical_to_logical(value: u32, scale: f32) -> u32 {
    ((value as f32 / scale.max(1.0)).ceil().max(1.0)) as u32
}
