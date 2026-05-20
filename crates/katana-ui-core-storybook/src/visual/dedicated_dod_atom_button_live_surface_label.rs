use super::{
    BASIC_PRESET_INDEX, BUTTON_LABEL_AVG_WIDTH, BUTTON_LABEL_ICON_OFFSET, BUTTON_LABEL_SIZE,
    BUTTON_PADDING_X, CLASSIC_PRESET_INDEX, ICON_ONLY_SIZE, ICON_SIZE,
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
        return;
    }
    draw_optional_icon(canvas, rect, kind, text_color);
    text.draw_centered(
        canvas,
        label,
        centered_label_x(rect, label, kind.has_icon()),
        TextVerticalBox::new(rect.y, rect.height as f32),
        BUTTON_LABEL_SIZE,
        text_color,
    );
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

fn draw_optional_icon(canvas: &mut Canvas, rect: Rect, kind: ButtonLiveKind, color: u32) {
    if kind.has_icon() {
        common::cross_icon(
            canvas,
            rect.x + BUTTON_LABEL_ICON_OFFSET,
            rect.y + (rect.height - ICON_SIZE) / metrics::PX_2,
            ICON_SIZE,
            color,
        );
    }
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
    if scenario.screen_state.has_widget_action() {
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
    if scenario.screen_state.has_widget_action() {
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

fn centered_label_x(rect: Rect, label: &str, icon: bool) -> usize {
    let icon_offset = if icon {
        BUTTON_LABEL_ICON_OFFSET
    } else {
        metrics::PX_0
    };
    let text_width = label.chars().count() * BUTTON_LABEL_AVG_WIDTH;
    rect.x + icon_offset + (rect.width.saturating_sub(text_width + icon_offset)) / metrics::PX_2
}
