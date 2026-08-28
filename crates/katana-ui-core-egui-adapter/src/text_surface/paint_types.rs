use katana_ui_core::render_model::{RGBA_CHANNEL_COUNT, UiRect};
use katana_ui_core::theme::FontToken;
use serde::{Deserialize, Serialize};

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

impl TextSurfaceGutterPaint {
    #[must_use]
    pub fn new(visual_role: impl Into<String>, foreground_rgba: [u8; RGBA_CHANNEL_COUNT]) -> Self {
        Self {
            visual_role: visual_role.into(),
            foreground_rgba,
            background_rgba: None,
            active_background_rgba: None,
            hovered_background_rgba: None,
            active_foreground_rgba: None,
            hovered_foreground_rgba: None,
        }
    }

    #[must_use]
    pub const fn background(mut self, background_rgba: [u8; RGBA_CHANNEL_COUNT]) -> Self {
        self.background_rgba = Some(background_rgba);
        self
    }

    #[must_use]
    pub fn active_background(mut self, active_background_rgba: [u8; RGBA_CHANNEL_COUNT]) -> Self {
        self.active_background_rgba = Some(active_background_rgba);
        self
    }

    #[must_use]
    pub fn hovered_background(mut self, hovered_background_rgba: [u8; RGBA_CHANNEL_COUNT]) -> Self {
        self.hovered_background_rgba = Some(hovered_background_rgba);
        self
    }

    #[must_use]
    pub fn active_foreground(mut self, active_foreground_rgba: [u8; RGBA_CHANNEL_COUNT]) -> Self {
        self.active_foreground_rgba = Some(active_foreground_rgba);
        self
    }

    #[must_use]
    pub fn hovered_foreground(mut self, hovered_foreground_rgba: [u8; RGBA_CHANNEL_COUNT]) -> Self {
        self.hovered_foreground_rgba = Some(hovered_foreground_rgba);
        self
    }

    pub(super) fn background_for_state(
        &self,
        active: bool,
        hovered: bool,
    ) -> Option<[u8; RGBA_CHANNEL_COUNT]> {
        if active {
            self.active_background_rgba.or(self.background_rgba)
        } else if hovered {
            self.hovered_background_rgba.or(self.background_rgba)
        } else {
            self.background_rgba
        }
    }

    pub(super) fn foreground_for_state(
        &self,
        active: bool,
        hovered: bool,
    ) -> [u8; RGBA_CHANNEL_COUNT] {
        if active {
            self.active_foreground_rgba.unwrap_or(self.foreground_rgba)
        } else if hovered {
            self.hovered_foreground_rgba.unwrap_or(self.foreground_rgba)
        } else {
            self.foreground_rgba
        }
    }
}

impl TextSurfaceAnnotationPaint {
    #[must_use]
    pub fn new(visual_role: impl Into<String>, color_rgba: [u8; RGBA_CHANNEL_COUNT]) -> Self {
        Self {
            visual_role: visual_role.into(),
            color_rgba,
        }
    }
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

impl TextSurfacePaintStyle {
    #[must_use]
    pub fn annotation_color(&self, visual_role: &str) -> [u8; RGBA_CHANNEL_COUNT] {
        self.annotation_paints
            .iter()
            .find(|paint| paint.visual_role == visual_role)
            .map_or(self.preedit_rgba, |paint| paint.color_rgba)
    }

    #[must_use]
    pub fn gutter_paint(&self, visual_role: &str) -> Option<&TextSurfaceGutterPaint> {
        (!visual_role.is_empty())
            .then(|| {
                self.gutter_paints
                    .iter()
                    .find(|paint| paint.visual_role == visual_role)
            })
            .flatten()
    }

    pub fn gutter_foreground_rgba(
        &self,
        visual_role: &str,
        active: bool,
        hovered: bool,
    ) -> Option<[u8; RGBA_CHANNEL_COUNT]> {
        self.gutter_paint(visual_role)
            .map(|paint| paint.foreground_for_state(active, hovered))
    }

    pub fn gutter_background_rgba(
        &self,
        visual_role: &str,
        active: bool,
        hovered: bool,
    ) -> Option<[u8; RGBA_CHANNEL_COUNT]> {
        self.gutter_paint(visual_role).and_then(|paint| {
            paint
                .background_for_state(active, hovered)
                .or(Some(self.gutter_background_rgba))
        })
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gutter_paint_state_builders_cover_active_hovered_and_default_colors() {
        let paint = TextSurfaceGutterPaint::new("line", [1, 2, 3, 4])
            .background([5, 6, 7, 8])
            .active_background([9, 10, 11, 12])
            .hovered_background([13, 14, 15, 16])
            .active_foreground([17, 18, 19, 20])
            .hovered_foreground([21, 22, 23, 24]);
        assert_eq!(
            paint.background_for_state(true, false),
            Some([9, 10, 11, 12])
        );
        assert_eq!(
            paint.background_for_state(false, true),
            Some([13, 14, 15, 16])
        );
        assert_eq!(paint.background_for_state(false, false), Some([5, 6, 7, 8]));
        assert_eq!(paint.foreground_for_state(true, false), [17, 18, 19, 20]);
        assert_eq!(paint.foreground_for_state(false, true), [21, 22, 23, 24]);
        assert_eq!(paint.foreground_for_state(false, false), [1, 2, 3, 4]);
    }
}
