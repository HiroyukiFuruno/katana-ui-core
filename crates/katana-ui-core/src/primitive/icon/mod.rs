mod types;

pub use types::{IconProps, IconSize, IconSource};

use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::views::{Decorators, svg};

/// Resolved, ready-to-render icon properties.
#[derive(Debug, Clone)]
pub struct ResolvedIcon {
    pub svg_content: String,
    pub size_px: f32,
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
    pub color_a: u8,
    pub is_empty: bool,
}

/// Builder for the Icon primitive.
#[derive(Debug, Clone)]
pub struct Icon {
    source: IconSource,
    props: IconProps,
}

impl Icon {
    #[must_use]
    pub fn new(source: IconSource) -> Self {
        Self {
            source,
            props: IconProps::default(),
        }
    }

    #[must_use]
    pub fn size(mut self, size: IconSize) -> Self {
        self.props.size = size;
        self
    }

    #[must_use]
    pub fn color_override(mut self, color: Color) -> Self {
        self.props.color_override = Some(color);
        self
    }

    /// Resolve rendering properties against the given theme.
    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedIcon {
        let svg_content = self.source.to_string_lossy().into_owned();
        let is_empty = svg_content.is_empty();
        let size_px = self.props.size.resolve_px(&theme.spacing);
        let c = self
            .props
            .color_override
            .as_ref()
            .unwrap_or(&theme.color.text);
        ResolvedIcon {
            svg_content,
            size_px,
            color_r: c.r,
            color_g: c.g,
            color_b: c.b,
            color_a: c.a,
            is_empty,
        }
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let resolved = self.resolve(&theme);
        let icon_color = floem::peniko::Color::rgba8(
            resolved.color_r,
            resolved.color_g,
            resolved.color_b,
            resolved.color_a,
        );
        svg(resolved.svg_content).style(move |style| {
            style
                .width(resolved.size_px)
                .height(resolved.size_px)
                .color(icon_color)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    const VALID_SVG: &[u8] =
        b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'><circle cx='8' cy='8' r='6'/></svg>";
    const INVALID_UTF8: &[u8] = &[0xFF, 0xFE];

    #[test]
    fn valid_svg_bytes_produce_non_empty_content() {
        let theme = Theme::default_light();
        let resolved = Icon::new(IconSource::SvgBytes(VALID_SVG)).resolve(&theme);
        assert!(!resolved.is_empty);
    }

    #[test]
    fn invalid_utf8_bytes_produce_empty_content_without_panic() {
        let theme = Theme::default_light();
        let resolved = Icon::new(IconSource::SvgBytes(INVALID_UTF8)).resolve(&theme);
        assert!(resolved.is_empty);
    }
}
