use std::rc::Rc;

use crate::theme::color::Color;

use super::ops::{self, DismissReason, FocusTransition};

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
            self.return_focus_after_close(DismissReason::Backdrop);
            true
        } else {
            false
        }
    }

    /// Tries to close by Esc key and returns whether close was executed.
    pub fn close_with_esc(&self) -> bool {
        if self.should_close_with_esc() {
            (self.on_close)();
            self.return_focus_after_close(DismissReason::Escape);
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

    fn return_focus_after_close(&self, reason: DismissReason) {
        let props = super::types::ModalProps {
            open: self.open,
            title: self.title.clone(),
            size: super::types::ModalSize::Md,
            window_placement: super::types::ModalWindowPlacement::SystemDefault,
            parent_interaction: super::types::ModalParentInteraction::Block,
            dismiss_on_backdrop: self.dismiss_on_backdrop,
            dismiss_on_esc: self.dismiss_on_esc,
            children: self.children.clone(),
            footer: self.footer.clone(),
            on_open: Rc::new(|| {}),
            on_close: Rc::clone(&self.on_close),
            on_focus_return: Rc::clone(&self.on_focus_return),
        };
        if ops::should_return_focus_after_close(&props, reason) {
            (self.on_focus_return)();
        }
    }
}
