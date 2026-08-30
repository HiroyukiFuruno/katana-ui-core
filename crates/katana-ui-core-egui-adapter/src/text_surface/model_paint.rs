use super::model::{TextSurfaceAnnotationPaint, TextSurfaceGutterPaint, TextSurfacePaintStyle};
use katana_ui_core::render_model::RGBA_CHANNEL_COUNT;

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

    fn background_for_state(
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

    fn foreground_for_state(&self, active: bool, hovered: bool) -> [u8; RGBA_CHANNEL_COUNT] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gutter_paint_state_fallbacks_follow_active_hovered_priority() {
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

    #[test]
    fn gutter_paint_state_defaults_fall_back_to_none_or_base_foreground() {
        let paint = TextSurfaceGutterPaint::new("line", [9, 8, 7, 6]);

        assert_eq!(paint.background_for_state(false, false), None);
        assert_eq!(paint.foreground_for_state(true, false), [9, 8, 7, 6]);
    }
}
