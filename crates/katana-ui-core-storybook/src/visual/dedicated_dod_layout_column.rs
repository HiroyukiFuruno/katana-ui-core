use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const COLUMN_ALIGN_PRESET_INDEX: usize = 1;
const COLUMN_OVERFLOW_PRESET_INDEX: usize = 2;
const COLUMN_THEME_PRESET_INDEX: usize = 3;
const COLUMN_PAGE: &str = "column";
const TRACK_X: usize = m::PX_16;
const TRACK_Y: usize = m::PX_38;
const TRACK_WIDTH: usize = m::PX_252;
const TRACK_HEIGHT: usize = m::PX_38;
const COLUMN_ITEM_X: usize = m::PX_40;
const ITEM_WIDTH: usize = m::PX_54;
const ITEM_HEIGHT: usize = m::PX_18;
const ITEM_GAP: usize = m::PX_8;
const TALL_ITEM_HEIGHT: usize = m::PX_26;
const LABEL_X: usize = m::PX_284;
const STATUS_Y: usize = m::PX_88;
const STATUS_WIDTH: usize = m::PX_92;
const STATUS_HEIGHT: usize = m::PX_18;
const STATUS_GAP: usize = m::PX_8;
const STATUS_TEXT_X: usize = m::PX_6;
const STATUS_TEXT_Y: usize = m::PX_4;
const COLUMN_BLOCK_COUNT: usize = 5;
const COLUMN_LABEL_COUNT: usize = 4;
const STATUS_LABEL_COUNT: usize = 3;

pub(super) fn column(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let accent = if scenario.screen_state.layout.is_page(COLUMN_PAGE)
        || scenario.screen_state.has_settings_override()
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
        "Column layout",
        &column_blocks(palette, scenario, accent),
        &column_labels(palette, scenario),
    );
    draw_status(canvas, text, palette, scenario, x, y);
}

fn column_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    accent: u32,
) -> [Block; COLUMN_BLOCK_COUNT] {
    let item_height = if scenario.preset_index == COLUMN_OVERFLOW_PRESET_INDEX {
        TALL_ITEM_HEIGHT
    } else {
        ITEM_HEIGHT
    };
    let gap = if scenario.preset_index == COLUMN_THEME_PRESET_INDEX {
        m::PX_14
    } else {
        ITEM_GAP
    };
    [
        Block::outlined(
            TRACK_X,
            TRACK_Y,
            TRACK_WIDTH,
            TRACK_HEIGHT + m::PX_34,
            palette.surface,
        ),
        Block::new(
            COLUMN_ITEM_X,
            TRACK_Y + m::PX_8,
            ITEM_WIDTH,
            item_height,
            accent,
        ),
        Block::new(
            COLUMN_ITEM_X,
            TRACK_Y + m::PX_8 + item_height + gap,
            ITEM_WIDTH,
            ITEM_HEIGHT,
            palette.panel,
        ),
        Block::new(
            column_item_x_for_preset(scenario),
            TRACK_Y + m::PX_8 + item_height + gap + ITEM_HEIGHT + gap,
            ITEM_WIDTH,
            ITEM_HEIGHT,
            common::TOKEN,
        ),
        Block::new(
            TRACK_X,
            TRACK_Y + TRACK_HEIGHT + m::PX_28,
            TRACK_WIDTH,
            m::PX_4,
            common::WARN,
        ),
    ]
}

fn column_labels(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [TextSpec; COLUMN_LABEL_COUNT] {
    [
        TextSpec::new(
            LABEL_X,
            m::PX_42,
            m::FONT_9,
            palette.text,
            column_preset_label(scenario),
        ),
        TextSpec::new(
            LABEL_X,
            m::PX_58,
            m::FONT_8,
            palette.muted,
            "vertical order",
        ),
        TextSpec::new(
            LABEL_X,
            m::PX_74,
            m::FONT_8,
            palette.muted,
            "state/action via settings",
        ),
        TextSpec::new(m::PX_52, m::PX_48, m::FONT_8, palette.background, "A"),
    ]
}

fn column_item_x_for_preset(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == COLUMN_ALIGN_PRESET_INDEX {
        return COLUMN_ITEM_X + m::PX_24;
    }
    COLUMN_ITEM_X
}

fn column_preset_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        COLUMN_ALIGN_PRESET_INDEX => "align=center",
        COLUMN_OVERFLOW_PRESET_INDEX => "overflow=clip",
        COLUMN_THEME_PRESET_INDEX => "theme gap=14",
        _ => "axis=column gap=8",
    }
}

fn draw_status(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    for (index, label) in status_labels(scenario).into_iter().enumerate() {
        let row_x = x + TRACK_X + index * (STATUS_WIDTH + STATUS_GAP);
        canvas.fill_rect(
            row_x,
            y + STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        );
        canvas.stroke_rect(
            row_x,
            y + STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.border,
        );
        text.draw(
            canvas,
            label,
            row_x + STATUS_TEXT_X,
            y + STATUS_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
        );
    }
}

fn status_labels(scenario: ScenarioContext<'_>) -> [&'static str; STATUS_LABEL_COUNT] {
    if scenario.screen_state.layout.is_page(COLUMN_PAGE) {
        return [
            scenario.screen_state.last_action,
            scenario.screen_state.last_event,
            scenario.screen_state.state_label,
        ];
    }
    if scenario.screen_state.has_settings_override() {
        return ["action layout", "event changed", "state override"];
    }
    ["action ready", "event ready", "state idle"]
}

#[cfg(test)]
mod tests {
    use super::status_labels;
    use crate::visual::render_context::ScenarioContext;
    use crate::visual::screen_state::StorybookScreenState;

    #[test]
    fn settings_override_is_reported_outside_the_active_page() {
        let state = StorybookScreenState {
            settings_revision: 1,
            ..StorybookScreenState::default()
        };

        assert_eq!(
            ["action layout", "event changed", "state override"],
            status_labels(ScenarioContext::for_test("other", 0, &state))
        );
    }
}
