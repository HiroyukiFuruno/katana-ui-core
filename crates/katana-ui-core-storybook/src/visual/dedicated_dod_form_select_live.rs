use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common};
pub(super) use super::dedicated_dod_form_select_live_layout::{
    select_close_button_rect, select_open_button_rect, select_reset_button_rect,
    select_state_read_button_rect,
};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::selection_control_metrics as sm;
use super::text::{TextRenderer, TextVerticalBox};

const CONTROL_BUTTON_GAP: usize = 8;
const CONTROL_TEXT_Y: usize = 6;
const LIGHT_OPTION_INDEX: usize = 1;
const DARK_OPTION_INDEX: usize = 2;
const SYSTEM_OPTION_INDEX: usize = 3;
const ITEMS_PRESET_INDEX: usize = 0;
const OPEN_PRESET_INDEX: usize = 1;
const SELECTED_PRESET_INDEX: usize = 2;
const PLACEHOLDER_PRESET_INDEX: usize = 3;
const DISABLED_PRESET_INDEX: usize = 4;

pub(super) fn select_box(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "SelectBox");
    draw_trigger(canvas, text, palette, scenario, x, y);
    draw_options(canvas, text, palette, scenario, x, y);
    draw_controls(canvas, text, palette, x, y);
    draw_status(canvas, text, palette, scenario, x, y);
}

fn draw_trigger(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let fill = if scenario.preset_index == DISABLED_PRESET_INDEX {
        palette.panel
    } else {
        palette.surface
    };
    let border = if scenario.preset_index == PLACEHOLDER_PRESET_INDEX {
        palette.accent
    } else {
        palette.border
    };
    canvas.fill_rect(
        x + sm::TRIGGER_X,
        y + sm::TRIGGER_Y,
        sm::TRIGGER_WIDTH,
        sm::TRIGGER_HEIGHT,
        fill,
    );
    canvas.stroke_rect(
        x + sm::TRIGGER_X,
        y + sm::TRIGGER_Y,
        sm::TRIGGER_WIDTH,
        sm::TRIGGER_HEIGHT,
        border,
    );
    text.draw_centered(
        canvas,
        select_value(scenario),
        x + sm::TRIGGER_X + sm::TEXT_X,
        TextVerticalBox::new(y + sm::TRIGGER_Y, sm::TRIGGER_HEIGHT as f32),
        m::FONT_9,
        trigger_text(palette, scenario),
    );
}

fn draw_options(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    if !select_open(scenario) {
        return;
    }
    canvas.fill_rect(
        x + sm::TRIGGER_X,
        y + sm::SELECT_OPTIONS_Y,
        sm::TRIGGER_WIDTH,
        sm::SELECT_OPTION_HEIGHT * sm::SELECT_OPTION_COUNT,
        palette.surface,
    );
    for (index, label) in ["Placeholder", "Light", "Dark", "System"]
        .into_iter()
        .enumerate()
    {
        let row_y = y + sm::SELECT_OPTIONS_Y + index * sm::SELECT_OPTION_HEIGHT;
        if selected_index(scenario) == Some(index) {
            canvas.fill_rect(
                x + sm::TRIGGER_X + sm::OPTION_ROW_INSET,
                row_y,
                sm::TRIGGER_WIDTH - sm::OPTION_ROW_WIDTH_REDUCTION,
                sm::SELECT_OPTION_HEIGHT,
                palette.accent,
            );
        }
        text.draw(
            canvas,
            label,
            x + sm::TRIGGER_X + sm::TEXT_X,
            row_y + m::PX_3,
            m::FONT_7,
            palette.text,
        );
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
    let rows = [
        status_action(scenario),
        status_event(scenario),
        status_state(scenario),
    ];
    for (index, row) in rows.into_iter().enumerate() {
        let row_y = y + sm::STATUS_Y + index * (sm::STATUS_HEIGHT + sm::STATUS_GAP);
        canvas.fill_rect(
            x + sm::STATUS_X,
            row_y,
            sm::STATUS_WIDTH,
            sm::STATUS_HEIGHT,
            palette.panel,
        );
        canvas.stroke_rect(
            x + sm::STATUS_X,
            row_y,
            sm::STATUS_WIDTH,
            sm::STATUS_HEIGHT,
            palette.border,
        );
        text.draw(
            canvas,
            row,
            x + sm::STATUS_X + sm::TEXT_X,
            row_y + sm::TEXT_Y,
            m::FONT_8,
            palette.muted,
        );
    }
}

fn draw_controls(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    for (rect, label) in [
        (select_state_read_button_rect(x, y), "read"),
        (select_open_button_rect(x, y), "open"),
        (select_close_button_rect(x, y), "close"),
        (select_reset_button_rect(x, y), "reset"),
    ] {
        canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.surface);
        canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, palette.border);
        text.draw(
            canvas,
            label,
            rect.x + CONTROL_BUTTON_GAP,
            rect.y + CONTROL_TEXT_Y,
            m::FONT_8,
            palette.text,
        );
    }
}

fn select_value(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == ITEMS_PRESET_INDEX {
        return "6 items";
    }
    if scenario.preset_index == PLACEHOLDER_PRESET_INDEX {
        return "Choose theme...";
    }
    if scenario.preset_index == DISABLED_PRESET_INDEX {
        return "Disabled";
    }
    match selected_index(scenario) {
        Some(LIGHT_OPTION_INDEX) => "Light",
        Some(DARK_OPTION_INDEX) => "Dark",
        Some(SYSTEM_OPTION_INDEX) => "System",
        _ => "Placeholder",
    }
}

fn select_open(scenario: ScenarioContext<'_>) -> bool {
    scenario.screen_state.selection.select_open
        || scenario.preset_index == OPEN_PRESET_INDEX
        || scenario.preset_index == ITEMS_PRESET_INDEX
}

fn selected_index(scenario: ScenarioContext<'_>) -> Option<usize> {
    if scenario
        .screen_state
        .selection
        .select_selected_index
        .is_some()
    {
        return scenario.screen_state.selection.select_selected_index;
    }
    match scenario.preset_index {
        SELECTED_PRESET_INDEX => Some(LIGHT_OPTION_INDEX),
        _ => None,
    }
}

fn trigger_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if matches!(
        scenario.preset_index,
        PLACEHOLDER_PRESET_INDEX | DISABLED_PRESET_INDEX
    ) {
        return palette.muted;
    }
    palette.text
}

fn status_action(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "action ready";
    }
    scenario.screen_state.last_action
}

fn status_event(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_event == "none" {
        return "event ready";
    }
    scenario.screen_state.last_event
}

fn status_state(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == ITEMS_PRESET_INDEX {
        return "items=6";
    }
    if scenario.preset_index == PLACEHOLDER_PRESET_INDEX {
        return "placeholder=true";
    }
    if scenario.preset_index == DISABLED_PRESET_INDEX {
        return "disabled=true";
    }
    if scenario.screen_state.state_label == "idle" {
        return "selected=none";
    }
    scenario.screen_state.state_label
}

#[cfg(test)]
mod tests {
    use super::select_value;
    use crate::visual::render_context::ScenarioContext;
    use crate::visual::screen_state::StorybookScreenState;

    #[test]
    fn live_select_values_cover_dark_and_system_options() {
        let mut state = StorybookScreenState::default();
        state.selection.select_selected_index = Some(2);
        assert_eq!(
            "Dark",
            select_value(ScenarioContext::for_test("select", usize::MAX, &state))
        );

        state.selection.select_selected_index = Some(3);
        assert_eq!(
            "System",
            select_value(ScenarioContext::for_test("select", usize::MAX, &state))
        );
    }
}
