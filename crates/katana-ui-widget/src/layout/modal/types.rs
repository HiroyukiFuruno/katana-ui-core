use std::{fmt, rc::Rc};

use floem::peniko::kurbo::Point;
use floem::window::WindowId;

/// Modal dialog size.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ModalSize {
    Sm,
    #[default]
    Md,
    Lg,
    Custom(f32),
}

/// Parent window interaction policy for `Modal`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ModalParentInteraction {
    #[default]
    Block,
    Allow,
}

/// Initial native-window placement for `Modal`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ModalWindowPlacement {
    #[default]
    SystemDefault,
    SameDisplayAs(WindowId),
    At(Point),
}

/// Error returned before requesting a native Modal window.
#[derive(Debug, Clone, PartialEq)]
pub enum ModalOpenError {
    SameDisplayPlacementUnavailable,
    InvalidWindowPosition { x: f64, y: f64 },
}

impl fmt::Display for ModalOpenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SameDisplayPlacementUnavailable => {
                write!(f, "same-display modal placement is unavailable")
            }
            Self::InvalidWindowPosition { x, y } => {
                write!(f, "modal window position must be finite: x={x}, y={y}")
            }
        }
    }
}

impl std::error::Error for ModalOpenError {}

/// Properties for `Modal`.
#[derive(Clone)]
pub struct ModalProps {
    pub open: bool,
    pub title: Option<String>,
    pub size: ModalSize,
    pub window_placement: ModalWindowPlacement,
    pub parent_interaction: ModalParentInteraction,
    pub dismiss_on_backdrop: bool,
    pub dismiss_on_esc: bool,
    pub children: Option<String>,
    pub footer: Option<String>,
    pub on_open: Rc<dyn Fn()>,
    pub on_close: Rc<dyn Fn()>,
    pub on_focus_return: Rc<dyn Fn()>,
}

/// Properties for `OverlayDialog`.
pub type OverlayDialogProps = ModalProps;

fn noop_close() {}
fn noop_open() {}
fn noop_focus_return() {}

impl Default for ModalProps {
    fn default() -> Self {
        Self {
            open: false,
            title: None,
            size: ModalSize::default(),
            window_placement: ModalWindowPlacement::default(),
            parent_interaction: ModalParentInteraction::default(),
            dismiss_on_backdrop: true,
            dismiss_on_esc: true,
            children: None,
            footer: None,
            on_open: Rc::new(noop_open),
            on_close: Rc::new(noop_close),
            on_focus_return: Rc::new(noop_focus_return),
        }
    }
}
