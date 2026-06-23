use super::{
    BASIC_PRESET_INDEX, BUTTON_LABEL_ICON_OFFSET, BUTTON_LABEL_SIZE, BUTTON_PADDING_X,
    CLASSIC_PRESET_INDEX, ICON_ONLY_SIZE, ICON_SIZE,
};
use crate::visual::canvas::Canvas;
use crate::visual::dedicated_dod_atom_button_live::ButtonLiveKind;
use crate::visual::dedicated_dod_common::{self as common, Rect};
use crate::visual::dedicated_dod_metrics as metrics;
use crate::visual::palette::VisualPalette;
use crate::visual::render_context::ScenarioContext;
use crate::visual::text::{TextRenderer, TextVerticalBox};

pub(in crate::visual) fn draw_button_label(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
    label: &str,
    kind: ButtonLiveKind,
) {
    if !scenario.screen_state.button_options.visible {
        draw_invisible_label(canvas, text, palette, rect);
        return;
    }
    let text_color = label_color(palette, scenario, kind);
    if !kind.has_visible_label() {
        draw_center_icon(canvas, rect, text_color);
        draw_icon_only_label_marker(canvas, palette, scenario, rect);
        draw_external_svg_marker(canvas, palette, scenario, rect);
        draw_aria_label_marker(canvas, palette, scenario, rect);
        return;
    }
    draw_optional_icon(canvas, rect, scenario, kind, text_color);
    draw_external_svg_marker(canvas, palette, scenario, rect);
    draw_aria_label_marker(canvas, palette, scenario, rect);
    let label_width = measure_button_label_width(text, label);
    text.draw_centered(
        canvas,
        label,
        centered_label_x(rect, label_width, scenario, kind.has_icon()),
        TextVerticalBox::new(rect.y, rect.height as f32),
        BUTTON_LABEL_SIZE,
        text_color,
    );
}

pub(in crate::visual) fn measure_button_label_width(text: &TextRenderer, label: &str) -> usize {
    text.measure_width(label, BUTTON_LABEL_SIZE)
}

fn draw_invisible_label(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    rect: Rect,
) {
    text.draw_centered(
        canvas,
        "visible=false",
        rect.x + BUTTON_PADDING_X / metrics::PX_2,
        TextVerticalBox::new(rect.y, rect.height as f32),
        BUTTON_LABEL_SIZE,
        palette.muted,
    );
}

fn draw_optional_icon(
    canvas: &mut Canvas,
    rect: Rect,
    scenario: ScenarioContext<'_>,
    kind: ButtonLiveKind,
    color: u32,
) {
    if !kind.has_icon() {
        return;
    }
    let icon_x = if scenario.screen_state.button_options.icon_trailing() {
        rect.x
            + rect
                .width
                .saturating_sub(BUTTON_LABEL_ICON_OFFSET + ICON_SIZE)
    } else {
        rect.x + BUTTON_LABEL_ICON_OFFSET
    };
    common::cross_icon(
        canvas,
        icon_x,
        rect.y + (rect.height - ICON_SIZE) / metrics::PX_2,
        ICON_SIZE,
        color,
    );
}

fn label_color(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    kind: ButtonLiveKind,
) -> u32 {
    if scenario.screen_state.button_options.disabled {
        return palette.muted;
    }
    if matches!(kind, ButtonLiveKind::TextButton) {
        return text_button_label_color(palette, scenario);
    }
    if scenario.screen_state.has_settings_override() {
        return palette.text;
    }
    if scenario.screen_state.is_button_pressed() {
        return palette.background;
    }
    if matches!(
        scenario.preset_index,
        CLASSIC_PRESET_INDEX | BASIC_PRESET_INDEX
    ) {
        return palette.text;
    }
    palette.background
}

fn text_button_label_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.is_button_pressed() {
        return common::SUCCESS;
    }
    palette.accent
}

fn draw_center_icon(canvas: &mut Canvas, rect: Rect, color: u32) {
    common::cross_icon(
        canvas,
        rect.x + (rect.width - ICON_ONLY_SIZE) / metrics::PX_2,
        rect.y + (rect.height - ICON_ONLY_SIZE) / metrics::PX_2,
        ICON_ONLY_SIZE,
        color,
    );
}

fn draw_icon_only_label_marker(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
) {
    if !scenario.screen_state.button_options.japanese_label {
        return;
    }
    canvas.fill_rect(
        rect.x + rect.width.saturating_sub(metrics::PX_14),
        rect.y + rect.height.saturating_sub(metrics::PX_10),
        metrics::PX_10,
        metrics::PX_3,
        palette.background,
    );
}

fn draw_external_svg_marker(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
) {
    if !scenario.screen_state.button_options.external_svg_source {
        return;
    }
    canvas.stroke_rect(
        rect.x + metrics::PX_8,
        rect.y + metrics::PX_8,
        rect.width.saturating_sub(metrics::PX_16),
        rect.height.saturating_sub(metrics::PX_16),
        palette.background,
    );
}

fn draw_aria_label_marker(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
) {
    if !scenario.screen_state.button_options.aria_label {
        return;
    }
    canvas.fill_rect(
        rect.x + metrics::PX_4,
        rect.y + metrics::PX_4,
        metrics::PX_6,
        metrics::PX_3,
        palette.selection,
    );
}

fn centered_label_x(
    rect: Rect,
    label_width: usize,
    scenario: ScenarioContext<'_>,
    icon: bool,
) -> usize {
    let icon_offset = if icon && !scenario.screen_state.button_options.icon_trailing() {
        BUTTON_LABEL_ICON_OFFSET
    } else {
        metrics::PX_0
    };
    rect.x + icon_offset + (rect.width.saturating_sub(label_width + icon_offset)) / metrics::PX_2
}

#[cfg(test)]
pub(in crate::visual) fn centered_label_x_for_test(
    rect: Rect,
    label_width: usize,
    scenario: ScenarioContext<'_>,
    icon: bool,
) -> usize {
    centered_label_x(rect, label_width, scenario, icon)
}
