use katana_ui_core::render_model::{RGBA_CHANNEL_COUNT, UiRect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EguiTextSurfaceDrawLayer {
    Background,
    Gutter,
    Selection,
    Preedit,
    Annotation,
    PlaceholderTexture,
    TextTexture,
    Caret,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfacePaintTexture {
    pub identity: String,
    pub width: u32,
    pub height: u32,
    pub rgba_pixels: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSurfacePaintOperationKind {
    Fill {
        bounds: UiRect,
        color_rgba: [u8; RGBA_CHANNEL_COUNT],
    },
    Texture {
        bounds: UiRect,
        texture: TextSurfacePaintTexture,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfacePaintOperation {
    pub layer: EguiTextSurfaceDrawLayer,
    pub clip_bounds: UiRect,
    pub kind: TextSurfacePaintOperationKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfacePaintPlan {
    pub surface_bounds: UiRect,
    pub viewport_bounds: UiRect,
    pub operations: Vec<TextSurfacePaintOperation>,
}
