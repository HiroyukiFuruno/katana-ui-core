use super::{BUTTON_BORDER, BUTTON_RADIUS, HOVER_BLEND_ALPHA, PRESSED_BLEND_ALPHA};
use crate::visual::canvas::Canvas;
use crate::visual::dedicated_dod_atom_button_live::ButtonLiveKind;
use crate::visual::dedicated_dod_common::Rect;
use crate::visual::dedicated_dod_metrics as metrics;
use crate::visual::palette::VisualPalette;
use crate::visual::render_context::ScenarioContext;

pub(super) fn draw_material_surface(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
    kind: ButtonLiveKind,
    fill: u32,
) {
    if matches!(kind, ButtonLiveKind::TextButton) {
        draw_text_button_surface(canvas, palette, scenario, rect, fill);
        return;
    }
    draw_box_button_surface(canvas, palette, scenario, rect, fill);
}

fn draw_box_button_surface(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
    fill: u32,
) {
    let surface = interactive_fill(palette, scenario, fill);
    if scenario.screen_state.button_options.border {
        draw_bordered_surface(canvas, palette, scenario, rect, surface);
    } else {
        canvas.fill_round_rect(
            rect.x,
            rect.y,
            rect.width,
            rect.height,
            BUTTON_RADIUS,
            surface,
        );
    }
    draw_interaction_chrome(canvas, palette, scenario, rect);
}

fn draw_bordered_surface(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
    surface: u32,
) {
    let border = if scenario.screen_state.preview_hovered {
        palette.accent
    } else {
        palette.border
    };
    canvas.fill_round_rect(
        rect.x,
        rect.y,
        rect.width,
        rect.height,
        BUTTON_RADIUS,
        border,
    );
    canvas.fill_round_rect(
        rect.x + BUTTON_BORDER,
        rect.y + BUTTON_BORDER,
        rect.width.saturating_sub(BUTTON_BORDER * metrics::PX_2),
        rect.height.saturating_sub(BUTTON_BORDER * metrics::PX_2),
        BUTTON_RADIUS.saturating_sub(BUTTON_BORDER),
        surface,
    );
}

fn draw_text_button_surface(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
    fill: u32,
) {
    canvas.fill_round_rect(rect.x, rect.y, rect.width, rect.height, BUTTON_RADIUS, fill);
    if scenario.screen_state.preview_hovered {
        canvas.blend_rect(
            rect.x,
            rect.y,
            rect.width,
            rect.height,
            palette.accent,
            HOVER_BLEND_ALPHA,
        );
    }
}

fn interactive_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>, fill: u32) -> u32 {
    let mut surface = fill;
    if scenario.screen_state.preview_hovered && !scenario.screen_state.button_options.disabled {
        surface = blend_color(surface, palette.accent, HOVER_BLEND_ALPHA);
    }
    if scenario.screen_state.has_widget_action() {
        surface = blend_color(surface, palette.background, PRESSED_BLEND_ALPHA);
    }
    surface
}

fn draw_interaction_chrome(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
) {
    if scenario.screen_state.preview_hovered && !scenario.screen_state.button_options.disabled {
        canvas.fill_rect(
            rect.x + BUTTON_RADIUS,
            rect.y,
            rect.width - BUTTON_RADIUS * metrics::PX_2,
            metrics::PX_1,
            palette.accent,
        );
    }
    if !scenario.screen_state.has_widget_action() {
        return;
    }
    canvas.fill_rect(
        rect.x + metrics::PX_2,
        rect.y + metrics::PX_2,
        rect.width.saturating_sub(metrics::PX_4),
        metrics::PX_2,
        palette.background,
    );
    canvas.fill_rect(
        rect.x + metrics::PX_2,
        rect.y + rect.height.saturating_sub(metrics::PX_4),
        rect.width.saturating_sub(metrics::PX_4),
        metrics::PX_2,
        palette.selection,
    );
}

fn blend_color(base: u32, overlay: u32, alpha: u8) -> u32 {
    const RED_SHIFT: u32 = 16;
    const GREEN_SHIFT: u32 = 8;
    const ALPHA_MAX: u32 = 255;
    let alpha = u32::from(alpha);
    let inverse = ALPHA_MAX - alpha;
    let red = blend_channel(base, overlay, alpha, inverse, RED_SHIFT);
    let green = blend_channel(base, overlay, alpha, inverse, GREEN_SHIFT);
    let blue = blend_channel(base, overlay, alpha, inverse, 0);
    (red << RED_SHIFT) | (green << GREEN_SHIFT) | blue
}

fn blend_channel(base: u32, overlay: u32, alpha: u32, inverse: u32, shift: u32) -> u32 {
    let base_channel = (base >> shift) & 0xff;
    let overlay_channel = (overlay >> shift) & 0xff;
    (overlay_channel * alpha + base_channel * inverse) / 255
}
