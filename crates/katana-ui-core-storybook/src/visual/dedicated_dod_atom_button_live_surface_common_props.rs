use super::{INNER_STROKE_INSET, INNER_STROKE_REDUCTION, SHADOW_OFFSET};
use crate::visual::button_options::{
    StorybookButtonCommandMode, StorybookButtonTabIndex, StorybookButtonZIndex,
};
use crate::visual::canvas::Canvas;
use crate::visual::dedicated_dod_common::Rect;
use crate::visual::dedicated_dod_metrics as m;
use crate::visual::palette::VisualPalette;
use crate::visual::render_context::ScenarioContext;

pub(super) fn draw_setting_outline(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
) {
    if scenario.screen_state.has_settings_override() && scenario.screen_state.button_options.border
    {
        canvas.stroke_rect(
            rect.x + INNER_STROKE_INSET,
            rect.y + INNER_STROKE_INSET,
            rect.width - INNER_STROKE_REDUCTION,
            rect.height - INNER_STROKE_REDUCTION,
            palette.accent,
        );
    }
}

pub(super) fn draw_hover_border(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
) {
    if !scenario.screen_state.preview_hovered || scenario.screen_state.button_options.disabled {
        return;
    }
    canvas.stroke_rect(
        rect.x,
        rect.y,
        rect.width,
        rect.height,
        palette.hover_border,
    );
}

pub(super) fn draw_focus_ring(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
) {
    if !scenario.screen_state.is_button_focused() || scenario.screen_state.button_options.disabled {
        return;
    }
    canvas.stroke_rect(
        rect.x.saturating_sub(m::PX_2),
        rect.y.saturating_sub(m::PX_2),
        rect.width + m::PX_4,
        rect.height + m::PX_4,
        palette.accent,
    );
    canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, palette.accent);
}

pub(super) fn draw_invisible_placeholder(canvas: &mut Canvas, palette: &VisualPalette, rect: Rect) {
    canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, palette.muted);
}

pub(super) fn draw_focusability_marker(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
) {
    if scenario.screen_state.button_options.focusable {
        return;
    }
    let marker_x = rect.x + rect.width.saturating_sub(m::PX_12);
    let marker_y = rect.y + m::PX_6;
    canvas.fill_rect(marker_x, marker_y, m::PX_8, m::PX_2, palette.muted);
    canvas.fill_rect(
        marker_x + m::PX_3,
        marker_y + m::PX_2,
        m::PX_2,
        m::PX_8,
        palette.muted,
    );
}

pub(super) fn draw_command_marker(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
) {
    if !matches!(
        scenario.screen_state.button_options.command_mode,
        StorybookButtonCommandMode::Open
    ) {
        return;
    }
    canvas.fill_rect(
        rect.x + rect.width.saturating_sub(m::PX_12),
        rect.y + rect.height.saturating_sub(m::PX_12),
        m::PX_6,
        m::PX_6,
        palette.background,
    );
}

pub(super) fn draw_keyboard_activation_marker(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
) {
    if scenario.screen_state.button_options.keyboard_activation {
        return;
    }
    let marker_x = rect.x + m::PX_6;
    let marker_y = rect.y + rect.height.saturating_sub(m::PX_10);
    canvas.fill_rect(marker_x, marker_y, m::PX_14, m::PX_2, palette.muted);
    canvas.fill_rect(
        marker_x,
        marker_y + m::PX_4,
        m::PX_8,
        m::PX_2,
        palette.muted,
    );
}

pub(super) fn draw_icon_position_marker(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
) {
    if !scenario.screen_state.button_options.icon_trailing() {
        return;
    }
    let marker_x = rect.x + rect.width.saturating_sub(m::PX_18);
    let marker_y = rect.y + m::PX_6;
    canvas.stroke_rect(marker_x, marker_y, m::PX_10, m::PX_10, palette.background);
}

pub(super) fn draw_tab_index_marker(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
) {
    let color = match scenario.screen_state.button_options.tab_index {
        StorybookButtonTabIndex::One => palette.background,
        StorybookButtonTabIndex::Disabled => palette.muted,
        StorybookButtonTabIndex::Zero => return,
    };
    canvas.stroke_rect(
        rect.x + m::PX_4,
        rect.y + m::PX_4,
        m::PX_12,
        m::PX_10,
        color,
    );
}

pub(super) fn draw_z_index_marker(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
) {
    let inset = match scenario.screen_state.button_options.z_index {
        StorybookButtonZIndex::Raised => m::PX_6,
        StorybookButtonZIndex::Overlay => m::PX_10,
        StorybookButtonZIndex::Auto => return,
    };
    canvas.fill_rect(
        rect.x + inset,
        rect.y + rect.height.saturating_sub(m::PX_6),
        rect.width.saturating_sub(inset * m::PX_2),
        m::PX_2,
        palette.background,
    );
    canvas.fill_rect(
        rect.x + rect.width.saturating_sub(m::PX_6),
        rect.y + inset,
        m::PX_2,
        rect.height.saturating_sub(inset * m::PX_2),
        palette.background,
    );
}

pub(super) fn draw_z_index_shadow(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    rect: Rect,
    z_index: StorybookButtonZIndex,
) {
    let offset = match z_index {
        StorybookButtonZIndex::Raised => SHADOW_OFFSET,
        StorybookButtonZIndex::Overlay => SHADOW_OFFSET * m::PX_2,
        StorybookButtonZIndex::Auto => return,
    };
    canvas.fill_rect(
        rect.x + offset,
        rect.y + offset,
        rect.width,
        rect.height,
        palette.code_background,
    );
}

#[cfg(test)]
mod tests {
    use super::{
        Canvas, Rect, ScenarioContext, StorybookButtonTabIndex, StorybookButtonZIndex,
        VisualPalette, draw_tab_index_marker, draw_z_index_marker, draw_z_index_shadow,
    };
    use crate::visual::screen_state::StorybookScreenState;
    use katana_ui_core::theme::ThemeSnapshot;

    #[test]
    fn disabled_tab_index_and_overlay_z_index_draw_their_distinct_markers() {
        let palette = VisualPalette::from_theme(&ThemeSnapshot::dark());
        let mut canvas = Canvas::new(80, 80, palette.surface);
        let rect = Rect::new(16, 16, 40, 40);
        let mut state = StorybookScreenState::default();
        state.button_options.tab_index = StorybookButtonTabIndex::Disabled;
        state.button_options.z_index = StorybookButtonZIndex::Overlay;
        let scenario = ScenarioContext {
            selected_page: "button",
            selected_instance_id: "button.preview",
            preset_index: 0,
            preset_tab_scroll_x: 0,
            tree_expansion: Default::default(),
            scrollbar_visible: true,
            panel_scroll: Default::default(),
            screen_state: &state,
            show_navigation_lines: true,
            show_navigation_text_connectors: false,
        };

        draw_tab_index_marker(&mut canvas, &palette, scenario, rect);
        draw_z_index_marker(&mut canvas, &palette, scenario, rect);
        draw_z_index_shadow(&mut canvas, &palette, rect, StorybookButtonZIndex::Overlay);

        assert!(canvas.non_background_pixels(palette.surface) > 0);
    }
}
