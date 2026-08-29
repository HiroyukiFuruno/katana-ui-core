use super::RGBA_CHANNEL_COUNT;
use katana_ui_core::render_model::{UiTextSpan, UiTextSpanStyle};
use katana_ui_core::theme::FontToken;

const DEFAULT_LINE_HEIGHT_RATIO: f32 = 1.45;

#[derive(Debug, Clone, PartialEq)]
pub struct PlatformTextRasterRequest {
    pub spans: Vec<UiTextSpan>,
    pub font: FontToken,
    pub fallback_color_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub line_height_px: f32,
    pub max_width_px: Option<f32>,
    pub scale_factor: f32,
}

impl PlatformTextRasterRequest {
    #[must_use]
    pub fn from_text(
        text: impl AsRef<str>,
        font: FontToken,
        color_rgba: [u8; RGBA_CHANNEL_COUNT],
    ) -> Self {
        let spans = UiTextSpan::emoji_marked_spans(text, UiTextSpanStyle::default());
        Self {
            spans,
            line_height_px: font.size * DEFAULT_LINE_HEIGHT_RATIO,
            font,
            fallback_color_rgba: color_rgba,
            max_width_px: None,
            scale_factor: 1.0,
        }
    }

    #[must_use]
    pub fn text(&self) -> String {
        self.spans.iter().map(|span| span.text.as_str()).collect()
    }

    #[must_use]
    pub fn normalized_scale_factor(&self) -> f32 {
        if self.scale_factor.is_finite() && self.scale_factor > 0.0 {
            self.scale_factor
        } else {
            1.0
        }
    }

    #[must_use]
    pub fn normalized_line_height(&self) -> f32 {
        if self.line_height_px.is_finite() && self.line_height_px > 0.0 {
            self.line_height_px
        } else {
            self.font.size.max(1.0) * DEFAULT_LINE_HEIGHT_RATIO
        }
    }

    #[must_use]
    pub fn normalized_max_width(&self, fallback: f32, maximum: f32) -> f32 {
        self.max_width_px
            .filter(|width| width.is_finite() && *width > 0.0)
            .unwrap_or(fallback)
            .min(maximum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::theme::{FontFamily, FontToken};

    fn font() -> FontToken {
        FontToken {
            name: "contract test".to_string(),
            family: FontFamily::Monospace,
            size: 12.0,
            weight: 400,
        }
    }

    #[test]
    fn normalized_fallback_contracts_for_scale_and_line_height() {
        let mut request =
            PlatformTextRasterRequest::from_text("a", font(), [1; RGBA_CHANNEL_COUNT]);
        request.scale_factor = f32::INFINITY;
        assert_eq!(request.normalized_scale_factor(), 1.0);
        request.scale_factor = -1.0;
        assert_eq!(request.normalized_scale_factor(), 1.0);

        request.line_height_px = f32::NEG_INFINITY;
        assert_eq!(request.normalized_line_height(), 12.0 * 1.45);
    }

    #[test]
    fn normalized_max_width_contracts_fallback_clamp_and_bound() {
        let mut request =
            PlatformTextRasterRequest::from_text("a", font(), [1; RGBA_CHANNEL_COUNT]);
        assert_eq!(request.normalized_max_width(4.0, 10.0), 4.0);

        request.max_width_px = Some(0.0);
        assert_eq!(request.normalized_max_width(4.0, 10.0), 4.0);

        request.max_width_px = Some(f32::INFINITY);
        assert_eq!(request.normalized_max_width(4.0, 10.0), 4.0);

        request.max_width_px = Some(25.0);
        assert_eq!(request.normalized_max_width(4.0, 10.0), 10.0);
    }
}
