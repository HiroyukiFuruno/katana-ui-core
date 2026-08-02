use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect};
use super::dedicated_dod_metrics as m;
use super::dedicated_dod_molecule_split_pane_labels as split_pane_labels;
use super::dedicated_dod_molecule_split_pane_labels::{
    ALIGN_PRESET_INDEX, AXIS_PRESET_INDEX, DEFAULT_RATIO_PERCENT, GAP_PRESET_INDEX,
    HANDLE_PRESET_INDEX, KEYBOARD_PRESET_INDEX, MAX_PRESET_INDEX, MIN_PRESET_INDEX,
    OVERFLOW_PRESET_INDEX, RATIO_PRESET_INDEX, RESET_PRESET_INDEX,
};
use super::layout_metrics::LayoutRect;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PANEL_X: usize = m::PX_18;
const PANEL_Y: usize = m::PX_36;
const PANEL_WIDTH: usize = m::PX_252;
const PANEL_HEIGHT: usize = m::PX_56;
const LEFT_WIDTH: usize = m::PX_92;
const RESET_LEFT_WIDTH: usize = m::PX_116;
const CLAMP_LEFT_WIDTH: usize = m::PX_72;
const KEYBOARD_LEFT_WIDTH: usize = m::PX_128;
const HANDLE_HEIGHT: usize = m::PX_56;
const VERTICAL_HANDLE_Y: usize = m::PX_62;
const SPLIT_BLOCK_COUNT: usize = 6;

pub(super) fn split_pane(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let accent = if scenario.screen_state.has_widget_action()
        || scenario.screen_state.has_settings_override()
        || scenario.screen_state.split_pane.focused()
        || scenario.screen_state.split_pane.hovered()
        || scenario.screen_state.split_pane.dragging()
        || scenario.screen_state.split_pane.resized()
    {
        common::SUCCESS
    } else {
        palette.accent
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "SplitPane",
        &split_blocks(palette, scenario, accent),
        &split_pane_labels::split_labels(palette, scenario),
    );
    split_pane_labels::draw_status(canvas, text, palette, scenario, x, y);
}

fn split_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    accent: u32,
) -> [Block; SPLIT_BLOCK_COUNT] {
    if scenario.preset_index == AXIS_PRESET_INDEX {
        return vertical_blocks(palette, accent);
    }
    horizontal_blocks(palette, scenario, accent)
}

fn horizontal_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    accent: u32,
) -> [Block; SPLIT_BLOCK_COUNT] {
    let left_width = left_width_for(scenario);
    let handle_width = handle_width_for(scenario);
    let right_x = PANEL_X + left_width + handle_width;
    [
        Block::outlined(PANEL_X, PANEL_Y, left_width, PANEL_HEIGHT, palette.surface),
        Block::new(
            PANEL_X + left_width,
            PANEL_Y,
            handle_width,
            HANDLE_HEIGHT,
            accent,
        ),
        Block::outlined(
            right_x,
            PANEL_Y,
            PANEL_WIDTH - left_width - handle_width,
            PANEL_HEIGHT,
            palette.surface,
        ),
        Block::new(
            PANEL_X + m::PX_10,
            PANEL_Y + m::PX_18,
            m::PX_48,
            m::PX_8,
            palette.panel,
        ),
        Block::new(
            right_x + m::PX_12,
            PANEL_Y + m::PX_18,
            m::PX_58,
            m::PX_8,
            common::TOKEN,
        ),
        Block::new(
            PANEL_X + left_width.saturating_sub(m::PX_12),
            PANEL_Y + m::PX_8,
            clamp_marker_width(scenario),
            PANEL_HEIGHT - m::PX_16,
            common::DANGER,
        ),
    ]
}

fn vertical_blocks(palette: &VisualPalette, accent: u32) -> [Block; SPLIT_BLOCK_COUNT] {
    [
        Block::outlined(PANEL_X, PANEL_Y, PANEL_WIDTH, m::PX_26, palette.surface),
        Block::new(PANEL_X, VERTICAL_HANDLE_Y, PANEL_WIDTH, m::PX_6, accent),
        Block::outlined(
            PANEL_X,
            VERTICAL_HANDLE_Y + m::PX_6,
            PANEL_WIDTH,
            m::PX_24,
            palette.surface,
        ),
        Block::new(
            PANEL_X + m::PX_18,
            PANEL_Y + m::PX_10,
            m::PX_64,
            m::PX_8,
            palette.panel,
        ),
        Block::new(
            PANEL_X + m::PX_18,
            VERTICAL_HANDLE_Y + m::PX_14,
            m::PX_72,
            m::PX_8,
            common::TOKEN,
        ),
        Block::new(
            PANEL_X + PANEL_WIDTH - m::PX_28,
            PANEL_Y + m::PX_8,
            m::PX_12,
            m::PX_40,
            common::DANGER,
        ),
    ]
}

