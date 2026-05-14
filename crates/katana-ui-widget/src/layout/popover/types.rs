use crate::theme::color::Color;
use floem::View;
use std::rc::Rc;

pub type PopoverChildren = Rc<dyn Fn() -> Box<dyn View>>;

/// Placement of the popover relative to its anchor.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Placement {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
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

/// Resolved layout for popover overlay placement.
#[derive(Debug, Clone, Copy)]
pub struct PopoverOverlay {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub placement: Placement,
    pub popover_bg: Color,
    pub popover_border: Color,
    pub shadow_color: Color,
    pub corner_radius: f32,
}

/// Properties for `Popover`.
#[derive(Clone)]
pub struct PopoverProps {
    pub open: bool,
    pub placement: Placement,
    pub offset: f32,
    pub width: f32,
    pub anchor: Option<AnchorRef>,
    pub children: Option<PopoverChildren>,
    pub on_close: Rc<dyn Fn()>,
    pub on_focus_in: Rc<dyn Fn()>,
    pub on_focus_out: Rc<dyn Fn()>,
    pub dismiss_on_outside_click: bool,
    pub dismiss_on_esc: bool,
}

/// Resolved visual and behavioral properties for `Popover`.
#[derive(Clone)]
pub struct ResolvedPopover {
    pub open: bool,
    pub placement: Placement,
    pub offset: f32,
    pub width: f32,
    pub anchor: Option<AnchorRect>,
    pub children: Option<PopoverChildren>,
    pub on_close: Rc<dyn Fn()>,
    pub on_focus_in: Rc<dyn Fn()>,
    pub on_focus_out: Rc<dyn Fn()>,
    pub dismiss_on_outside_click: bool,
    pub dismiss_on_esc: bool,
    pub popover_bg: Color,
    pub popover_border: Color,
    pub shadow_color: Color,
    pub corner_radius: f32,
}

/// Builder for the Popover layout widget.
#[derive(Clone)]
pub struct Popover {
    pub(crate) props: PopoverProps,
}
