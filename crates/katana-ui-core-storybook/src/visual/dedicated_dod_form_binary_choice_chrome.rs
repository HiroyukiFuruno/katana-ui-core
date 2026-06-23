use super::canvas::Canvas;
use super::dedicated_dod_form_binary_choice_layout::{CHOICE_LABEL_X, CHOICE_ROW_HEIGHT};
use super::dedicated_dod_metrics as m;
use super::layout_metrics::LayoutRect;
use super::palette::VisualPalette;
use super::text::{TextRenderer, TextVerticalBox};

const CHOICE_ROW_RADIUS: usize = 6;
const CONTROL_RADIUS: usize = 5;
const STATUS_RADIUS: usize = 5;
const CHROME_BORDER_INSET: usize = 1;

pub(super) fn draw_choice_row_with_border(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    row: LayoutRect,
    label: &str,
    disabled: bool,
    border: u32,
) {
    let text_color = if disabled {
        palette.muted
    } else {
        palette.text
    };
    draw_choice_row_background(canvas, palette, row, border);
    text.draw_centered(
        canvas,
        label,
        row.x + CHOICE_LABEL_X,
        TextVerticalBox::new(row.y, CHOICE_ROW_HEIGHT as f32),
        m::FONT_13,
        text_color,
    );
}

pub(super) fn choice_row_border(
    palette: &VisualPalette,
    disabled: bool,
    hovered: bool,
    active: bool,
) -> u32 {
    if active {
        return palette.accent;
    }
    if disabled {
        return palette.muted;
    }
    if hovered {
        return palette.hover_border;
    }
    palette.border
}

pub(super) fn draw_choice_row_background(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    row: LayoutRect,
    border: u32,
) {
    draw_rounded_panel(canvas, row, border, palette.surface, CHOICE_ROW_RADIUS);
}

pub(super) fn draw_control_background(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    rect: LayoutRect,
) {
    draw_rounded_panel(
        canvas,
        rect,
        palette.border,
        palette.surface,
        CONTROL_RADIUS,
    );
}

pub(super) fn draw_status_background(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    rect: LayoutRect,
) {
    draw_rounded_panel(canvas, rect, palette.border, palette.panel, STATUS_RADIUS);
}

fn draw_rounded_panel(
    canvas: &mut Canvas,
    rect: LayoutRect,
    border: u32,
    fill: u32,
    radius: usize,
) {
    canvas.fill_round_rect(rect.x, rect.y, rect.width, rect.height, radius, border);
    canvas.fill_round_rect(
        rect.x + CHROME_BORDER_INSET,
        rect.y + CHROME_BORDER_INSET,
        rect.width.saturating_sub(CHROME_BORDER_INSET * 2),
        rect.height.saturating_sub(CHROME_BORDER_INSET * 2),
        radius.saturating_sub(CHROME_BORDER_INSET),
        fill,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visual::palette::VisualPalette;
    use katana_ui_core::theme::ThemeSnapshot;

    #[test]
    fn choice_row_border_uses_shared_binary_choice_state_priority() {
        let palette = VisualPalette::from_theme(&ThemeSnapshot::dark());

        assert_eq!(
            palette.border,
            choice_row_border(&palette, false, false, false)
        );
        assert_eq!(
            palette.hover_border,
            choice_row_border(&palette, false, true, false)
        );
        assert_eq!(
            palette.accent,
            choice_row_border(&palette, false, true, true)
        );
        assert_eq!(
            palette.muted,
            choice_row_border(&palette, true, true, false)
        );
    }
}
