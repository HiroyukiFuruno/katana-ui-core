use super::command_chrome_artifact::{
    CommandChromePaintOperation, CommandChromePaintOperationKind, CommandChromePaintPlan,
};
use super::command_chrome_floating::{
    FLOATING_PANEL_BORDER_PX, FLOATING_PANEL_PADDING_PX, FLOATING_PANEL_RADIUS_PX,
};
use super::command_chrome_paint::{
    layered_fill, plan_surface_bounds, texture, texture_from_raster, union_bounds,
};
use super::command_chrome_types::{
    CommandChromePaintStyle, EguiCommandChromeDrawLayer, RenderedRaster,
};
use crate::render_model::{RGBA_CHANNEL_COUNT, UiRect};

const FLOATING_PANEL_BORDER_HIGHLIGHT_DELTA: u8 = 32;
const RGBA_ALPHA_CHANNEL_INDEX: usize = 3;

pub(super) struct TooltipPaintSource {
    bounds: UiRect,
    text_bounds: UiRect,
    rendered: RenderedRaster,
}

impl TooltipPaintSource {
    pub(super) fn new(bounds: UiRect, text_bounds: UiRect, rendered: RenderedRaster) -> Self {
        Self {
            bounds,
            text_bounds,
            rendered,
        }
    }

    pub(super) const fn bounds(&self) -> UiRect {
        self.bounds
    }

    pub(super) fn raster_identity(&self) -> &str {
        &self.rendered.identity
    }
}

pub(super) fn build_floating_paint_plan(
    panel_bounds: UiRect,
    toolbar_plan: &CommandChromePaintPlan,
    tooltip: Option<&TooltipPaintSource>,
    style: &CommandChromePaintStyle,
) -> CommandChromePaintPlan {
    let inner_bounds = UiRect::new(
        panel_bounds
            .x
            .saturating_add((FLOATING_PANEL_PADDING_PX + FLOATING_PANEL_BORDER_PX) as i32),
        panel_bounds
            .y
            .saturating_add((FLOATING_PANEL_PADDING_PX + FLOATING_PANEL_BORDER_PX) as i32),
        panel_bounds
            .width
            .saturating_sub((FLOATING_PANEL_PADDING_PX + FLOATING_PANEL_BORDER_PX) * 2),
        panel_bounds
            .height
            .saturating_sub((FLOATING_PANEL_PADDING_PX + FLOATING_PANEL_BORDER_PX) * 2),
    );
    let border_color = [
        style.action_rgba[0].saturating_add(FLOATING_PANEL_BORDER_HIGHLIGHT_DELTA),
        style.action_rgba[1].saturating_add(FLOATING_PANEL_BORDER_HIGHLIGHT_DELTA),
        style.action_rgba[2].saturating_add(FLOATING_PANEL_BORDER_HIGHLIGHT_DELTA),
        style.action_rgba[RGBA_ALPHA_CHANNEL_INDEX],
    ];
    let mut operations = vec![
        rounded_fill(
            EguiCommandChromeDrawLayer::PanelBorder,
            panel_bounds,
            panel_bounds,
            border_color,
            FLOATING_PANEL_RADIUS_PX,
        ),
        rounded_fill(
            EguiCommandChromeDrawLayer::PanelFill,
            panel_bounds,
            inner_bounds,
            style.action_rgba,
            FLOATING_PANEL_RADIUS_PX.saturating_sub(FLOATING_PANEL_BORDER_PX),
        ),
    ];
    operations.extend(toolbar_plan.operations.iter().cloned());
    if let Some(tooltip) = tooltip {
        append_tooltip_operations(&mut operations, tooltip, style);
    }
    let initial_bounds = union_bounds(panel_bounds, toolbar_plan.surface_bounds);
    CommandChromePaintPlan {
        surface_bounds: plan_surface_bounds(initial_bounds, &operations),
        operations,
    }
}

fn rounded_fill(
    layer: EguiCommandChromeDrawLayer,
    clip_bounds: UiRect,
    bounds: UiRect,
    color_rgba: [u8; RGBA_CHANNEL_COUNT],
    radius_px: u32,
) -> CommandChromePaintOperation {
    CommandChromePaintOperation {
        layer,
        clip_bounds,
        kind: CommandChromePaintOperationKind::RoundedFill {
            bounds,
            color_rgba,
            radius_px,
        },
    }
}

fn append_tooltip_operations(
    operations: &mut Vec<CommandChromePaintOperation>,
    tooltip: &TooltipPaintSource,
    style: &CommandChromePaintStyle,
) {
    operations.push(layered_fill(
        EguiCommandChromeDrawLayer::TooltipFill,
        tooltip.bounds,
        tooltip.bounds,
        style.hovered_action_rgba,
    ));
    operations.push(texture(
        EguiCommandChromeDrawLayer::TooltipTexture,
        tooltip.bounds,
        tooltip.text_bounds,
        texture_from_raster(&tooltip.rendered),
    ));
}
