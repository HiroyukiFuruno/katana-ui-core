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

/// Properties for `Popover`.
#[derive(Debug, Clone)]
pub struct PopoverProps {
    pub open: bool,
    pub placement: Placement,
    pub offset: f32,
    pub dismiss_on_outside_click: bool,
    pub dismiss_on_esc: bool,
}
