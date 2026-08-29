use super::command_chrome_artifact::{CommandChromePaintOperation, CommandChromePaintPlan};
use super::command_chrome_paint::{
    layered_fill, plan_surface_bounds, texture, texture_from_raster, union_bounds,
};
use super::command_chrome_types::{
    CommandChromePaintStyle, EguiCommandChromeDrawLayer, RenderedRaster,
};
use katana_ui_core::render_model::UiRect;

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
    let mut operations = vec![layered_fill(
        EguiCommandChromeDrawLayer::PanelFill,
        panel_bounds,
        panel_bounds,
        style.action_rgba,
    )];
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
