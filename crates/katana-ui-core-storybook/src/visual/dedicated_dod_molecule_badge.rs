use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, ChipSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PASSIVE_STATUS_PRESET: usize = 1;
const SMALL_SIZE_PRESET: usize = 2;
const THEME_BADGE_PRESET: usize = 3;
const STATUS_X: usize = 18;
const STATUS_Y: usize = 96;
const STATUS_WIDTH: usize = 96;
const STATUS_HEIGHT: usize = 18;
const STATUS_GAP: usize = 8;
const STATUS_TEXT_X: usize = 7;
const STATUS_TEXT_Y: usize = 5;
const BADGE_CHIP_COUNT: usize = 6;

pub(super) fn badge(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "Badge tone grid");
    common::draw_chips(canvas, text, palette, x, y, &chips(palette, scenario));
    draw_status(canvas, text, palette, scenario, x, y);
}

fn chips(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [ChipSpec; BADGE_CHIP_COUNT] {
    let compact = scenario.preset_index == SMALL_SIZE_PRESET;
    let width = if compact { m::PX_80 } else { m::PX_94 };
    let height = if compact { m::PX_18 } else { m::PX_20 };
    let second_x = if compact { m::PX_104 } else { m::PX_118 };
    let third_x = if compact { m::PX_190 } else { m::PX_222 };
    let second_y = if compact { m::PX_60 } else { m::PX_64 };

    [
        ChipSpec::new(
            m::PX_14,
            m::PX_36,
            width,
            height,
            "neutral",
            neutral_fill(palette, scenario),
        ),
        ChipSpec::new(
            second_x,
            m::PX_36,
            width,
            height,
            "accent",
            accent_fill(palette, scenario),
        ),
        ChipSpec::new(third_x, m::PX_36, width, height, "danger", common::DANGER),
        ChipSpec::new(m::PX_14, second_y, width, height, "warning", common::WARN),
        ChipSpec::new(
            second_x,
            second_y,
            width,
            height,
            "success",
            common::SUCCESS,
        ),
        ChipSpec::new(
            third_x,
            second_y,
            width,
            height,
            "● icon",
            icon_fill(palette, scenario),
        ),
    ]
}

fn neutral_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() || scenario.preset_index == PASSIVE_STATUS_PRESET {
        return common::DANGER;
    }
    if scenario.preset_index == THEME_BADGE_PRESET {
        return palette.surface;
    }
    palette.panel
}

fn accent_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == THEME_BADGE_PRESET {
        return common::TOKEN;
    }
    palette.accent
}

fn icon_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == THEME_BADGE_PRESET {
        return palette.text;
    }
    common::PURPLE
}

fn draw_status(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let rows = [
        action_label(scenario),
        event_label(scenario),
        state_label(scenario),
    ];
    for (index, row) in rows.into_iter().enumerate() {
        let row_x = x + STATUS_X + index * (STATUS_WIDTH + STATUS_GAP);
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
            row,
            row_x + STATUS_TEXT_X,
            y + STATUS_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
        );
    }
}

fn action_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "badge_passive";
    }
    scenario.screen_state.last_action
}

fn event_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_event == "none" {
        return "event none";
    }
    scenario.screen_state.last_event
}

fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == PASSIVE_STATUS_PRESET {
        return "use Chip";
    }
    if scenario.screen_state.state_label == "idle" {
        return "state=ready";
    }
    scenario.screen_state.state_label
}
