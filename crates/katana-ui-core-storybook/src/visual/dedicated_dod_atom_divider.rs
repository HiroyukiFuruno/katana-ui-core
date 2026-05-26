use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const VERTICAL_PRESET_INDEX: usize = 1;
const INSET_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;
const STAGE_X: usize = m::PX_16;
const STAGE_Y: usize = m::PX_36;
const STAGE_WIDTH: usize = m::PX_252;
const STAGE_HEIGHT: usize = m::PX_68;
const LINE_X: usize = m::PX_38;
const LINE_Y: usize = m::PX_66;
const LINE_WIDTH: usize = m::PX_190;
const LINE_HEIGHT: usize = m::PX_2;
const VERTICAL_LINE_X: usize = m::PX_142;
const VERTICAL_LINE_HEIGHT: usize = m::PX_50;
const INSET_X: usize = m::PX_70;
const INSET_WIDTH: usize = m::PX_128;
const LABEL_X: usize = m::PX_284;
const STATUS_Y: usize = m::PX_88;
const STATUS_WIDTH: usize = m::PX_92;
const STATUS_HEIGHT: usize = m::PX_18;
const STATUS_GAP: usize = m::PX_8;
const STATUS_TEXT_X: usize = m::PX_6;
const STATUS_TEXT_Y: usize = m::PX_4;
const DIVIDER_BLOCK_COUNT: usize = 5;
const DIVIDER_LABEL_COUNT: usize = 4;
const STATUS_LABEL_COUNT: usize = 3;

pub(super) fn divider(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let line_color = if scenario.screen_state.has_settings_override() {
        common::SUCCESS
    } else {
        line_color_for(palette, scenario)
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "Divider",
        &divider_blocks(palette, scenario, line_color),
        &divider_labels(palette, scenario),
    );
    draw_status(canvas, text, palette, scenario, x, y);
}

fn divider_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    line_color: u32,
) -> [Block; DIVIDER_BLOCK_COUNT] {
    let rect = line_rect_for(scenario);
    [
        Block::outlined(STAGE_X, STAGE_Y, STAGE_WIDTH, STAGE_HEIGHT, palette.surface),
        Block::new(rect.x, rect.y, rect.width, rect.height, line_color),
        Block::new(
            STAGE_X + m::PX_16,
            STAGE_Y + m::PX_14,
            m::PX_42,
            m::PX_8,
            palette.panel,
        ),
        Block::new(
            STAGE_X + STAGE_WIDTH - m::PX_58,
            STAGE_Y + STAGE_HEIGHT - m::PX_20,
            theme_marker_width(scenario),
            m::PX_6,
            common::TOKEN,
        ),
        Block::new(
            STAGE_X + m::PX_16,
            STAGE_Y + STAGE_HEIGHT - m::PX_18,
            m::PX_38,
            m::PX_4,
            common::WARN,
        ),
    ]
}

fn divider_labels(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [TextSpec; DIVIDER_LABEL_COUNT] {
    [
        TextSpec::new(
            LABEL_X,
            m::PX_42,
            m::FONT_9,
            palette.text,
            divider_preset_label(scenario),
        ),
        TextSpec::new(
            LABEL_X,
            m::PX_58,
            m::FONT_8,
            palette.muted,
            "separator line",
        ),
        TextSpec::new(
            LABEL_X,
            m::PX_74,
            m::FONT_8,
            palette.muted,
            "state/action via settings",
        ),
        TextSpec::new(m::PX_48, m::PX_52, m::FONT_8, palette.text, "A"),
    ]
}

fn line_rect_for(scenario: ScenarioContext<'_>) -> Rect {
    if scenario.preset_index == VERTICAL_PRESET_INDEX {
        return Rect::new(
            VERTICAL_LINE_X,
            LINE_Y - m::PX_22,
            LINE_HEIGHT,
            VERTICAL_LINE_HEIGHT,
        );
    }
    if scenario.preset_index == INSET_PRESET_INDEX {
        return Rect::new(INSET_X, LINE_Y, INSET_WIDTH, LINE_HEIGHT);
    }
    Rect::new(LINE_X, LINE_Y, LINE_WIDTH, LINE_HEIGHT)
}

fn line_color_for(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == THEME_PRESET_INDEX {
        return palette.accent;
    }
    palette.border
}

fn theme_marker_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == THEME_PRESET_INDEX {
        return m::PX_42;
    }
    m::PX_14
}

fn divider_preset_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        VERTICAL_PRESET_INDEX => "vertical divider",
        INSET_PRESET_INDEX => "inset divider",
        THEME_PRESET_INDEX => "theme line",
        _ => "horizontal divider",
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
        let row_x = x + STAGE_X + index * (STATUS_WIDTH + STATUS_GAP);
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
    if scenario.screen_state.has_settings_override() {
        return ["action divider", "event size", "state override"];
    }
    ["action ready", "event ready", "state idle"]
}
