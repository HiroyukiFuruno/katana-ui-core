use super::types::ToggleSize;
use crate::theme::Theme;
use crate::theme::color::Color;

const TRACK_W_SM: f32 = 28.0;
const TRACK_H_SM: f32 = 16.0;
const TRACK_W_MD: f32 = 36.0;
const TRACK_H_MD: f32 = 20.0;
const TRACK_W_LG: f32 = 44.0;
const TRACK_H_LG: f32 = 24.0;
const THUMB_INSET: f32 = 2.0;

pub(super) struct TrackDims {
    pub width: f32,
    pub height: f32,
}

pub(super) fn track_dims(size: ToggleSize) -> TrackDims {
    match size {
        ToggleSize::Sm => TrackDims {
            width: TRACK_W_SM,
            height: TRACK_H_SM,
        },
        ToggleSize::Md => TrackDims {
            width: TRACK_W_MD,
            height: TRACK_H_MD,
        },
        ToggleSize::Lg => TrackDims {
            width: TRACK_W_LG,
            height: TRACK_H_LG,
        },
    }
}

pub(super) fn thumb_size(dims: &TrackDims) -> f32 {
    dims.height - THUMB_INSET * 2.0
}

pub(super) fn thumb_offset_on(dims: &TrackDims) -> f32 {
    dims.width - thumb_size(dims) - THUMB_INSET
}

pub(super) fn thumb_offset_off() -> f32 {
    THUMB_INSET
}

pub(super) fn track_color(value: bool, disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.border
    } else if value {
        theme.color.accent
    } else {
        theme.color.surface
    }
}

pub(super) fn thumb_color(disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.text_disabled
    } else {
        theme.color.bg
    }
}
