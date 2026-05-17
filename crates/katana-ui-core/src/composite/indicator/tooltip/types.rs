use crate::layout::popover::Placement;
use crate::theme::color::Color;

/// Tooltip が対応する表示方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipPlacement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl TooltipPlacement {
    #[must_use]
    pub const fn as_popover_placement(self) -> Placement {
        match self {
            Self::Top => Placement::Top,
            Self::Bottom => Placement::Bottom,
            Self::Left => Placement::Left,
            Self::Right => Placement::Right,
        }
    }

    pub(crate) const fn from_popover_placement(placement: Placement) -> Self {
        match placement {
            Placement::Bottom => Self::Bottom,
            Placement::Left => Self::Left,
            Placement::Right => Self::Right,
            _ => Self::Top,
        }
    }
}

/// Properties for `Tooltip`.
#[derive(Debug, Clone)]
pub struct TooltipProps {
    pub label: String,
    pub placement: TooltipPlacement,
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
