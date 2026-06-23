use crate::visual::canvas::Canvas;
use crate::visual::dedicated_dod_common::Rect;
use crate::visual::dedicated_dod_metrics as m;
use crate::visual::palette::VisualPalette;
use crate::visual::panel_screen_state::PanelChildKey;
use crate::visual::render_context::ScenarioContext;
use crate::visual::text::TextRenderer;

use super::super::super::model::{
    HORIZONTAL_PRESET_INDEX, NESTED_PRESET_INDEX, SCROLLBAR_PRESET_INDEX, VERTICAL_PRESET_INDEX,
};

const VERTICAL_ROW_COUNT: usize = 8;
const HORIZONTAL_CARD_COUNT: usize = 5;
const EVEN_ROW_DIVISOR: usize = 2;
const EVEN_ROW_REMAINDER: usize = 0;
const LABEL_X_OFFSET: usize = m::PX_12;
const VERTICAL_LABEL_Y_OFFSET: usize = m::PX_96;
const HORIZONTAL_LABEL_Y_OFFSET: usize = m::PX_84;
const TOGGLE_PANEL_X_OFFSET: usize = m::PX_10;
const TOGGLE_PANEL_Y_OFFSET: usize = m::PX_14;
const TOGGLE_PANEL_WIDTH_INSET: usize = m::PX_26;
const TOGGLE_PANEL_HEIGHT: usize = m::PX_58;
const TOGGLE_KNOB_VISIBLE_X_OFFSET: usize = 102;
const TOGGLE_KNOB_HIDDEN_X_OFFSET: usize = m::PX_28;
const TOGGLE_KNOB_Y_OFFSET: usize = m::PX_28;
const TOGGLE_KNOB_WIDTH: usize = m::PX_52;
const TOGGLE_KNOB_HEIGHT: usize = m::PX_20;
const TOGGLE_KNOB_RADIUS: usize = m::PX_4;
const NESTED_LABEL_Y_OFFSET: usize = m::PX_92;

pub(super) fn draw(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    clip: Rect,
) {
    canvas.fill_rect(clip.x, clip.y, clip.width, clip.height, palette.surface);
    match scenario.preset_index {
        VERTICAL_PRESET_INDEX => draw_vertical(canvas, text, palette, scenario, clip),
        HORIZONTAL_PRESET_INDEX => draw_horizontal(canvas, text, palette, scenario, clip),
        SCROLLBAR_PRESET_INDEX => draw_toggle(canvas, text, palette, scenario, clip),
        NESTED_PRESET_INDEX => draw_nested(canvas, text, palette, scenario, clip),
        _ => draw_vertical(canvas, text, palette, scenario, clip),
    }
}

fn draw_vertical(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    clip: Rect,
) {
    let state = scenario.screen_state.panel.child(PanelChildKey::Preview);
    let offset = (state.scroll_y as usize) % m::PX_96;
    for index in 0..VERTICAL_ROW_COUNT {
        let row_y = (clip.y + m::PX_8 + index * m::PX_32).saturating_sub(offset);
        let color = if index % EVEN_ROW_DIVISOR == EVEN_ROW_REMAINDER {
            palette.panel
        } else {
            palette.code_background
        };
        canvas.fill_rect(
            clip.x + m::PX_10,
            row_y,
            clip.width - m::PX_28,
            m::PX_22,
            color,
        );
        canvas.fill_rect(
            clip.x + m::PX_18,
            row_y + m::PX_6,
            m::PX_56,
            m::PX_6,
            palette.accent,
        );
    }
    text.draw(
        canvas,
        "viewport clips vertical content",
        clip.x + LABEL_X_OFFSET,
        clip.y + VERTICAL_LABEL_Y_OFFSET,
        m::FONT_8,
        palette.muted,
    );
}

fn draw_horizontal(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    clip: Rect,
) {
    let state = scenario.screen_state.panel.child(PanelChildKey::Preview);
    let offset = (state.scroll_x as usize) % m::PX_180;
    let track_x = (clip.x + m::PX_10).saturating_sub(offset);
    canvas.fill_rect(
        track_x,
        clip.y + m::PX_12,
        clip.width + m::PX_180,
        m::PX_52,
        palette.accent,
    );
    for index in 0..HORIZONTAL_CARD_COUNT {
        canvas.fill_rect(
            track_x + m::PX_18 + index * m::PX_72,
            clip.y + m::PX_24,
            m::PX_52,
            m::PX_26,
            palette.panel,
        );
    }
    text.draw(
        canvas,
        "wide surface is clipped inside preview",
        clip.x + LABEL_X_OFFSET,
        clip.y + HORIZONTAL_LABEL_Y_OFFSET,
        m::FONT_8,
        palette.muted,
    );
}

fn draw_toggle(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    clip: Rect,
) {
    let active = scenario.screen_state.panel.active_panel;
    let visible = scenario.screen_state.panel.child(active).scrollbar_visible;
    canvas.fill_rect(
        clip.x + TOGGLE_PANEL_X_OFFSET,
        clip.y + TOGGLE_PANEL_Y_OFFSET,
        clip.width - TOGGLE_PANEL_WIDTH_INSET,
        TOGGLE_PANEL_HEIGHT,
        palette.code_background,
    );
    canvas.stroke_rect(
        clip.x + TOGGLE_PANEL_X_OFFSET,
        clip.y + TOGGLE_PANEL_Y_OFFSET,
        clip.width - TOGGLE_PANEL_WIDTH_INSET,
        TOGGLE_PANEL_HEIGHT,
        palette.border,
    );
    let knob_x = if visible {
        clip.x + TOGGLE_KNOB_VISIBLE_X_OFFSET
    } else {
        clip.x + TOGGLE_KNOB_HIDDEN_X_OFFSET
    };
    canvas.fill_round_rect(
        knob_x,
        clip.y + TOGGLE_KNOB_Y_OFFSET,
        TOGGLE_KNOB_WIDTH,
        TOGGLE_KNOB_HEIGHT,
        TOGGLE_KNOB_RADIUS,
        palette.accent,
    );
    text.draw(
        canvas,
        if visible {
            "scrollbar on"
        } else {
            "scrollbar off"
        },
        clip.x + m::PX_18,
        clip.y + HORIZONTAL_LABEL_Y_OFFSET,
        m::FONT_8,
        palette.muted,
    );
}

fn draw_nested(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    clip: Rect,
) {
    let active = scenario.screen_state.panel.active_panel;
    for (index, panel) in [
        PanelChildKey::Navigation,
        PanelChildKey::Preview,
        PanelChildKey::Details,
    ]
    .into_iter()
    .enumerate()
    {
        let state = scenario.screen_state.panel.child(panel);
        let panel_x = clip.x + m::PX_10 + index * m::PX_80;
        let panel_y = clip.y + m::PX_16 + (state.scroll_y as usize % m::PX_18);
        canvas.fill_rect(panel_x, panel_y, m::PX_66, m::PX_52, palette.panel);
        canvas.stroke_rect(
            panel_x,
            panel_y,
            m::PX_66,
            m::PX_52,
            if panel == active {
                palette.accent
            } else {
                palette.border
            },
        );
        canvas.fill_rect(
            panel_x + m::PX_8,
            panel_y + m::PX_12,
            m::PX_38,
            m::PX_6,
            palette.accent,
        );
    }
    text.draw(
        canvas,
        "child panels keep independent offsets",
        clip.x + LABEL_X_OFFSET,
        clip.y + NESTED_LABEL_Y_OFFSET,
        m::FONT_8,
        palette.muted,
    );
}
