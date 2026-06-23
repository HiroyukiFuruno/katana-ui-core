#[path = "body_details.rs"]
mod body_details;
#[path = "body_preview.rs"]
mod body_preview;

use crate::visual::canvas::Canvas;
use crate::visual::dedicated_dod_common::Rect;
use crate::visual::dedicated_dod_metrics as m;
use crate::visual::palette::VisualPalette;
use crate::visual::panel_screen_state::PanelChildKey;
use crate::visual::render_context::ScenarioContext;
use crate::visual::text::TextRenderer;

const NAV_ROW_COUNT: usize = 9;
const NAV_SELECTED_MODULO: usize = 3;
const NAV_SELECTED_REMAINDER: usize = 1;

pub(super) fn draw(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    panel: PanelChildKey,
    clip: Rect,
) {
    match panel {
        PanelChildKey::Navigation => draw_navigation(canvas, palette, scenario, clip),
        PanelChildKey::Preview => body_preview::draw(canvas, text, palette, scenario, clip),
        PanelChildKey::Details => body_details::draw(canvas, text, palette, scenario, clip),
    }
}

fn draw_navigation(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    clip: Rect,
) {
    canvas.fill_rect(clip.x, clip.y, clip.width, clip.height, palette.surface);
    let state = scenario.screen_state.panel.child(PanelChildKey::Navigation);
    let offset = (state.scroll_y as usize) % m::PX_72;
    for index in 0..NAV_ROW_COUNT {
        let content_y = clip.y + m::PX_6 + index * m::PX_24;
        let row_y = content_y.saturating_sub(offset);
        if row_y + m::PX_14 < clip.y || row_y > clip.y + clip.height {
            continue;
        }
        let fill = if index % NAV_SELECTED_MODULO == NAV_SELECTED_REMAINDER {
            palette.selection
        } else {
            palette.panel
        };
        canvas.fill_rect(
            clip.x + m::PX_8,
            row_y,
            clip.width - m::PX_22,
            m::PX_14,
            fill,
        );
        canvas.fill_rect(
            clip.x + m::PX_14,
            row_y + m::PX_4,
            m::PX_6,
            m::PX_6,
            palette.accent,
        );
    }
}
