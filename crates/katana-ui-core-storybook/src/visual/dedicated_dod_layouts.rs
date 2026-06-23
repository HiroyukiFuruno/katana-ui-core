use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const ROW_ALIGN_PRESET_INDEX: usize = 1;
const ROW_OVERFLOW_PRESET_INDEX: usize = 2;
const ROW_THEME_PRESET_INDEX: usize = 3;
const ROW_PAGE: &str = "row";
const TRACK_X: usize = m::PX_16;
const TRACK_Y: usize = m::PX_38;
const TRACK_WIDTH: usize = m::PX_252;
const TRACK_HEIGHT: usize = m::PX_38;
const ITEM_Y: usize = m::PX_48;
const ITEM_WIDTH: usize = m::PX_54;
const ITEM_HEIGHT: usize = m::PX_18;
const ITEM_GAP: usize = m::PX_8;
const WIDE_ITEM_WIDTH: usize = m::PX_88;
const LABEL_X: usize = m::PX_284;
const STATUS_Y: usize = m::PX_88;
const STATUS_WIDTH: usize = m::PX_92;
const STATUS_HEIGHT: usize = m::PX_18;
const STATUS_GAP: usize = m::PX_8;
const STATUS_TEXT_X: usize = m::PX_6;
const STATUS_TEXT_Y: usize = m::PX_4;
const ROW_BLOCK_COUNT: usize = 5;
const ROW_LABEL_COUNT: usize = 4;
const STATUS_LABEL_COUNT: usize = 3;

pub(super) fn row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let accent = if scenario.screen_state.layout.is_page(ROW_PAGE)
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
        "Row layout",
        &row_blocks(palette, scenario, accent),
        &row_labels(palette, scenario),
    );
    draw_status(canvas, text, palette, scenario, x, y);
}

fn row_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    accent: u32,
) -> [Block; ROW_BLOCK_COUNT] {
    let item_width = if scenario.preset_index == ROW_OVERFLOW_PRESET_INDEX {
        WIDE_ITEM_WIDTH
    } else {
        ITEM_WIDTH
    };
    let gap = if scenario.preset_index == ROW_THEME_PRESET_INDEX {
        m::PX_18
    } else {
        ITEM_GAP
    };
    [
        Block::outlined(TRACK_X, TRACK_Y, TRACK_WIDTH, TRACK_HEIGHT, palette.surface),
        Block::new(TRACK_X + m::PX_10, ITEM_Y, item_width, ITEM_HEIGHT, accent),
        Block::new(
            TRACK_X + m::PX_10 + item_width + gap,
            ITEM_Y,
            ITEM_WIDTH,
            ITEM_HEIGHT,
            palette.panel,
        ),
        Block::new(
            TRACK_X + m::PX_10 + item_width + gap + ITEM_WIDTH + gap,
            item_y_for_preset(scenario),
            ITEM_WIDTH,
            ITEM_HEIGHT,
            common::TOKEN,
        ),
        Block::new(
            TRACK_X + TRACK_WIDTH - m::PX_22,
            TRACK_Y,
            m::PX_6,
            TRACK_HEIGHT,
            common::WARN,
        ),
    ]
}

fn row_labels(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [TextSpec; ROW_LABEL_COUNT] {
    [
        TextSpec::new(
            LABEL_X,
            m::PX_42,
            m::FONT_9,
            palette.text,
            preset_label(scenario),
        ),
        TextSpec::new(
            LABEL_X,
            m::PX_58,
            m::FONT_8,
            palette.muted,
            "children keep order",
        ),
        TextSpec::new(
            LABEL_X,
            m::PX_74,
            m::FONT_8,
            palette.muted,
            "state/action via settings",
        ),
        TextSpec::new(m::PX_28, m::PX_52, m::FONT_8, palette.background, "A  B  C"),
    ]
}

fn item_y_for_preset(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == ROW_ALIGN_PRESET_INDEX {
        return m::PX_54;
    }
    ITEM_Y
}

fn preset_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        ROW_ALIGN_PRESET_INDEX => "align=center",
        ROW_OVERFLOW_PRESET_INDEX => "overflow=clip",
        ROW_THEME_PRESET_INDEX => "theme gap=18",
        _ => "axis=row gap=8",
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
    if scenario.screen_state.layout.is_page(ROW_PAGE) {
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
