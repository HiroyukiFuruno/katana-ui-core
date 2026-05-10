/// Position of the tooltip relative to its anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Placement {
    #[default]
    Top,
    Bottom,
    Start,
    End,
}

/// Properties for `Tooltip`.
#[derive(Debug, Clone)]
pub struct TooltipProps {
    pub label: String,
    pub placement: Placement,
    pub delay_ms: u32,
    pub max_width: f32,
}
