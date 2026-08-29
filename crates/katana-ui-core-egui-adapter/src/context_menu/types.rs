use katana_ui_core::molecule::selection::{ContextMenuEvent, ContextMenuItemKind};
use katana_ui_core::render_model::{RGBA_CHANNEL_COUNT, UiIconProps, UiRect};
use katana_ui_core::theme::FontToken;
use katana_ui_core_svg_raster::UiSvgRasterError;
use katana_ui_core_text_raster::PlatformTextRasterError;
use serde::{Deserialize, Serialize};

pub(super) const MENU_MIN_WIDTH_PX: u32 = 180;
pub(super) const ROW_HEIGHT_PX: u32 = 30;
pub(super) const MENU_PADDING_PX: u32 = 6;
pub(super) const ITEM_LEFT_PADDING_PX: i32 = 8;
pub(super) const ITEM_TOP_PADDING_PX: i32 = 7;
pub(super) const ICON_SIZE_PX: u32 = 16;
pub(super) const ICON_LABEL_GAP_PX: i32 = 6;

/// Retained KUC-owned actual menu surface. It accepts no consumer geometry.
pub struct EguiContextMenuAdapter {
    pub(super) menu: katana_ui_core::molecule::selection::ContextMenu,
    pub(super) presentation: ContextMenuPresentation,
    pub(super) anchor: Option<crate::text_surface::TextSurfaceContextTargetAnchor>,
    pub(super) submenu_path: Vec<usize>,
    pub(super) scroll_path: Vec<usize>,
    pub(super) vertical_scroll_offset: f32,
    pub(super) focus_return: Option<egui::Id>,
    pub(super) type_ahead: katana_ui_core::molecule::selection::ContextMenuTypeAheadBuffer,
    pub(super) text_rasterizer: katana_ui_core_text_raster::PlatformTextRasterizer,
    pub(super) svg_rasterizer: katana_ui_core_svg_raster::UiSvgRasterizer,
    pub(super) textures: crate::texture_cache::RgbaTextureCache,
}

/// Controlled host presentation with only generic, opaque menu data.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextMenuPresentation {
    pub visible: bool,
    pub items: Vec<ContextMenuPresentationItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextMenuPresentationItem {
    pub id: String,
    pub label: String,
    pub accessibility_label: String,
    pub icon: Option<UiIconProps>,
    pub enabled: bool,
    pub checked: bool,
    pub kind: ContextMenuItemKind,
    pub children: Vec<Self>,
}

impl ContextMenuPresentationItem {
    #[must_use]
    pub fn action(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            accessibility_label: String::new(),
            icon: None,
            enabled: true,
            checked: false,
            kind: ContextMenuItemKind::Action,
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContextMenuRasterStyle {
    pub font: FontToken,
    pub text_color_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub icon_color_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub line_height_px: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextMenuPaintStyle {
    pub background_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub highlighted_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub disabled_rgba: [u8; RGBA_CHANNEL_COUNT],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextMenuPaintTexture {
    pub identity: String,
    pub width: u32,
    pub height: u32,
    pub rgba_pixels: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextMenuPaintOperationKind {
    Fill {
        bounds: UiRect,
        color_rgba: [u8; RGBA_CHANNEL_COUNT],
    },
    Texture {
        bounds: UiRect,
        texture: ContextMenuPaintTexture,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextMenuPaintOperation {
    pub clip_bounds: UiRect,
    pub kind: ContextMenuPaintOperationKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextMenuPaintPlan {
    pub surface_bounds: UiRect,
    pub operations: Vec<ContextMenuPaintOperation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiContextMenuFrameRecord {
    pub bounds: UiRect,
    pub viewport_bounds: UiRect,
    pub highlighted_path: Vec<usize>,
    pub focused: bool,
    pub items: Vec<EguiContextMenuItemFrame>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiContextMenuItemFrame {
    pub id: String,
    pub bounds: UiRect,
    pub disabled: bool,
    pub checked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextMenuArtifactFrame {
    pub record: EguiContextMenuFrameRecord,
    pub paint_plan: ContextMenuPaintPlan,
    pub events: Vec<ContextMenuEvent>,
    pub frame_record_hash: String,
    pub paint_plan_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiContextMenuOutput {
    pub record: Option<EguiContextMenuFrameRecord>,
    pub events: Vec<ContextMenuEvent>,
    pub artifact: Option<ContextMenuArtifactFrame>,
}

#[derive(Debug)]
pub enum ContextMenuAdapterError {
    Raster(PlatformTextRasterError),
    Svg(UiSvgRasterError),
    ArtifactSerialization(String),
}

impl From<PlatformTextRasterError> for ContextMenuAdapterError {
    fn from(value: PlatformTextRasterError) -> Self {
        Self::Raster(value)
    }
}

impl From<UiSvgRasterError> for ContextMenuAdapterError {
    fn from(value: UiSvgRasterError) -> Self {
        Self::Svg(value)
    }
}

impl std::fmt::Display for ContextMenuAdapterError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raster(error) => write!(formatter, "context menu raster failed: {error}"),
            Self::Svg(error) => write!(formatter, "context menu SVG raster failed: {error:?}"),
            Self::ArtifactSerialization(error) => {
                write!(
                    formatter,
                    "context menu artifact serialization failed: {error}"
                )
            }
        }
    }
}

impl std::error::Error for ContextMenuAdapterError {}

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn context_menu_error_conversions_and_display_cover_every_variant() {
        let raster = ContextMenuAdapterError::from(PlatformTextRasterError::EmptyText);
        assert!(raster.to_string().contains("context menu raster failed"));

        let svg = ContextMenuAdapterError::from(UiSvgRasterError::EmptySource);
        assert!(svg.to_string().contains("context menu SVG raster failed"));

        let serialization = ContextMenuAdapterError::ArtifactSerialization("opaque".into());
        assert_eq!(
            serialization.to_string(),
            "context menu artifact serialization failed: opaque"
        );
    }
}
