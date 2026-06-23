#[path = "body_content.rs"]
mod body_content;

use super::super::canvas::Canvas;
use super::super::dedicated_dod_common::Rect;
use super::super::dedicated_dod_metrics as m;
use super::super::palette::VisualPalette;
use super::super::render_context::ScenarioContext;
use super::super::text::TextRenderer;
use super::model::{DETAILS_SLOT, NAV_SLOT, PREVIEW_SLOT, PanelSlot};

pub(super) fn draw(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    draw_slot_content(canvas, text, palette, scenario, x, y, NAV_SLOT);
    draw_slot_content(canvas, text, palette, scenario, x, y, PREVIEW_SLOT);
    draw_slot_content(canvas, text, palette, scenario, x, y, DETAILS_SLOT);
}

fn draw_slot_content(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
    slot: PanelSlot,
) {
    let rect = slot.rect(x, y);
    let clip = Rect::new(
        rect.x + m::PX_8,
        rect.y + m::PX_28,
        rect.width - m::PX_24,
        rect.height - m::PX_48,
    );
    canvas.with_clip(clip.x, clip.y, clip.width, clip.height, |canvas| {
        body_content::draw(canvas, text, palette, scenario, slot.key, clip);
    });
}
