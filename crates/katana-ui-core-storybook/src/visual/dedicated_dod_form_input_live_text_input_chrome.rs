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
const SEARCH_ICON_SOURCE_VIEWBOX_SIZE: f32 = 24.0;
const SEARCH_ICON_CIRCLE_CENTER: f32 = 11.0;
const SEARCH_ICON_CIRCLE_RADIUS: f32 = 8.0;
const SEARCH_ICON_LINE_START: f32 = 16.65;
const SEARCH_ICON_LINE_END: f32 = 21.0;
const SEARCH_ICON_STROKE_WIDTH: f32 = 2.0;
const SOURCE_PIXEL_CENTER_OFFSET: f32 = 0.5;
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
    draw_storybook_search_svg_icon(canvas, palette, x, y);
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

fn draw_storybook_search_svg_icon(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
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
    center: f32,
    radius: f32,
    color: u32,
) {
    for current_y in 0..SEARCH_ICON_VIEWBOX_SIZE {
        for current_x in 0..SEARCH_ICON_VIEWBOX_SIZE {
            let dx = source_coordinate(current_x) - center;
            let dy = source_coordinate(current_y) - center;
            let distance_squared = dx.mul_add(dx, dy * dy);
            let half_stroke = SEARCH_ICON_STROKE_WIDTH / 2.0;
            let outer = (radius + half_stroke) * (radius + half_stroke);
            let inner = (radius - half_stroke) * (radius - half_stroke);
            if distance_squared >= inner && distance_squared <= outer {
                canvas.set(x + current_x, y + current_y, color);
            }
        }
    }
}

fn draw_diagonal_handle(canvas: &mut Canvas, x: usize, y: usize, color: u32) {
    let start = scaled_source_coordinate(SEARCH_ICON_LINE_START);
    let end = scaled_source_coordinate(SEARCH_ICON_LINE_END);
    let stroke = scaled_source_coordinate(SEARCH_ICON_STROKE_WIDTH).max(1);
    for offset in 0..=(end - start) {
        canvas.fill_rect(
            x + start + offset,
            y + start + offset,
            stroke,
            stroke,
            color,
        );
    }
}

fn source_coordinate(target_pixel: usize) -> f32 {
    (target_pixel as f32 + SOURCE_PIXEL_CENTER_OFFSET) * SEARCH_ICON_SOURCE_VIEWBOX_SIZE
        / SEARCH_ICON_VIEWBOX_SIZE as f32
}

fn scaled_source_coordinate(source: f32) -> usize {
    (source * SEARCH_ICON_VIEWBOX_SIZE as f32 / SEARCH_ICON_SOURCE_VIEWBOX_SIZE).round() as usize
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
pub(in crate::visual) fn search_svg_fixture_for_test() -> &'static str {
    crate::storybook_svg_fixtures::SEARCH_SVG
}
