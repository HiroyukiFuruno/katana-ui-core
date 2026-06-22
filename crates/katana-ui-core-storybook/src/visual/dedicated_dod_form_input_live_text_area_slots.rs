use super::super::dedicated_dod_form_input_live_layout::{FIELD_X, TEXT_AREA_WIDTH, TEXT_AREA_Y};
use super::{Canvas, ScenarioContext, TextRenderer, VisualPalette, m};
use crate::visual::layout_metrics::LayoutRect;
use crate::visual::text::TextBox;

pub(in crate::visual) const TEXT_AREA_LEADING_SVG_PRESET_INDEX: usize = 10;
pub(in crate::visual) const TEXT_AREA_TRAILING_BUTTONS_PRESET_INDEX: usize = 11;
pub(in crate::visual) const TEXT_AREA_CLEAR_ACTION_PRESET_INDEX: usize = 12;

const ICON_LEFT_IN_AREA: usize = 8;
const ICON_TOP_IN_AREA: usize = 8;
const ICON_SIZE: usize = 14;
const ICON_HANDLE_OFFSET: usize = 9;
const ICON_HANDLE_SIZE: usize = 5;
const LINE_TEXT_SLOT_OFFSET: usize = 22;
const TRAILING_BUTTON_COUNT: usize = 2;
const TRAILING_BUTTON_SIZE: usize = 20;
const TRAILING_BUTTON_GAP: usize = 4;
const TRAILING_BUTTON_INSET: usize = 6;
const TRAILING_BUTTON_TOP_OFFSET: usize = 8;
const CLEAR_ACTION_WIDTH: usize = 54;
const CLEAR_ACTION_HEIGHT: usize = 18;
const CLEAR_ACTION_RIGHT_INSET: usize = 8;
const CLEAR_ACTION_BOTTOM_INSET: usize = 8;
const CLEAR_ACTION_TEXT_SIZE: f32 = 8.0;
const BUTTON_LABELS: [&str; TRAILING_BUTTON_COUNT] = ["clr", "fmt"];

pub(super) fn draw_text_area_entry_slots(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    draw_leading_svg_icon(canvas, palette, scenario, x, y);
    draw_trailing_icon_buttons(canvas, text, palette, scenario, x, y);
    draw_clear_action(canvas, text, palette, scenario, x, y);
}

pub(super) fn line_text_x_offset(preset_index: usize) -> usize {
    if leading_svg_visible(preset_index) {
        return LINE_TEXT_SLOT_OFFSET;
    }
    0
}

pub(in crate::visual) fn text_area_trailing_icon_button_rects(
    x: usize,
    y: usize,
) -> [LayoutRect; TRAILING_BUTTON_COUNT] {
    let total_width = TRAILING_BUTTON_COUNT * TRAILING_BUTTON_SIZE
        + (TRAILING_BUTTON_COUNT - 1) * TRAILING_BUTTON_GAP;
    let left = x + FIELD_X + TEXT_AREA_WIDTH - TRAILING_BUTTON_INSET - total_width;
    let top = y + TEXT_AREA_Y + TRAILING_BUTTON_TOP_OFFSET;
    [
        trailing_icon_button_rect(left, top, 0),
        trailing_icon_button_rect(left, top, 1),
    ]
}

fn draw_leading_svg_icon(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    if !leading_svg_visible(scenario.preset_index) {
        return;
    }
    let icon = leading_icon_rect(x, y);
    canvas.stroke_rect(icon.x, icon.y, icon.width, icon.height, palette.muted);
    canvas.fill_rect(
        icon.x + ICON_HANDLE_OFFSET,
        icon.y + ICON_HANDLE_OFFSET,
        ICON_HANDLE_SIZE,
        m::PX_2,
        palette.muted,
    );
}

fn draw_trailing_icon_buttons(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    if scenario.preset_index != TEXT_AREA_TRAILING_BUTTONS_PRESET_INDEX {
        return;
    }
    for (index, (rect, label)) in text_area_trailing_icon_button_rects(x, y)
        .into_iter()
        .zip(BUTTON_LABELS)
        .enumerate()
    {
        let border = if scenario.screen_state.hovered_text_area_icon_button_index == Some(index) {
            palette.hover_border
        } else {
            palette.border
        };
        canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.panel);
        canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, border);
        text.draw_in_box(
            canvas,
            label,
            TextBox::centered(rect.x, rect.y, rect.width, rect.height),
            CLEAR_ACTION_TEXT_SIZE,
            palette.text,
        );
    }
}

fn draw_clear_action(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    if scenario.preset_index != TEXT_AREA_CLEAR_ACTION_PRESET_INDEX {
        return;
    }
    let rect = clear_action_rect(x, y);
    let border = if scenario.screen_state.hovered_text_area_clear_action {
        palette.hover_border
    } else {
        palette.border
    };
    canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.panel);
    canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, border);
    text.draw_in_box(
        canvas,
        "clear",
        TextBox::centered(rect.x, rect.y, rect.width, rect.height),
        CLEAR_ACTION_TEXT_SIZE,
        palette.text,
    );
}

fn leading_svg_visible(preset_index: usize) -> bool {
    matches!(
        preset_index,
        TEXT_AREA_LEADING_SVG_PRESET_INDEX | TEXT_AREA_TRAILING_BUTTONS_PRESET_INDEX
    )
}

fn leading_icon_rect(x: usize, y: usize) -> LayoutRect {
    LayoutRect::new(
        x + FIELD_X + ICON_LEFT_IN_AREA,
        y + TEXT_AREA_Y + ICON_TOP_IN_AREA,
        ICON_SIZE,
        ICON_SIZE,
    )
}

pub(in crate::visual) fn clear_action_rect(x: usize, y: usize) -> LayoutRect {
    LayoutRect::new(
        x + FIELD_X + TEXT_AREA_WIDTH - CLEAR_ACTION_RIGHT_INSET - CLEAR_ACTION_WIDTH,
        y + TEXT_AREA_Y + m::PX_92 - CLEAR_ACTION_BOTTOM_INSET - CLEAR_ACTION_HEIGHT,
        CLEAR_ACTION_WIDTH,
        CLEAR_ACTION_HEIGHT,
    )
}

fn trailing_icon_button_rect(left: usize, top: usize, index: usize) -> LayoutRect {
    LayoutRect::new(
        left + index * (TRAILING_BUTTON_SIZE + TRAILING_BUTTON_GAP),
        top,
        TRAILING_BUTTON_SIZE,
        TRAILING_BUTTON_SIZE,
    )
}
