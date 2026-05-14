mod types;
mod view;

pub use types::{AlignCenterWrapperProps, ResolvedAlignCenterWrapper};

use crate::theme::Theme;
use crate::theme::color::Color;

/// Builder for a thin wrapper that centers child views.
#[derive(Debug, Clone)]
pub struct AlignCenterWrapper {
    props: AlignCenterWrapperProps,
}

impl AlignCenterWrapper {
    /// Creates a wrapper centered on both axes by default.
    #[must_use]
    pub fn new() -> Self {
        Self {
            props: AlignCenterWrapperProps {
                horizontal: true,
                vertical: true,
                width: None,
                height: None,
                padding: 0.0,
                gap: 0.0,
                background: None,
                disabled: false,
            },
        }
    }

    /// Enables or disables horizontal centering.
    #[must_use]
    pub fn horizontal(mut self, horizontal: bool) -> Self {
        self.props.horizontal = horizontal;
        self
    }

    /// Enables or disables vertical centering.
    #[must_use]
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.props.vertical = vertical;
        self
    }

    /// Sets fixed width for the wrapper container.
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.props.width = Some(width);
        self
    }

    /// Sets fixed height for the wrapper container.
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.props.height = Some(height);
        self
    }

    /// Sets wrapper padding around child content.
    #[must_use]
    pub fn padding(mut self, padding: f32) -> Self {
        self.props.padding = padding;
        self
    }

    /// Sets gap between centered child content.
    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.props.gap = gap;
        self
    }

    /// Sets background color for the wrapper container.
    #[must_use]
    pub fn background(mut self, background: Color) -> Self {
        self.props.background = Some(background);
        self
    }

    /// Sets disabled visual state for the wrapper container.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    /// Resolves options used by the view layer.
    #[must_use]
    pub fn resolve(&self, _theme: &Theme) -> ResolvedAlignCenterWrapper {
        ResolvedAlignCenterWrapper {
            horizontal: self.props.horizontal,
            vertical: self.props.vertical,
            width: self.props.width,
            height: self.props.height,
            padding: self.props.padding,
            gap: self.props.gap,
            background: self.props.background,
            disabled: self.props.disabled,
        }
    }
}

impl Default for AlignCenterWrapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;
    use crate::theme::color::Color;

    #[test]
    fn defaults_center_both_axes() {
        let state = AlignCenterWrapper::new().resolve(&Theme::default_light());
        assert!(state.horizontal);
        assert!(state.vertical);
    }

    #[test]
    fn size_and_padding_are_resolved() {
        let state = AlignCenterWrapper::new()
            .width(120.0)
            .height(80.0)
            .padding(12.0)
            .gap(6.0)
            .resolve(&Theme::default_light());

        assert_eq!(state.width, Some(120.0));
        assert_eq!(state.height, Some(80.0));
        assert_eq!(state.padding, 12.0);
        assert_eq!(state.gap, 6.0);
    }

    #[test]
    fn background_is_stored() {
        let state = AlignCenterWrapper::new()
            .background(Color {
                r: 1,
                g: 2,
                b: 3,
                a: 4,
            })
            .resolve(&Theme::default_light());

        assert_eq!(
            state.background,
            Some(Color {
                r: 1,
                g: 2,
                b: 3,
                a: 4,
            })
        );
    }

    #[test]
    fn disabled_state_is_resolved() {
        let state = AlignCenterWrapper::new()
            .disabled(true)
            .resolve(&Theme::default_light());

        assert!(state.disabled);
    }
}
