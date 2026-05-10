mod ops;
mod types;
mod view;

pub use types::{ModalProps, ModalSize};

use crate::theme::Theme;
use crate::theme::color::Color;
use view::{
    corner_radius, dialog_bg, dialog_border, dialog_padding, dialog_width, overlay_color,
    title_color, title_font_size,
};

/// Resolved visual properties for `Modal`.
#[derive(Debug, Clone)]
pub struct ResolvedModal {
    pub open: bool,
    pub title: Option<String>,
    pub dismiss_on_backdrop: bool,
    pub dismiss_on_esc: bool,
    pub overlay_color: Color,
    pub dialog_bg: Color,
    pub dialog_border: Color,
    pub dialog_width: f32,
    pub corner_radius: f32,
    pub padding: f32,
    pub title_font_size: f32,
    pub title_color: Color,
}

/// Builder for the Modal layout widget.
#[derive(Debug, Clone)]
pub struct Modal {
    props: ModalProps,
}

impl Modal {
    #[must_use]
    pub fn new() -> Self {
        Self {
            props: ModalProps {
                open: false,
                title: None,
                size: ModalSize::default(),
                dismiss_on_backdrop: true,
                dismiss_on_esc: true,
            },
        }
    }

    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.props.open = open;
        self
    }

    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.props.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn size(mut self, size: ModalSize) -> Self {
        self.props.size = size;
        self
    }

    #[must_use]
    pub fn dismiss_on_backdrop(mut self, v: bool) -> Self {
        self.props.dismiss_on_backdrop = v;
        self
    }

    #[must_use]
    pub fn dismiss_on_esc(mut self, v: bool) -> Self {
        self.props.dismiss_on_esc = v;
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedModal {
        ResolvedModal {
            open: self.props.open,
            title: self.props.title.clone(),
            dismiss_on_backdrop: ops::should_dismiss_on_backdrop(&self.props),
            dismiss_on_esc: ops::should_dismiss_on_esc(&self.props),
            overlay_color: overlay_color(theme),
            dialog_bg: dialog_bg(theme),
            dialog_border: dialog_border(theme),
            dialog_width: dialog_width(&self.props.size),
            corner_radius: corner_radius(),
            padding: dialog_padding(),
            title_font_size: title_font_size(),
            title_color: title_color(theme),
        }
    }
}

impl Default for Modal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn dismiss_on_backdrop_default_true() {
        let theme = Theme::default_light();
        let r = Modal::new().open(true).resolve(&theme);
        assert!(r.dismiss_on_backdrop);
    }

    #[test]
    fn dismiss_on_backdrop_can_be_disabled() {
        let theme = Theme::default_light();
        let r = Modal::new().dismiss_on_backdrop(false).resolve(&theme);
        assert!(!r.dismiss_on_backdrop);
    }

    #[test]
    fn dismiss_on_esc_default_true() {
        let theme = Theme::default_light();
        let r = Modal::new().resolve(&theme);
        assert!(r.dismiss_on_esc);
    }

    #[test]
    fn size_sm_width() {
        let theme = Theme::default_light();
        let r = Modal::new().size(ModalSize::Sm).resolve(&theme);
        assert!((r.dialog_width - 320.0).abs() < f32::EPSILON);
    }

    #[test]
    fn size_lg_width() {
        let theme = Theme::default_light();
        let r = Modal::new().size(ModalSize::Lg).resolve(&theme);
        assert!((r.dialog_width - 640.0).abs() < f32::EPSILON);
    }

    #[test]
    fn size_custom_width() {
        let theme = Theme::default_light();
        let r = Modal::new().size(ModalSize::Custom(400.0)).resolve(&theme);
        assert!((r.dialog_width - 400.0).abs() < f32::EPSILON);
    }

    #[test]
    fn title_stored_correctly() {
        let theme = Theme::default_light();
        let r = Modal::new().title("Confirm").resolve(&theme);
        assert_eq!(r.title.as_deref(), Some("Confirm"));
    }

    #[test]
    fn open_flag_propagated() {
        let theme = Theme::default_light();
        let r_open = Modal::new().open(true).resolve(&theme);
        let r_closed = Modal::new().open(false).resolve(&theme);
        assert!(r_open.open);
        assert!(!r_closed.open);
    }
}
