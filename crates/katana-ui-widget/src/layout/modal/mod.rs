mod native_window;
mod ops;
mod overlay_dialog;
mod placement;
mod resolved;
mod types;
mod view;

pub use ops::FocusTransition;
pub use overlay_dialog::OverlayDialog;
pub use resolved::ResolvedModal;
pub use types::{
    ModalParentInteraction, ModalProps, ModalSize, ModalWindowPlacement, OverlayDialogProps,
};

use crate::theme::Theme;
use floem::peniko::kurbo::Point;
use floem::window::WindowId;
use std::rc::Rc;

/// Builder for the Modal layout widget.
#[derive(Clone)]
pub struct Modal {
    pub(super) props: ModalProps,
}

impl Modal {
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
    pub fn window_placement(mut self, placement: ModalWindowPlacement) -> Self {
        self.props.window_placement = placement;
        self
    }

    #[must_use]
    pub fn window_position(self, position: Point) -> Self {
        self.window_placement(ModalWindowPlacement::At(position))
    }

    #[must_use]
    pub fn same_display_as(self, parent_window_id: WindowId) -> Self {
        self.window_placement(ModalWindowPlacement::SameDisplayAs(parent_window_id))
    }

    #[must_use]
    pub fn parent_interaction(mut self, parent_interaction: ModalParentInteraction) -> Self {
        self.props.parent_interaction = parent_interaction;
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
    pub fn on_open(mut self, on_open: impl Fn() + 'static) -> Self {
        self.props.on_open = Rc::new(on_open);
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
    pub fn as_overlay_dialog(&self) -> OverlayDialog {
        OverlayDialog {
            props: self.props.clone(),
        }
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedModal {
        self.as_overlay_dialog().resolve(theme)
    }
}

impl Default for Modal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