fn left_width_for(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.split_pane.dragging()
        || scenario.screen_state.split_pane.resized()
        || scenario.screen_state.split_pane.ratio_percent() != DEFAULT_RATIO_PERCENT
    {
        return PANEL_WIDTH * usize::from(scenario.screen_state.split_pane.ratio_percent()) / 100;
    }
    if scenario.preset_index == GAP_PRESET_INDEX {
        return m::PX_84;
    }
    if scenario.preset_index == ALIGN_PRESET_INDEX {
        return m::PX_104;
    }
    if scenario.preset_index == OVERFLOW_PRESET_INDEX {
        return m::PX_78;
    }
    if scenario.preset_index == RATIO_PRESET_INDEX {
        return m::PX_142;
    }
    if scenario.preset_index == MIN_PRESET_INDEX {
        return CLAMP_LEFT_WIDTH;
    }
    if scenario.preset_index == MAX_PRESET_INDEX {
        return m::PX_160;
    }
    if scenario.preset_index == KEYBOARD_PRESET_INDEX {
        return KEYBOARD_LEFT_WIDTH;
    }
    if scenario.preset_index == RESET_PRESET_INDEX || scenario.screen_state.has_widget_action() {
        return RESET_LEFT_WIDTH;
    }
    LEFT_WIDTH
}

fn handle_width_for(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.split_pane.focused()
        || scenario.screen_state.split_pane.hovered()
        || scenario.screen_state.split_pane.dragging()
    {
        return m::PX_10;
    }
    if scenario.preset_index == GAP_PRESET_INDEX {
        return m::PX_14;
    }
    if scenario.preset_index == HANDLE_PRESET_INDEX {
        return m::PX_12;
    }
    m::PX_6
}

pub(super) fn handle_drag_rect(origin_x: usize, origin_y: usize) -> LayoutRect {
    LayoutRect::new(
        origin_x + PANEL_X + LEFT_WIDTH,
        origin_y + PANEL_Y,
        m::PX_10,
        HANDLE_HEIGHT,
    )
}

pub(super) fn resize_handle_rect(origin_x: usize, origin_y: usize) -> LayoutRect {
    LayoutRect::new(
        origin_x + PANEL_X + PANEL_WIDTH - m::PX_12,
        origin_y + PANEL_Y + PANEL_HEIGHT - m::PX_12,
        m::PX_12,
        m::PX_12,
    )
}

fn clamp_marker_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.split_pane.resized() {
        return m::PX_20;
    }
    if scenario.preset_index == MIN_PRESET_INDEX {
        return m::PX_22;
    }
    if scenario.preset_index == MAX_PRESET_INDEX {
        return m::PX_18;
    }
    if scenario.preset_index == OVERFLOW_PRESET_INDEX {
        return m::PX_16;
    }
    if scenario.preset_index == KEYBOARD_PRESET_INDEX {
        return m::PX_14;
    }
    m::PX_8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visual::screen_state::StorybookScreenState;
    use crate::visual::window_interaction::split_pane_operation::SplitPaneStoryAction;

    #[test]
    fn live_drag_and_resize_state_drive_split_geometry() {
        let mut dragged = StorybookScreenState::default();
        dragged.register_split_pane_action(SplitPaneStoryAction::Drag);
        let drag_scenario = ScenarioContext::for_test("split-pane", 0, &dragged);
        assert_eq!(
            PANEL_WIDTH * usize::from(dragged.split_pane.ratio_percent()) / 100,
            left_width_for(drag_scenario)
        );
        assert_eq!(m::PX_10, handle_width_for(drag_scenario));

        let mut resized = StorybookScreenState::default();
        resized.register_split_pane_action(SplitPaneStoryAction::Resize);
        assert_eq!(
            m::PX_20,
            clamp_marker_width(ScenarioContext::for_test("split-pane", 0, &resized))
        );
    }
}
