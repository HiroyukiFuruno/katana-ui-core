use super::super::canvas::Canvas;
use super::super::dedicated_dod_common::Rect;
use super::super::dedicated_dod_metrics as m;
use super::super::palette::VisualPalette;
use super::super::render_context::ScenarioContext;
use katana_ui_core::render_model::UiPanelProps;

const BAR_THICKNESS: usize = 5;
const SCROLLBAR_MIN_THUMB: usize = 16;

pub(super) fn draw(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    _scenario: ScenarioContext<'_>,
    rect: Rect,
    props: &UiPanelProps,
) {
    if props.vertical_scrollbar_visible {
        draw_vertical_scrollbar(canvas, palette, rect, props);
    }
    if props.horizontal_scrollbar_visible {
        draw_horizontal_scrollbar(canvas, palette, rect, props);
    }
}

fn draw_vertical_scrollbar(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    rect: Rect,
    props: &UiPanelProps,
) {
    let track = Rect::new(
        rect.x + rect.width - m::PX_12,
        rect.y + m::PX_8,
        BAR_THICKNESS,
        rect.height - m::PX_16,
    );
    let thumb_height = thumb_length(track.height, props.viewport_height, props.content_height);
    let thumb_y = thumb_offset(
        track.y,
        track.height,
        thumb_height,
        props.scroll_y,
        props.content_height,
        props.viewport_height,
    );
    canvas.fill_round_rect(
        track.x,
        track.y,
        track.width,
        track.height,
        m::PX_2,
        palette.border,
    );
    canvas.fill_round_rect(
        track.x,
        thumb_y,
        track.width,
        thumb_height,
        m::PX_2,
        palette.accent,
    );
}

fn draw_horizontal_scrollbar(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    rect: Rect,
    props: &UiPanelProps,
) {
    let track = Rect::new(
        rect.x + m::PX_10,
        rect.y + rect.height - m::PX_12,
        rect.width - m::PX_24,
        BAR_THICKNESS,
    );
    let thumb_width = thumb_length(track.width, props.viewport_width, props.content_width);
    let thumb_x = thumb_offset(
        track.x,
        track.width,
        thumb_width,
        props.scroll_x,
        props.content_width,
        props.viewport_width,
    );
    canvas.fill_round_rect(
        track.x,
        track.y,
        track.width,
        track.height,
        m::PX_2,
        palette.border,
    );
    canvas.fill_round_rect(
        thumb_x,
        track.y,
        thumb_width,
        track.height,
        m::PX_2,
        palette.accent,
    );
}

fn thumb_length(track_length: usize, viewport: u32, content: u32) -> usize {
    if content == 0 {
        return track_length;
    }
    let raw = track_length * viewport as usize / content as usize;
    raw.clamp(SCROLLBAR_MIN_THUMB.min(track_length), track_length)
}

fn thumb_offset(
    track_start: usize,
    track_length: usize,
    thumb_length: usize,
    offset: u32,
    content: u32,
    viewport: u32,
) -> usize {
    let max_offset = content.saturating_sub(viewport) as usize;
    if max_offset == 0 {
        return track_start;
    }
    let travel = track_length.saturating_sub(thumb_length);
    track_start + travel * offset as usize / max_offset
}

#[cfg(test)]
mod tests {
    use super::{thumb_length, thumb_offset};

    #[test]
    fn empty_content_and_non_scrollable_content_pin_the_thumb() {
        assert_eq!(40, thumb_length(40, 20, 0));
        assert_eq!(7, thumb_offset(7, 40, 40, 99, 20, 20));
    }
}
