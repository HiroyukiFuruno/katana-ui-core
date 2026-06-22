use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const BASELINE_PRESET_INDEX: usize = 1;
const MIXED_TEXT_PRESET_INDEX: usize = 2;
const THEME_ALIGN_PRESET_INDEX: usize = 3;
const STAGE_X: usize = m::PX_16;
const STAGE_Y: usize = m::PX_36;
const STAGE_WIDTH: usize = m::PX_252;
const STAGE_HEIGHT: usize = m::PX_74;
const CENTER_LINE_X: usize = m::PX_142;
const CENTER_LINE_Y: usize = m::PX_72;
const CHILD_WIDTH: usize = m::PX_78;
const MIXED_CHILD_WIDTH: usize = m::PX_112;
const CHILD_HEIGHT: usize = m::PX_24;
const CHILD_Y: usize = m::PX_60;
const BASELINE_CHILD_Y: usize = m::PX_66;
const LABEL_X: usize = m::PX_284;
const STATUS_Y: usize = m::PX_88;
const STATUS_WIDTH: usize = m::PX_92;
const STATUS_HEIGHT: usize = m::PX_18;
const STATUS_GAP: usize = m::PX_8;
const STATUS_TEXT_X: usize = m::PX_6;
const STATUS_TEXT_Y: usize = m::PX_4;
const ALIGN_BLOCK_COUNT: usize = 6;
const ALIGN_LABEL_COUNT: usize = 4;
const STATUS_LABEL_COUNT: usize = 3;

pub(super) fn align_center(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let accent = if scenario.screen_state.state_label == "keyboard=center" {
        common::TOKEN
    } else if scenario.screen_state.has_settings_override()
        || scenario.screen_state.layout.is_page("align-center")
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
        "AlignCenter layout",
        &align_blocks(palette, scenario, accent),
        &align_labels(palette, scenario),
    );
    draw_status(canvas, text, palette, scenario, x, y);
}

fn align_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    accent: u32,
) -> [Block; ALIGN_BLOCK_COUNT] {
    let child_width = child_width_for(scenario);
    let child_x = STAGE_X + (STAGE_WIDTH - child_width) / 2;
    [
        Block::outlined(STAGE_X, STAGE_Y, STAGE_WIDTH, STAGE_HEIGHT, palette.surface),
        Block::new(
            CENTER_LINE_X,
            STAGE_Y + m::PX_6,
            m::PX_2,
            STAGE_HEIGHT - m::PX_12,
            common::WARN,
        ),
        Block::new(
            STAGE_X + m::PX_12,
            CENTER_LINE_Y,
            STAGE_WIDTH - m::PX_24,
            m::PX_2,
            common::WARN,
        ),
        Block::new(
            child_x,
            child_y_for(scenario),
            child_width,
            CHILD_HEIGHT,
            accent,
        ),
        Block::new(
            child_x + m::PX_12,
            child_y_for(scenario) + m::PX_8,
            child_width - m::PX_24,
            m::PX_4,
            common::TOKEN,
        ),
        Block::new(
            child_x + child_width - m::PX_14,
            child_y_for(scenario) + m::PX_4,
            theme_marker_width(scenario),
            CHILD_HEIGHT - m::PX_8,
            common::PURPLE,
        ),
    ]
}

fn align_labels(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [TextSpec; ALIGN_LABEL_COUNT] {
    [
        TextSpec::new(
            LABEL_X,
            m::PX_42,
            m::FONT_9,
            palette.text,
            align_preset_label(scenario),
        ),
        TextSpec::new(
            LABEL_X,
            m::PX_58,
            m::FONT_8,
            palette.muted,
            "centered child",
        ),
        TextSpec::new(
            LABEL_X,
            m::PX_74,
            m::FONT_8,
            palette.muted,
            "state/action via settings",
        ),
        TextSpec::new(m::PX_108, m::PX_68, m::FONT_8, palette.background, "center"),
    ]
}

fn child_width_for(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.layout.is_page("align-center")
        && scenario.screen_state.layout.resized()
    {
        return MIXED_CHILD_WIDTH;
    }
    if scenario.preset_index == MIXED_TEXT_PRESET_INDEX {
        return MIXED_CHILD_WIDTH;
    }
    CHILD_WIDTH
}

fn child_y_for(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == BASELINE_PRESET_INDEX {
        return BASELINE_CHILD_Y;
    }
    CHILD_Y
}

fn theme_marker_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.state_label == "keyboard=center" {
        return m::PX_26;
    }
    if scenario.screen_state.layout.is_page("align-center")
        && (scenario.screen_state.layout.hovered() || scenario.screen_state.layout.focused())
    {
        return m::PX_18;
    }
    if scenario.preset_index == THEME_ALIGN_PRESET_INDEX {
        return m::PX_22;
    }
    m::PX_8
}

fn align_preset_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        BASELINE_PRESET_INDEX => "baseline center",
        MIXED_TEXT_PRESET_INDEX => "mixed text width",
        THEME_ALIGN_PRESET_INDEX => "theme marker",
        _ => "center box",
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
    if scenario.screen_state.layout.is_page("align-center")
        && scenario.screen_state.layout.hovered()
    {
        return ["action hover", "event hover", "state center"];
    }
    if scenario.screen_state.layout.is_page("align-center")
        && scenario.screen_state.layout.focused()
    {
        return ["action focus", "event focus", "state center"];
    }
    if scenario.screen_state.layout.is_page("align-center")
        && scenario.screen_state.layout.resized()
    {
        return ["action resize", "event layout", "state center"];
    }
    if scenario.screen_state.layout.is_page("align-center") {
        return ["action key", "event center", "state center"];
    }
    if scenario.screen_state.has_settings_override() {
        return ["action align", "event center", "state override"];
    }
    ["action ready", "event ready", "state idle"]
}
