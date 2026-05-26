use crate::visual::canvas::Canvas;
use crate::visual::dedicated_dod_common::Rect;
use crate::visual::dedicated_dod_metrics as m;
use crate::visual::palette::VisualPalette;
use crate::visual::render_context::ScenarioContext;
use crate::visual::text::TextRenderer;

pub(super) fn draw(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    clip: Rect,
) {
    canvas.fill_rect(clip.x, clip.y, clip.width, clip.height, palette.surface);
    let active = scenario.screen_state.panel.active_panel;
    let state = scenario.screen_state.panel.child(active);
    for (index, value) in [
        format!("active: {}", active.label()),
        format!("x:{} y:{}", state.scroll_x, state.scroll_y),
        format!(
            "bar: {}",
            if state.scrollbar_visible { "on" } else { "off" }
        ),
        "clip: local".to_string(),
    ]
    .into_iter()
    .enumerate()
    {
        let row_y = clip.y + m::PX_8 + index * m::PX_24;
        canvas.fill_rect(
            clip.x + m::PX_8,
            row_y,
            clip.width - m::PX_18,
            m::PX_18,
            palette.panel,
        );
        text.draw(
            canvas,
            &value,
            clip.x + m::PX_14,
            row_y + m::PX_4,
            m::FONT_8,
            palette.text,
        );
    }
}
