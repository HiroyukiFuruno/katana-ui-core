use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, ChipSpec};
use super::dedicated_dod_metrics as m;
use super::layout_metrics::LayoutRect;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PASSIVE_STATUS_PRESET: usize = 1;
const SMALL_SIZE_PRESET: usize = 2;
const THEME_BADGE_PRESET: usize = 3;
const LEADING_ICON_PRESET: usize = 4;
const FILLED_VARIANT_PRESET: usize = 5;
const STATUS_X: usize = 18;
const STATUS_Y: usize = 96;
const STATUS_WIDTH: usize = 96;
const STATUS_HEIGHT: usize = 18;
const STATUS_GAP: usize = 8;
const STATUS_TEXT_X: usize = 7;
const STATUS_TEXT_Y: usize = 5;
const BADGE_CHIP_COUNT: usize = 6;
const NEUTRAL_CHIP_INDEX: usize = 0;
const ACCENT_CHIP_INDEX: usize = 1;
const DANGER_CHIP_INDEX: usize = 2;
const WARNING_CHIP_INDEX: usize = 3;
const SUCCESS_CHIP_INDEX: usize = 4;
const ICON_CHIP_INDEX: usize = 5;

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
    chip_specs(palette, scenario)
}

fn chip_specs(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [ChipSpec; BADGE_CHIP_COUNT] {
    let rects = chip_rects(scenario.preset_index);
    let leading_icon = if scenario.preset_index == LEADING_ICON_PRESET {
        "◆ icon"
    } else {
        "● icon"
    };

    [
        ChipSpec::new(
            rects[NEUTRAL_CHIP_INDEX].x,
            rects[NEUTRAL_CHIP_INDEX].y,
            rects[NEUTRAL_CHIP_INDEX].width,
            rects[NEUTRAL_CHIP_INDEX].height,
            "neutral",
            neutral_fill(palette, scenario),
        ),
        ChipSpec::new(
            rects[ACCENT_CHIP_INDEX].x,
            rects[ACCENT_CHIP_INDEX].y,
            rects[ACCENT_CHIP_INDEX].width,
            rects[ACCENT_CHIP_INDEX].height,
            "accent",
            accent_fill(palette, scenario),
        ),
        ChipSpec::new(
            rects[DANGER_CHIP_INDEX].x,
            rects[DANGER_CHIP_INDEX].y,
            rects[DANGER_CHIP_INDEX].width,
            rects[DANGER_CHIP_INDEX].height,
            "danger",
            danger_fill(scenario),
        ),
        ChipSpec::new(
            rects[WARNING_CHIP_INDEX].x,
            rects[WARNING_CHIP_INDEX].y,
            rects[WARNING_CHIP_INDEX].width,
            rects[WARNING_CHIP_INDEX].height,
            "warning",
            common::WARN,
        ),
        ChipSpec::new(
            rects[SUCCESS_CHIP_INDEX].x,
            rects[SUCCESS_CHIP_INDEX].y,
            rects[SUCCESS_CHIP_INDEX].width,
            rects[SUCCESS_CHIP_INDEX].height,
            "success",
            common::SUCCESS,
        ),
        ChipSpec::new(
            rects[ICON_CHIP_INDEX].x,
            rects[ICON_CHIP_INDEX].y,
            rects[ICON_CHIP_INDEX].width,
            rects[ICON_CHIP_INDEX].height,
            leading_icon,
            icon_fill(palette, scenario),
        ),
    ]
}

fn chip_rects(preset_index: usize) -> [LayoutRect; BADGE_CHIP_COUNT] {
    let compact = preset_index == SMALL_SIZE_PRESET;
    let width = if compact { m::PX_80 } else { m::PX_94 };
    let height = if compact { m::PX_18 } else { m::PX_20 };
    let second_x = if compact { m::PX_104 } else { m::PX_118 };
    let third_x = if compact { m::PX_190 } else { m::PX_222 };
    let second_y = if compact { m::PX_60 } else { m::PX_64 };
    [
        LayoutRect::new(m::PX_14, m::PX_36, width, height),
        LayoutRect::new(second_x, m::PX_36, width, height),
        LayoutRect::new(third_x, m::PX_36, width, height),
        LayoutRect::new(m::PX_14, second_y, width, height),
        LayoutRect::new(second_x, second_y, width, height),
        LayoutRect::new(third_x, second_y, width, height),
    ]
}

#[cfg(test)]
pub(super) fn badge_chip_rects_for_test(
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) -> [LayoutRect; BADGE_CHIP_COUNT] {
    chip_rects(scenario.preset_index)
        .map(|rect| LayoutRect::new(x + rect.x, y + rect.y, rect.width, rect.height))
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
    if scenario.preset_index == THEME_BADGE_PRESET || scenario.preset_index == FILLED_VARIANT_PRESET
    {
        return common::TOKEN;
    }
    palette.accent
}

fn danger_fill(scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == FILLED_VARIANT_PRESET {
        return common::SUCCESS;
    }
    common::DANGER
}

fn icon_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == LEADING_ICON_PRESET {
        return common::SUCCESS;
    }
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
    if scenario.preset_index == LEADING_ICON_PRESET {
        return "leading_icon=dot";
    }
    if scenario.preset_index == FILLED_VARIANT_PRESET {
        return "variant=filled";
    }
    if scenario.screen_state.state_label == "idle" {
        return "state=ready";
    }
    scenario.screen_state.state_label
}
