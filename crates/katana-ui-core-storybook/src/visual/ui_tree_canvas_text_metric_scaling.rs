use super::{BODY_FONT_SIZE, COMPACT_BODY_FONT_SIZE};
use katana_ui_core::render_model::UiDimension;

const UNDERLINE_OFFSET_SCALE_NUMERATOR: usize = 122;
const STRIKE_THROUGH_OFFSET_SCALE_NUMERATOR: usize = 72;
const OFFSET_SCALE_DENOMINATOR: usize = 100;

pub(super) const fn underline_offset(font_size: f32) -> usize {
    (font_size as usize).saturating_mul(UNDERLINE_OFFSET_SCALE_NUMERATOR) / OFFSET_SCALE_DENOMINATOR
}

pub(super) const fn strikethrough_offset(font_size: f32) -> usize {
    (font_size as usize).saturating_mul(STRIKE_THROUGH_OFFSET_SCALE_NUMERATOR)
        / OFFSET_SCALE_DENOMINATOR
}

pub(super) fn scale_usize(value: usize, scale: f32) -> usize {
    if value == 0 {
        return 0;
    }
    ((value as f32) * scale).round().max(1.0) as usize
}

pub(super) fn dimension_px(value: &UiDimension) -> usize {
    match value {
        UiDimension::Px(value) => usize::from(*value),
        _ => 0,
    }
}

pub(super) fn scaled_document_text_line_height(
    default_height: usize,
    compact_height: usize,
    font_size: f32,
) -> usize {
    if font_size <= COMPACT_BODY_FONT_SIZE {
        return compact_height;
    }
    if font_size >= BODY_FONT_SIZE {
        return default_height;
    }
    let ratio = (font_size - COMPACT_BODY_FONT_SIZE) / (BODY_FONT_SIZE - COMPACT_BODY_FONT_SIZE);
    (compact_height as f32 + (default_height - compact_height) as f32 * ratio)
        .round()
        .max(1.0) as usize
}
