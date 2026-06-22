use super::layout_metrics::{LayoutRect, PRESET_TEXT_X_OFFSET};
use super::text::TextRenderer;
use std::borrow::Cow;

const PRESET_TEXT_SIZE: f32 = 12.0;
const PRESET_TEXT_MIN_SIZE: f32 = 9.0;
const PRESET_TEXT_RIGHT_PADDING: usize = 8;
const TRUNCATION_MARKER: &str = "...";

pub(super) struct PresetTabLabel<'a> {
    pub(super) text: Cow<'a, str>,
    pub(super) size: f32,
    pub(super) clip_width: usize,
}

pub(super) fn fit<'a>(text: &TextRenderer, rect: LayoutRect, label: &'a str) -> PresetTabLabel<'a> {
    let clip_width = clip_width(rect);
    let size = fit_size(text, label, clip_width);
    if text.measure_width(label, size) <= clip_width {
        return PresetTabLabel {
            text: Cow::Borrowed(label),
            size,
            clip_width,
        };
    }

    PresetTabLabel {
        text: Cow::Owned(truncate(text, label, size, clip_width)),
        size,
        clip_width,
    }
}

fn fit_size(text: &TextRenderer, label: &str, clip_width: usize) -> f32 {
    let mut size = PRESET_TEXT_SIZE;
    while size > PRESET_TEXT_MIN_SIZE && text.measure_width(label, size) > clip_width {
        size -= 1.0;
    }
    size
}

fn truncate(text: &TextRenderer, label: &str, size: f32, clip_width: usize) -> String {
    if text.measure_width(TRUNCATION_MARKER, size) > clip_width {
        return String::new();
    }

    let mut result = String::new();
    for character in label.chars() {
        let candidate = format!("{result}{character}{TRUNCATION_MARKER}");
        if text.measure_width(&candidate, size) > clip_width {
            break;
        }
        result.push(character);
    }
    result.push_str(TRUNCATION_MARKER);
    result
}

fn clip_width(rect: LayoutRect) -> usize {
    rect.width
        .saturating_sub(PRESET_TEXT_X_OFFSET + PRESET_TEXT_RIGHT_PADDING)
}

#[cfg(test)]
pub(super) fn measured_width_for_test(
    text: &TextRenderer,
    rect: LayoutRect,
    label: &str,
) -> (usize, usize) {
    let fitted = fit(text, rect, label);
    (
        text.measure_width(fitted.text.as_ref(), fitted.size),
        fitted.clip_width,
    )
}
