use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::layout_metrics::LayoutRect;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PANEL_X: usize = 26;
const PANEL_Y: usize = 26;
const PANEL_WIDTH: usize = 210;
const PANEL_HEIGHT: usize = 86;
const ROW_X: usize = PANEL_X + 10;
const FIRST_ROW_Y: usize = PANEL_Y + 12;
const ROW_WIDTH: usize = PANEL_WIDTH - 20;
const ROW_HEIGHT: usize = 22;
const ROW_GAP: usize = 8;
const ROW_TEXT_X_OFFSET: usize = 10;
const ROW_TEXT_Y_OFFSET: usize = 7;
const MENU_BLOCK_COUNT: usize = 3;
const SHORTCUT_PRESET_INDEX: usize = 1;
const DISABLED_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;
const STATUS_X: usize = PANEL_X + PANEL_WIDTH + 18;
const STATUS_Y: usize = FIRST_ROW_Y + 4;

pub(super) fn menu(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "Menu panel",
        &menu_blocks(palette, scenario),
        &[
            TextSpec::new(
                ROW_X + ROW_TEXT_X_OFFSET,
                FIRST_ROW_Y + ROW_TEXT_Y_OFFSET,
                m::FONT_8,
                palette.text,
                first_row_label(scenario),
            ),
            TextSpec::new(
                ROW_X + ROW_TEXT_X_OFFSET,
                FIRST_ROW_Y + ROW_HEIGHT + ROW_GAP + ROW_TEXT_Y_OFFSET,
                m::FONT_8,
                second_row_text_color(palette, scenario),
                second_row_label(scenario),
            ),
            TextSpec::new(
                STATUS_X,
                STATUS_Y,
                m::FONT_8,
                palette.muted,
                menu_status_label(scenario),
            ),
        ],
    );
    draw_interaction_border(canvas, palette, scenario, x, y);
}

fn draw_interaction_border(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let highlighted =
        scenario.screen_state.preview_hovered || scenario.screen_state.is_button_focused();
    if !highlighted {
        return;
    }
    canvas.stroke_rect(
        x + ROW_X,
        y + FIRST_ROW_Y,
        ROW_WIDTH,
        ROW_HEIGHT,
        palette.hover_border,
    );
}

fn menu_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [Block; MENU_BLOCK_COUNT] {
    [
        Block::outlined(
            PANEL_X,
            PANEL_Y,
            PANEL_WIDTH,
            PANEL_HEIGHT,
            panel_fill(palette, scenario),
        ),
        Block::outlined(
            ROW_X,
            FIRST_ROW_Y,
            ROW_WIDTH,
            ROW_HEIGHT,
            first_row_fill(palette, scenario),
        ),
        Block::outlined(
            ROW_X,
            FIRST_ROW_Y + ROW_HEIGHT + ROW_GAP,
            ROW_WIDTH,
            ROW_HEIGHT,
            second_row_fill(palette, scenario),
        ),
    ]
}

fn panel_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == THEME_PRESET_INDEX {
        return palette.background;
    }
    palette.panel
}

fn first_row_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.selection.select_selected_index == Some(0)
        || scenario.screen_state.has_widget_action()
        || scenario.screen_state.has_settings_override()
        || scenario.preset_index == SHORTCUT_PRESET_INDEX
    {
        return palette.accent;
    }
    palette.surface
}

fn second_row_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DISABLED_PRESET_INDEX {
        return palette.panel;
    }
    if scenario.screen_state.selection.select_selected_index == Some(1) {
        return palette.accent;
    }
    if scenario.preset_index == THEME_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.surface
}

fn second_row_text_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DISABLED_PRESET_INDEX {
        return palette.muted;
    }
    palette.text
}

fn first_row_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == SHORTCUT_PRESET_INDEX {
        return "Open Cmd+O";
    }
    "Open"
}

fn second_row_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == DISABLED_PRESET_INDEX {
        return "Disabled";
    }
    "Close"
}

fn menu_status_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.selection.select_open {
        return "callback: menu_opened";
    }
    if scenario.screen_state.selection.select_selected_index == Some(0) {
        return "callback: open";
    }
    if scenario.screen_state.selection.select_selected_index == Some(1) {
        return "callback: close";
    }
    "state: closed"
}

pub(super) fn first_row_rect(component: LayoutRect) -> LayoutRect {
    LayoutRect::new(
        component.x + ROW_X,
        component.y + FIRST_ROW_Y,
        ROW_WIDTH,
        ROW_HEIGHT,
    )
}

pub(super) fn second_row_rect(component: LayoutRect) -> LayoutRect {
    LayoutRect::new(
        component.x + ROW_X,
        component.y + FIRST_ROW_Y + ROW_HEIGHT + ROW_GAP,
        ROW_WIDTH,
        ROW_HEIGHT,
    )
}

#[cfg(test)]
mod tests {
    use super::menu_status_label;
    use crate::visual::render_context::ScenarioContext;
    use crate::visual::screen_state::StorybookScreenState;

    #[test]
    fn selected_first_menu_row_reports_open_callback() {
        let mut state = StorybookScreenState::default();
        state.selection.select_selected_index = Some(0);

        assert_eq!(
            "callback: open",
            menu_status_label(ScenarioContext::for_test("menu", 0, &state))
        );
    }
}
