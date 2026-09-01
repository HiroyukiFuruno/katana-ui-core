use crate::egui::texture_cache::RgbaTextureCache;
use crate::render_model::RGBA_CHANNEL_COUNT;
use crate::svg_raster::UiSvgRasterizer;
use crate::text_raster::PlatformTextMetricsFrame;
use crate::text_raster::PlatformTextRasterizer;
use crate::theme::FontToken;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

mod paint_types;

pub use paint_types::{
    EguiTextSurfaceDrawLayer, TextSurfacePaintOperation, TextSurfacePaintOperationKind,
    TextSurfacePaintPlan, TextSurfacePaintTexture,
};

pub type SharedTextMetrics = Rc<RefCell<PlatformTextMetricsFrame>>;

pub struct EguiTextSurfaceAdapter {
    pub(super) rasterizer: PlatformTextRasterizer,
    pub(super) svg_rasterizer: UiSvgRasterizer,
    pub(super) textures: RgbaTextureCache,
    pub(super) pending_focus_request: Option<bool>,
    pub(super) pointer_exclusion_bounds: Vec<crate::render_model::UiRect>,
    pub(super) metrics: SharedTextMetrics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EguiTextSurfaceKey {
    Enter,
    Escape,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiTextSurfaceInputPolicy {
    pub suppressed_keys: Vec<EguiTextSurfaceKey>,
    pub suppress_text_input: bool,
    pub publish_context_target: bool,
    pub publish_text_input_target: bool,
    pub retain_pointer_focus: bool,
}

impl Default for EguiTextSurfaceInputPolicy {
    fn default() -> Self {
        Self {
            suppressed_keys: Vec::new(),
            suppress_text_input: false,
            publish_context_target: true,
            publish_text_input_target: false,
            retain_pointer_focus: false,
        }
    }
}

impl EguiTextSurfaceInputPolicy {
    #[must_use]
    pub fn suppress(mut self, value: EguiTextSurfaceKey) -> Self {
        if !self.suppressed_keys.contains(&value) {
            self.suppressed_keys.push(value);
        }
        self
    }

    #[must_use]
    pub(crate) fn context_menu() -> Self {
        Self {
            suppressed_keys: vec![
                EguiTextSurfaceKey::Escape,
                EguiTextSurfaceKey::ArrowUp,
                EguiTextSurfaceKey::ArrowDown,
                EguiTextSurfaceKey::ArrowLeft,
                EguiTextSurfaceKey::ArrowRight,
            ],
            suppress_text_input: true,
            publish_context_target: true,
            publish_text_input_target: false,
            retain_pointer_focus: false,
        }
    }

    #[must_use]
    pub(crate) const fn without_context_target(mut self) -> Self {
        self.publish_context_target = false;
        self
    }

    #[must_use]
    pub(crate) const fn with_text_input_target(mut self) -> Self {
        self.publish_text_input_target = true;
        self
    }

    #[must_use]
    pub(crate) const fn with_retained_pointer_focus(mut self) -> Self {
        self.retain_pointer_focus = true;
        self
    }

    pub(super) fn suppresses_event(&self, event: &egui::Event) -> bool {
        if self.suppress_text_input && matches!(event, egui::Event::Text(_) | egui::Event::Ime(_)) {
            return true;
        }
        let egui::Event::Key {
            key, pressed: true, ..
        } = event
        else {
            return false;
        };
        let Some(key) = (match key {
            egui::Key::Enter => Some(EguiTextSurfaceKey::Enter),
            egui::Key::Escape => Some(EguiTextSurfaceKey::Escape),
            egui::Key::ArrowUp => Some(EguiTextSurfaceKey::ArrowUp),
            egui::Key::ArrowDown => Some(EguiTextSurfaceKey::ArrowDown),
            egui::Key::ArrowLeft => Some(EguiTextSurfaceKey::ArrowLeft),
            egui::Key::ArrowRight => Some(EguiTextSurfaceKey::ArrowRight),
            _ => None,
        }) else {
            return false;
        };
        self.suppressed_keys.contains(&key)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextSurfaceRasterStyle {
    pub font: FontToken,
    pub fallback_color_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub line_height_px: f32,
}

impl TextSurfaceRasterStyle {
    #[must_use]
    pub fn new(
        font: FontToken,
        fallback_color_rgba: [u8; RGBA_CHANNEL_COUNT],
        line_height_px: f32,
    ) -> Self {
        Self {
            font,
            fallback_color_rgba,
            line_height_px,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceAnnotationPaint {
    pub visual_role: String,
    pub color_rgba: [u8; RGBA_CHANNEL_COUNT],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceGutterPaint {
    pub visual_role: String,
    pub foreground_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub background_rgba: Option<[u8; RGBA_CHANNEL_COUNT]>,
    pub active_background_rgba: Option<[u8; RGBA_CHANNEL_COUNT]>,
    pub hovered_background_rgba: Option<[u8; RGBA_CHANNEL_COUNT]>,
    pub active_foreground_rgba: Option<[u8; RGBA_CHANNEL_COUNT]>,
    pub hovered_foreground_rgba: Option<[u8; RGBA_CHANNEL_COUNT]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfacePaintStyle {
    pub background_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub gutter_background_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub gutter_paints: Vec<TextSurfaceGutterPaint>,
    pub selection_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub preedit_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub caret_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub annotation_paints: Vec<TextSurfaceAnnotationPaint>,
}

#[cfg(test)]
#[path = "model_tests.rs"]
mod tests;
