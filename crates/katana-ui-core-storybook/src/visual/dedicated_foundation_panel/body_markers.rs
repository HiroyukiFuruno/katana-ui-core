use crate::visual::canvas::Canvas;
use crate::visual::dedicated_dod_metrics as m;
use crate::visual::palette::VisualPalette;
use crate::visual::render_context::ScenarioContext;
use crate::visual::text::TextRenderer;

use super::super::model::{DETAILS_SLOT, PREVIEW_SLOT, component_scrollbars_visible};

pub(super) fn draw_vertical(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    text.draw(
        canvas,
        "wheel y changes preview_y",
        x + PREVIEW_SLOT.x + m::PX_10,
        y + PREVIEW_SLOT.y + m::PX_78,
        m::FONT_7,
        palette.muted,
    );
}

pub(super) fn draw_horizontal(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    text.draw(
        canvas,
        "wheel x changes preview_x",
        x + PREVIEW_SLOT.x + m::PX_10,
        y + PREVIEW_SLOT.y + m::PX_78,
        m::FONT_7,
        palette.muted,
    );
}

pub(super) fn draw_scrollbar(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let label = if component_scrollbars_visible(scenario, scenario.screen_state.panel.active_panel)
    {
        "panel scrollbar visible"
    } else {
        "panel scrollbar hidden"
    };
    text.draw(
        canvas,
        label,
        x + DETAILS_SLOT.x + m::PX_10,
        y + DETAILS_SLOT.y + m::PX_54,
        m::FONT_7,
        palette.muted,
    );
}

pub(super) fn draw_nested(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    text.draw(
        canvas,
        "inner scroll moves child panel thumbs",
        x + PREVIEW_SLOT.x + m::PX_10,
        y + PREVIEW_SLOT.y + m::PX_78,
        m::FONT_7,
        palette.muted,
    );
    text.draw(
        canvas,
        &format!(
            "x{} y{}",
            scenario
                .screen_state
                .panel
                .child(scenario.screen_state.panel.active_panel)
                .scroll_x,
            scenario
                .screen_state
                .panel
                .child(scenario.screen_state.panel.active_panel)
                .scroll_y
        ),
        x + DETAILS_SLOT.x + m::PX_10,
        y + DETAILS_SLOT.y + m::PX_54,
        m::FONT_7,
        palette.muted,
    );
}
