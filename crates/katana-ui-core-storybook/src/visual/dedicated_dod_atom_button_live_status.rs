use super::canvas::Canvas;
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const STATUS_X: usize = 360;
const STATUS_Y: usize = 34;
const STATUS_WIDTH: usize = 132;
const STATUS_HEIGHT: usize = 16;
const STATUS_GAP: usize = 4;
const STATUS_TEXT_X: usize = 8;
const STATUS_TEXT_Y: usize = 5;
#[cfg(test)]
const MIN_FRAME_PADDING: usize = 8;
#[cfg(test)]
const MAX_LABEL_CHARS: usize = 14;
const STATUS_ROW_COUNT: usize = 3;

pub(super) fn draw_status_rows(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let rows = [
        action_label(scenario).to_string(),
        event_label(scenario).to_string(),
        state_label(scenario).to_string(),
    ];
    for (index, label) in rows.into_iter().take(STATUS_ROW_COUNT).enumerate() {
        let row_y = y + STATUS_Y + index * (STATUS_HEIGHT + STATUS_GAP);
        canvas.fill_rect(
            x + STATUS_X,
            row_y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        );
        canvas.stroke_rect(
            x + STATUS_X,
            row_y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.border,
        );
        text.draw(
            canvas,
            &label,
            x + STATUS_X + STATUS_TEXT_X,
            row_y + STATUS_TEXT_Y,
            m::FONT_8,
            palette.muted,
        );
    }
}

fn action_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.screen_state.last_action {
        "none" => "action ready",
        "button_press" | "text_button_press" | "svg_button_press" | "icon_text_button_press" => {
            "press"
        }
        "button_press_blocked" => "blocked",
        "button_option_apply" => "option apply",
        "settings_option_changed" => "settings edit",
        _ => scenario.screen_state.last_action,
    }
}

fn event_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.screen_state.last_event {
        "none" => "event ready",
        "button_clicked"
        | "text_button_clicked"
        | "svg_button_clicked"
        | "icon_text_button_clicked" => "clicked",
        "button_disabled_ignored" => "ignored",
        "button_option_changed" => "option change",
        "button_settings_changed" => "settings event",
        _ => scenario.screen_state.last_event,
    }
}

fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "state idle";
    }
    scenario.screen_state.state_label
}

#[cfg(test)]
pub(super) fn status_rows_fit_for_test(scenario: ScenarioContext<'_>) -> bool {
    [
        action_label(scenario).to_string(),
        event_label(scenario).to_string(),
        state_label(scenario).to_string(),
    ]
    .into_iter()
    .all(|label| label.chars().count() <= MAX_LABEL_CHARS)
}

#[cfg(test)]
pub(super) const fn status_rows_have_frame_padding_for_test() -> bool {
    STATUS_X >= MIN_FRAME_PADDING
        && STATUS_Y >= MIN_FRAME_PADDING
        && STATUS_WIDTH + STATUS_X <= super::dedicated_dod_common::AREA_WIDTH - MIN_FRAME_PADDING
        && status_rows_bottom_for_test()
            <= super::dedicated_dod_common::AREA_HEIGHT - MIN_FRAME_PADDING
}

#[cfg(test)]
pub(super) const fn status_rows_start_x_for_test() -> usize {
    STATUS_X
}

#[cfg(test)]
pub(super) const fn status_rows_bottom_for_test() -> usize {
    STATUS_Y + STATUS_ROW_COUNT * STATUS_HEIGHT + (STATUS_ROW_COUNT - 1) * STATUS_GAP
}
