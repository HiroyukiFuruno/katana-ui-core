use super::{
    BASIC_HEIGHT, BASIC_MIN_WIDTH, BASIC_PRESET_INDEX, BUTTON_ICON_GAP, BUTTON_LABEL_AVG_WIDTH,
    BUTTON_LABEL_ICON_OFFSET, BUTTON_PADDING_X, CLASSIC_HEIGHT, CLASSIC_MIN_WIDTH,
    CLASSIC_PRESET_INDEX, CUSTOM_WIDTH, DENSE_HEIGHT, DENSE_MIN_WIDTH, DENSE_PRESET_INDEX,
    FILL_WIDTH, MODERN_HEIGHT, MODERN_MIN_WIDTH, PERCENT_WIDTH,
};
use crate::visual::button_options::{StorybookButtonHeightMode, StorybookButtonWidthMode};

const TALL_HEIGHT: usize = 48;
const ICON_ONLY_WIDTH: usize = 44;

pub(in crate::visual) fn button_layout(
    preset_index: usize,
    width_mode: StorybookButtonWidthMode,
    height_mode: StorybookButtonHeightMode,
    label: &str,
    icon: bool,
    label_visible: bool,
) -> ButtonVisualLayout {
    let height = button_height(preset_index, height_mode);
    match width_mode {
        StorybookButtonWidthMode::Px => return ButtonVisualLayout::new(CUSTOM_WIDTH, height),
        StorybookButtonWidthMode::Percent => return ButtonVisualLayout::new(PERCENT_WIDTH, height),
        StorybookButtonWidthMode::Fill => return ButtonVisualLayout::new(FILL_WIDTH, height),
        StorybookButtonWidthMode::Auto => {}
    }
    if !label_visible {
        return ButtonVisualLayout::new(ICON_ONLY_WIDTH.max(height), height);
    }
    let min_width = match preset_index {
        CLASSIC_PRESET_INDEX => CLASSIC_MIN_WIDTH,
        BASIC_PRESET_INDEX => BASIC_MIN_WIDTH,
        DENSE_PRESET_INDEX => DENSE_MIN_WIDTH,
        _ => MODERN_MIN_WIDTH,
    };
    auto_layout(label, min_width, height, icon)
}

fn button_height(preset_index: usize, height_mode: StorybookButtonHeightMode) -> usize {
    match height_mode {
        StorybookButtonHeightMode::Compact => DENSE_HEIGHT,
        StorybookButtonHeightMode::Tall => TALL_HEIGHT,
        StorybookButtonHeightMode::Auto => preset_height(preset_index),
    }
}

fn preset_height(preset_index: usize) -> usize {
    match preset_index {
        CLASSIC_PRESET_INDEX => CLASSIC_HEIGHT,
        BASIC_PRESET_INDEX => BASIC_HEIGHT,
        DENSE_PRESET_INDEX => DENSE_HEIGHT,
        _ => MODERN_HEIGHT,
    }
}

fn auto_layout(label: &str, min_width: usize, height: usize, icon: bool) -> ButtonVisualLayout {
    let icon_space = if icon {
        BUTTON_LABEL_ICON_OFFSET + BUTTON_ICON_GAP
    } else {
        0
    };
    let text_width = label.chars().count() * BUTTON_LABEL_AVG_WIDTH;
    ButtonVisualLayout::new(
        min_width.max(text_width + BUTTON_PADDING_X + icon_space),
        height,
    )
}

pub(in crate::visual) struct ButtonVisualLayout {
    pub(in crate::visual) width: usize,
    pub(in crate::visual) height: usize,
}

impl ButtonVisualLayout {
    const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}
