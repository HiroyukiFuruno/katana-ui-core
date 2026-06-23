use super::dedicated_dod_common::TextSpec;
use super::dedicated_dod_metrics as m;
use super::dedicated_notification_toast::{
    BODY_Y, CLOSE_X, CLOSE_Y, STACK_X, STACK_Y, STATUS_TEXT_X, STATUS_TEXT_Y, STATUS_X, STATUS_Y,
    TEXT_X, TITLE_Y, body_label, close_text, stack_label, state_label, title_label, toast_text,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const LABEL_COUNT: usize = 5;

pub(super) fn labels(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            TEXT_X,
            TITLE_Y,
            m::FONT_9,
            toast_text(palette, scenario),
            title_label(scenario),
        ),
        TextSpec::new(
            TEXT_X,
            BODY_Y,
            m::FONT_8,
            toast_text(palette, scenario),
            body_label(scenario),
        ),
        TextSpec::new(
            CLOSE_X + STATUS_TEXT_X,
            CLOSE_Y + STATUS_TEXT_Y,
            m::FONT_7,
            close_text(palette, scenario),
            "x",
        ),
        TextSpec::new(
            STACK_X + STATUS_TEXT_X,
            STACK_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
            stack_label(scenario),
        ),
        TextSpec::new(
            STATUS_X + STATUS_TEXT_X,
            STATUS_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
    ]
}
