use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const STACK_Z_PRESET_INDEX: usize = 1;
const STACK_OVERLAY_PRESET_INDEX: usize = 2;
const STACK_THEME_PRESET_INDEX: usize = 3;
const STACK_PAGE: &str = "stack";
const STAGE_X: usize = m::PX_16;
const STAGE_Y: usize = m::PX_36;
const STAGE_WIDTH: usize = m::PX_252;
const STAGE_HEIGHT: usize = m::PX_74;
const CARD_X: usize = m::PX_42;
const CARD_Y: usize = m::PX_62;
const CARD_WIDTH: usize = m::PX_112;
const CARD_HEIGHT: usize = m::PX_34;
const DEFAULT_OFFSET: usize = m::PX_10;
const THEME_OFFSET: usize = m::PX_18;
const LABEL_X: usize = m::PX_284;
const STATUS_Y: usize = m::PX_88;
const STATUS_WIDTH: usize = m::PX_92;
const STATUS_HEIGHT: usize = m::PX_18;
const STATUS_GAP: usize = m::PX_8;
const STATUS_TEXT_X: usize = m::PX_6;
const STATUS_TEXT_Y: usize = m::PX_4;
const STACK_BLOCK_COUNT: usize = 6;
const STACK_LABEL_COUNT: usize = 4;
const STATUS_LABEL_COUNT: usize = 3;

pub(super) fn stack(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let accent = if scenario.screen_state.layout.is_page(STACK_PAGE)
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
        "Stack layout",
        &stack_blocks(palette, scenario, accent),
        &stack_labels(palette, scenario),
    );
    draw_status(canvas, text, palette, scenario, x, y);
}

fn stack_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    accent: u32,
) -> [Block; STACK_BLOCK_COUNT] {
    let offset = if scenario.preset_index == STACK_THEME_PRESET_INDEX {
        THEME_OFFSET
    } else {
        DEFAULT_OFFSET
    };
    [
        Block::outlined(STAGE_X, STAGE_Y, STAGE_WIDTH, STAGE_HEIGHT, palette.surface),
        Block::new(CARD_X, CARD_Y, CARD_WIDTH, CARD_HEIGHT, palette.panel),
        Block::new(
            CARD_X + offset,
            CARD_Y - offset,
            CARD_WIDTH,
            CARD_HEIGHT,
            second_layer_color(scenario, accent),
        ),
        Block::new(
            CARD_X + (offset * 2),
            CARD_Y - (offset * 2),
            CARD_WIDTH,
            CARD_HEIGHT,
            top_layer_color(scenario),
        ),
        Block::new(
            CARD_X + m::PX_118,
            CARD_Y - (offset * 2),
            m::PX_8,
            CARD_HEIGHT,
            common::WARN,
        ),
        Block::new(
            STAGE_X + STAGE_WIDTH - m::PX_36,
            STAGE_Y + m::PX_8,
            overlay_marker_width(scenario),
            m::PX_10,
            common::DANGER,
        ),
    ]
}

fn stack_labels(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [TextSpec; STACK_LABEL_COUNT] {
    [
        TextSpec::new(
            LABEL_X,
            m::PX_42,
            m::FONT_9,
            palette.text,
            stack_preset_label(scenario),
        ),
        TextSpec::new(
            LABEL_X,
            m::PX_58,
            m::FONT_8,
            palette.muted,
            "absolute children",
        ),
        TextSpec::new(
            LABEL_X,
            m::PX_74,
            m::FONT_8,
            palette.muted,
            "state/action via settings",
        ),
        TextSpec::new(m::PX_68, m::PX_72, m::FONT_8, palette.background, "z"),
    ]
}

fn second_layer_color(scenario: ScenarioContext<'_>, accent: u32) -> u32 {
    if scenario.preset_index == STACK_Z_PRESET_INDEX {
        return common::TOKEN;
    }
    accent
}

fn top_layer_color(scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == STACK_OVERLAY_PRESET_INDEX {
        return common::PURPLE;
    }
    common::TOKEN
}

fn overlay_marker_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == STACK_OVERLAY_PRESET_INDEX {
        return m::PX_28;
    }
    m::PX_12
}

fn stack_preset_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        STACK_Z_PRESET_INDEX => "z order swap",
        STACK_OVERLAY_PRESET_INDEX => "overlay marker",
        STACK_THEME_PRESET_INDEX => "theme offset=18",
        _ => "stack offset=10",
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
    if scenario.screen_state.layout.is_page(STACK_PAGE) {
        return [
            scenario.screen_state.last_action,
            scenario.screen_state.last_event,
            scenario.screen_state.state_label,
        ];
    }
    if scenario.screen_state.has_settings_override() {
        return ["action stack", "event z-order", "state override"];
    }
    ["action ready", "event ready", "state idle"]
}
