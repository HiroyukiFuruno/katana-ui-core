use std::rc::Rc;

/// Placement of the popover relative to its anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Placement {
    #[default]
    Bottom,
    Top,
    Start,
    End,
}

/// Anchor position and dimensions (in logical pixels).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnchorRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl AnchorRect {
    #[must_use]
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// Anchor reference for popover placement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnchorRef {
    pub rect: AnchorRect,
}

impl AnchorRef {
    #[must_use]
    pub const fn new(rect: AnchorRect) -> Self {
        Self { rect }
    }
}

/// Properties for `Popover`.
#[derive(Clone)]
pub struct PopoverProps {
    pub open: bool,
    pub placement: Placement,
    pub offset: f32,
    pub anchor: Option<AnchorRef>,
    pub children: Option<String>,
    pub on_close: Rc<dyn Fn()>,
    pub dismiss_on_outside_click: bool,
    pub dismiss_on_esc: bool,
}
