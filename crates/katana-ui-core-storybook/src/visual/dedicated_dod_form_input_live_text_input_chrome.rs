use super::super::dedicated_dod_form_input_live_layout::{
    FIELD_HEIGHT, FIELD_TRAILING_BUTTON_COUNT, FIELD_X, FIELD_Y,
    text_input_trailing_icon_button_rects,
};
use super::{
    Canvas, INPUT_ICON_BUTTONS_PRESET_INDEX, INPUT_LEADING_ICON_PRESET_INDEX,
    INPUT_PLACEHOLDER_PRESET_INDEX, INPUT_READONLY_PRESET_INDEX, INPUT_RESERVED_SLOT_PRESET_INDEX,
    TextRenderer, VisualPalette, m,
};
use crate::visual::layout_metrics::LayoutRect;
use crate::visual::text::TextBox;

const PLACEHOLDER_TEXT: &str = "ファイル名で検索...";
const TRAILING_BUTTON_LABELS: [&str; FIELD_TRAILING_BUTTON_COUNT] = [".*", "ab", "Aa"];
const SEARCH_ICON_LEFT_IN_FIELD: usize = 8;
const SEARCH_ICON_VIEWBOX_SIZE: usize = 16;
const SEARCH_ICON_CIRCLE_CENTER: isize = 7;
const SEARCH_ICON_CIRCLE_RADIUS: isize = 4;
const SEARCH_ICON_HANDLE_START: usize = 10;
const SEARCH_ICON_HANDLE_END: usize = 14;
const SEARCH_ICON_STROKE_WIDTH: usize = 2;
const HOVER_INNER_BORDER_INSET: usize = 1;
const HOVER_INNER_BORDER_SHRINK: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LeadingSlotMode {
    None,
    Reserved,
    SearchIcon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct TextInputChrome {
    pub(super) leading_slot: LeadingSlotMode,
    pub(super) trailing_icon_buttons: bool,
    pub(super) readonly: bool,
    pub(super) placeholder: Option<&'static str>,
}

impl TextInputChrome {
    pub(super) const fn plain() -> Self {
        Self {
            leading_slot: LeadingSlotMode::None,
            trailing_icon_buttons: false,
            readonly: false,
            placeholder: None,
        }
    }

    pub(super) const fn search() -> Self {
        Self {
            leading_slot: LeadingSlotMode::SearchIcon,
            trailing_icon_buttons: false,
            readonly: false,
            placeholder: None,
        }
    }

    pub(super) const fn leading_slot_reserved(&self) -> bool {
        !matches!(self.leading_slot, LeadingSlotMode::None)
    }
}

pub(super) fn draw_leading_slot(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    mode: LeadingSlotMode,
) {
    if mode != LeadingSlotMode::SearchIcon {
        return;
    }
    draw_katana_search_svg_icon(canvas, palette, x, y);
}

pub(super) fn draw_trailing_icon_buttons(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    visible: bool,
    hovered_index: Option<usize>,
) {
    if !visible {
        return;
    }
    for (index, (rect, label)) in text_input_trailing_icon_button_rects(x, y)
        .into_iter()
        .zip(TRAILING_BUTTON_LABELS)
        .enumerate()
    {
        let hovered = hovered_index == Some(index);
        let border = if hovered {
            palette.hover_border
        } else {
            palette.border
        };
        canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.panel);
        canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, border);
        if hovered {
            canvas.stroke_rect(
                rect.x + HOVER_INNER_BORDER_INSET,
                rect.y + HOVER_INNER_BORDER_INSET,
                rect.width - HOVER_INNER_BORDER_SHRINK,
                rect.height - HOVER_INNER_BORDER_SHRINK,
                border,
            );
        }
        text.draw_in_box(
            canvas,
            label,
            TextBox::centered(rect.x, rect.y, rect.width, rect.height),
            m::FONT_8,
            palette.text,
        );
    }
}

pub(super) const fn chrome_for_preset(index: usize) -> TextInputChrome {
    match index {
        INPUT_READONLY_PRESET_INDEX => TextInputChrome {
            readonly: true,
            ..TextInputChrome::plain()
        },
        INPUT_PLACEHOLDER_PRESET_INDEX => TextInputChrome {
            placeholder: Some(PLACEHOLDER_TEXT),
            ..TextInputChrome::plain()
        },
        INPUT_RESERVED_SLOT_PRESET_INDEX => TextInputChrome {
            leading_slot: LeadingSlotMode::Reserved,
            ..TextInputChrome::plain()
        },
        INPUT_LEADING_ICON_PRESET_INDEX => TextInputChrome {
            leading_slot: LeadingSlotMode::SearchIcon,
            ..TextInputChrome::plain()
        },
        INPUT_ICON_BUTTONS_PRESET_INDEX => TextInputChrome {
            leading_slot: LeadingSlotMode::SearchIcon,
            trailing_icon_buttons: true,
            ..TextInputChrome::plain()
        },
        _ => TextInputChrome::plain(),
    }
}

fn draw_katana_search_svg_icon(canvas: &mut Canvas, palette: &VisualPalette, x: usize, y: usize) {
    let icon = search_icon_visual_rect(x, y);
    draw_stroked_circle(
        canvas,
        icon.x,
        icon.y,
        SEARCH_ICON_CIRCLE_CENTER,
        SEARCH_ICON_CIRCLE_RADIUS,
        palette.muted,
    );
    draw_diagonal_handle(canvas, icon.x, icon.y, palette.muted);
}

fn draw_stroked_circle(
    canvas: &mut Canvas,
    x: usize,
    y: usize,
    center: isize,
    radius: isize,
    color: u32,
) {
    for current_y in 0..SEARCH_ICON_VIEWBOX_SIZE {
        for current_x in 0..SEARCH_ICON_VIEWBOX_SIZE {
            let dx = current_x as isize - center;
            let dy = current_y as isize - center;
            let distance_squared = dx * dx + dy * dy;
            let outer = radius * radius + radius;
            let inner = radius * radius - radius;
            if distance_squared >= inner && distance_squared <= outer {
                canvas.set(x + current_x, y + current_y, color);
            }
        }
    }
}

fn draw_diagonal_handle(canvas: &mut Canvas, x: usize, y: usize, color: u32) {
    for offset in 0..=(SEARCH_ICON_HANDLE_END - SEARCH_ICON_HANDLE_START) {
        canvas.fill_rect(
            x + SEARCH_ICON_HANDLE_START + offset,
            y + SEARCH_ICON_HANDLE_START + offset,
            SEARCH_ICON_STROKE_WIDTH,
            SEARCH_ICON_STROKE_WIDTH,
            color,
        );
    }
}

fn search_icon_visual_rect(x: usize, y: usize) -> LayoutRect {
    LayoutRect::new(
        x + FIELD_X + SEARCH_ICON_LEFT_IN_FIELD,
        y + FIELD_Y + (FIELD_HEIGHT - SEARCH_ICON_VIEWBOX_SIZE).div_ceil(2),
        SEARCH_ICON_VIEWBOX_SIZE,
        SEARCH_ICON_VIEWBOX_SIZE,
    )
}

#[cfg(test)]
pub(in crate::visual) fn search_icon_visual_rect_for_test(x: usize, y: usize) -> LayoutRect {
    search_icon_visual_rect(x, y)
}

#[cfg(test)]
pub(in crate::visual) fn katana_search_svg_for_test() -> &'static str {
    crate::katana_icons::SEARCH_SVG
}
