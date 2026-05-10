mod types;
mod view;

pub use types::{ToggleProps, ToggleSize};

use crate::theme::Theme;
use crate::theme::color::Color;
use view::{thumb_color, thumb_offset_off, thumb_offset_on, thumb_size, track_color, track_dims};

/// Resolved visual properties for `Toggle`.
#[derive(Debug, Clone)]
pub struct ResolvedToggle {
    pub track_width: f32,
    pub track_height: f32,
    pub track_color: Color,
    pub thumb_size: f32,
    pub thumb_offset: f32,
    pub thumb_color: Color,
    pub disabled: bool,
    pub value: bool,
    pub a11y_label: String,
}

/// Builder for the Toggle composite widget.
#[derive(Debug, Clone)]
pub struct Toggle {
    props: ToggleProps,
}

impl Toggle {
    #[must_use]
    pub fn new(a11y_label: impl Into<String>) -> Self {
        Self {
            props: ToggleProps {
                value: false,
                size: ToggleSize::default(),
                disabled: false,
                a11y_label: a11y_label.into(),
            },
        }
    }

    #[must_use]
    pub fn value(mut self, value: bool) -> Self {
        self.props.value = value;
        self
    }

    #[must_use]
    pub fn size(mut self, size: ToggleSize) -> Self {
        self.props.size = size;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedToggle {
        let dims = track_dims(self.props.size);
        let thumb_sz = thumb_size(&dims);
        let thumb_off = if self.props.value {
            thumb_offset_on(&dims)
        } else {
            thumb_offset_off()
        };

        ResolvedToggle {
            track_width: dims.width,
            track_height: dims.height,
            track_color: track_color(self.props.value, self.props.disabled, theme),
            thumb_size: thumb_sz,
            thumb_offset: thumb_off,
            thumb_color: thumb_color(self.props.disabled, theme),
            disabled: self.props.disabled,
            value: self.props.value,
            a11y_label: self.props.a11y_label.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn off_thumb_at_leading_edge() {
        let theme = Theme::default_light();
        let r = Toggle::new("Test").value(false).resolve(&theme);
        assert!(r.thumb_offset < r.track_width / 2.0);
    }

    #[test]
    fn on_thumb_at_trailing_edge() {
        let theme = Theme::default_light();
        let r = Toggle::new("Test").value(true).resolve(&theme);
        assert!(r.thumb_offset >= r.track_width / 2.0);
    }

    #[test]
    fn on_track_color_is_accent() {
        let theme = Theme::default_light();
        let r = Toggle::new("Test").value(true).resolve(&theme);
        assert_eq!(r.track_color, theme.color.accent);
    }

    #[test]
    fn disabled_track_color_is_border() {
        let theme = Theme::default_light();
        let r = Toggle::new("Test").disabled(true).resolve(&theme);
        assert_eq!(r.track_color, theme.color.border);
    }

    #[test]
    fn a11y_label_preserved() {
        let theme = Theme::default_light();
        let r = Toggle::new("Dark mode").resolve(&theme);
        assert_eq!(r.a11y_label, "Dark mode");
    }
}
