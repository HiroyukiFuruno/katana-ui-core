use super::{INNER_STROKE_INSET, INNER_STROKE_REDUCTION, SHADOW_OFFSET};
use crate::visual::button_options::StorybookButtonZIndex;
use crate::visual::canvas::Canvas;
use crate::visual::dedicated_dod_common::Rect;
use crate::visual::dedicated_dod_metrics as m;
use crate::visual::palette::VisualPalette;
use crate::visual::render_context::ScenarioContext;

pub(super) fn draw_setting_outline(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
) {
    if scenario.screen_state.has_settings_override() && scenario.screen_state.button_options.border
    {
        canvas.stroke_rect(
            rect.x + INNER_STROKE_INSET,
            rect.y + INNER_STROKE_INSET,
            rect.width - INNER_STROKE_REDUCTION,
            rect.height - INNER_STROKE_REDUCTION,
            palette.accent,
        );
    }
}

pub(super) fn draw_invisible_placeholder(canvas: &mut Canvas, palette: &VisualPalette, rect: Rect) {
    canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, palette.muted);
}

pub(super) fn draw_z_index_shadow(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    rect: Rect,
    z_index: StorybookButtonZIndex,
) {
    if matches!(z_index, StorybookButtonZIndex::Auto) {
        return;
    }
    let offset = match z_index {
        StorybookButtonZIndex::Raised => SHADOW_OFFSET,
        StorybookButtonZIndex::Overlay => SHADOW_OFFSET * m::PX_2,
        StorybookButtonZIndex::Auto => m::PX_0,
    };
    canvas.fill_rect(
        rect.x + offset,
        rect.y + offset,
        rect.width,
        rect.height,
        palette.code_background,
    );
}
