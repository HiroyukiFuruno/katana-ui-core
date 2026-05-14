pub use crate::layout::popover::{FreePlacement, Placement};
use crate::theme::color::Color;

/// Properties for `Tooltip`.
#[derive(Debug, Clone)]
pub struct TooltipProps {
    pub label: String,
    pub placement: Placement,
    pub delay_ms: u32,
    pub max_width: f32,
    pub dismiss_on_pointer_leave: bool,
    pub dismiss_on_focus_loss: bool,
    pub show_arrow: bool,
    pub visible: bool,
}

#[derive(Debug, Clone)]
pub struct ResolvedTooltip {
    pub label: String,
    pub placement: Placement,
    pub delay_ms: u32,
    pub max_width: f32,
    pub font_size: f32,
    pub pad_v: f32,
    pub pad_h: f32,
    pub bg_color: Color,
    pub text_color: Color,
    pub dismiss_on_pointer_leave: bool,
    pub dismiss_on_focus_loss: bool,
    pub show_arrow: bool,
    pub visible: bool,
}

#[derive(Debug, Clone)]
pub struct Tooltip {
    pub(super) props: TooltipProps,
}
