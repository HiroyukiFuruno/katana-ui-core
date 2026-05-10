mod ops;
mod types;
mod view;

pub use ops::FocusTransition;
pub use types::{ModalProps, ModalSize};

use crate::theme::Theme;
use crate::theme::color::Color;
use std::rc::Rc;
use view::{overlay_view, title_color};

fn noop_close() {}
fn noop_focus_return() {}

/// Resolved visual and behavioral properties for `Modal`.
#[derive(Clone)]
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
    pub content_gap: f32,
    pub footer_gap: f32,
    pub title_font_size: f32,
    pub title_color: Color,
    pub children: Option<String>,
    pub footer: Option<String>,
    pub on_close: Rc<dyn Fn()>,
    pub on_focus_return: Rc<dyn Fn()>,
    pub trap_focus: bool,
    pub focus_on_open: FocusTransition,
    pub focus_on_close: FocusTransition,
}

impl ResolvedModal {
    /// Returns whether the modal should be closed by backdrop click.
    #[must_use]
    pub fn should_close_with_backdrop(&self) -> bool {
        self.open && self.dismiss_on_backdrop
    }

    /// Returns whether the modal should be closed by Esc key.
    #[must_use]
    pub fn should_close_with_esc(&self) -> bool {
        self.open && self.dismiss_on_esc
    }

    /// Tries to close by backdrop click and returns whether close was executed.
    pub fn close_with_backdrop(&self) -> bool {
        if self.should_close_with_backdrop() {
            (self.on_close)();
            (self.on_focus_return)();
            true
        } else {
            false
        }
    }

    /// Tries to close by Esc key and returns whether close was executed.
    pub fn close_with_esc(&self) -> bool {
        if self.should_close_with_esc() {
            (self.on_close)();
            (self.on_focus_return)();
            true
        } else {
            false
        }
    }

    /// Indicates whether focus can be returned to the trigger after close.
    #[must_use]
    pub fn focus_returns_to_trigger(&self) -> bool {
        !self.open
    }
}

/// Builder for the Modal layout widget.
#[derive(Clone)]
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
                children: None,
                footer: None,
                on_close: Rc::new(noop_close),
                on_focus_return: Rc::new(noop_focus_return),
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
        let dialog_view = view::dialog_view(theme, &self.props.size);
        ResolvedModal {
            open: self.props.open,
            title: self.props.title.clone(),
            dismiss_on_backdrop: ops::should_dismiss_on_backdrop(&self.props),
            dismiss_on_esc: ops::should_dismiss_on_esc(&self.props),
            overlay_color: overlay.background,
            dialog_bg: dialog_view.background,
            dialog_border: dialog_view.border_color,
            dialog_width: dialog_view.width,
            corner_radius: dialog_view.corner_radius,
            padding: dialog_view.padding,
            content_gap: dialog_view.content_gap,
            footer_gap: dialog_view.footer_gap,
            title_font_size: dialog_view.title_font_size,
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

impl Default for Modal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
