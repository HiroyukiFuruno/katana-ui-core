use std::rc::Rc;

use crate::theme::Theme;

use super::ops;
use super::resolved::ResolvedModal;
use super::types::{ModalProps, ModalSize, OverlayDialogProps};
use super::view::{dialog_view, overlay_view, title_color};

/// Builder for in-window overlay dialog, extracted from legacy modal behavior.
#[derive(Clone)]
pub struct OverlayDialog {
    pub(crate) props: OverlayDialogProps,
}

impl OverlayDialog {
    #[must_use]
    pub fn new() -> Self {
        Self {
            props: ModalProps::default(),
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
    pub fn children(mut self, children: impl Into<String>) -> Self {
        self.props.children = Some(children.into());
        self
    }

    #[must_use]
    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.props.footer = Some(footer.into());
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
    pub fn on_close(mut self, on_close: impl Fn() + 'static) -> Self {
        self.props.on_close = Rc::new(on_close);
        self
    }

    #[must_use]
    pub fn on_focus_return(mut self, on_focus_return: impl Fn() + 'static) -> Self {
        self.props.on_focus_return = Rc::new(on_focus_return);
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedModal {
        let overlay = overlay_view(theme);
        let dialog = dialog_view(theme, &self.props.size);
        ResolvedModal {
            open: self.props.open,
            title: self.props.title.clone(),
            dismiss_on_backdrop: ops::should_dismiss_on_backdrop(&self.props),
            dismiss_on_esc: ops::should_dismiss_on_esc(&self.props),
            overlay_color: overlay.background,
            dialog_bg: dialog.background,
            dialog_border: dialog.border_color,
            dialog_width: dialog.width,
            corner_radius: dialog.corner_radius,
            padding: dialog.padding,
            content_gap: dialog.content_gap,
            footer_gap: dialog.footer_gap,
            title_font_size: dialog.title_font_size,
            title_color: title_color(theme),
            children: self.props.children.clone(),
            footer: self.props.footer.clone(),
            on_close: Rc::clone(&self.props.on_close),
            on_focus_return: Rc::clone(&self.props.on_focus_return),
            trap_focus: ops::should_trap_focus(&self.props),
            focus_on_open: ops::focus_transition(false, self.props.open),
            focus_on_close: ops::focus_transition(self.props.open, false),
        }
    }
}

impl Default for OverlayDialog {
    fn default() -> Self {
        Self::new()
    }
}
