use crate::layout::popover::{AnchorRect, Placement};
use floem::IntoView;
use floem::View;
use floem::peniko::Color;
use floem::views::{Decorators, container, label};

const TOOLTIP_ARROW_SIZE: f32 = 8.0;
const TOOLTIP_ARROW_INSET: f32 = 6.0;
const TOOLTIP_ARROW_ROTATION_DEG: f32 = 45.0;
const CENTER_RATE: f32 = 0.5;

pub(super) struct TooltipArrowConfig {
    pub(super) tooltip_bg: Color,
    pub(super) tooltip_max_width: f32,
    pub(super) tooltip_height: f32,
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) placement: Placement,
    pub(super) anchor: AnchorRect,
    pub(super) show_arrow: bool,
}

pub(super) fn arrow_view(config: TooltipArrowConfig) -> Box<dyn View> {
    let Some((x, y)) = arrow_origin(&config) else {
        return container(label(|| ""))
            .style(|style| style.size(0.0, 0.0))
            .into_any();
    };
    let tooltip_bg = config.tooltip_bg;
    container(label(|| ""))
        .style(move |style| {
            style
                .absolute()
                .inset_left(x)
                .inset_top(y)
                .width(TOOLTIP_ARROW_SIZE)
                .height(TOOLTIP_ARROW_SIZE)
                .background(tooltip_bg)
                .rotate(TOOLTIP_ARROW_ROTATION_DEG)
        })
        .into_any()
}

fn arrow_origin(config: &TooltipArrowConfig) -> Option<(f32, f32)> {
    if !config.show_arrow {
        return None;
    }

    let anchor_center_x = config.anchor.x + (config.anchor.width * CENTER_RATE);
    let anchor_center_y = config.anchor.y + (config.anchor.height * CENTER_RATE);
    let min_x = config.x + TOOLTIP_ARROW_INSET;
    let max_x = config.x + config.tooltip_max_width - TOOLTIP_ARROW_INSET - TOOLTIP_ARROW_SIZE;
    let min_y = config.y + TOOLTIP_ARROW_INSET;
    let max_y = config.y + config.tooltip_height - TOOLTIP_ARROW_INSET - TOOLTIP_ARROW_SIZE;

    match config.placement {
        Placement::Bottom => Some((
            (anchor_center_x - (TOOLTIP_ARROW_SIZE * CENTER_RATE)).clamp(min_x, max_x),
            config.y - (TOOLTIP_ARROW_SIZE * CENTER_RATE),
        )),
        Placement::Top => Some((
            (anchor_center_x - (TOOLTIP_ARROW_SIZE * CENTER_RATE)).clamp(min_x, max_x),
            config.y + config.tooltip_height - (TOOLTIP_ARROW_SIZE * CENTER_RATE),
        )),
        Placement::Right => Some((
            config.x - (TOOLTIP_ARROW_SIZE * CENTER_RATE),
            (anchor_center_y - (TOOLTIP_ARROW_SIZE * CENTER_RATE)).clamp(min_y, max_y),
        )),
        Placement::Left => Some((
            config.x + config.tooltip_max_width - (TOOLTIP_ARROW_SIZE * CENTER_RATE),
            (anchor_center_y - (TOOLTIP_ARROW_SIZE * CENTER_RATE)).clamp(min_y, max_y),
        )),
    }
}
