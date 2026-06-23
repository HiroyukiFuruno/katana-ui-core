use super::dedicated_dod_common::TextSpec;
use super::dedicated_dod_metrics as m;
use super::dedicated_toast_stack_manager::{
    ACTION_X, ACTION_Y, LABEL_X_OFFSET, LABEL_Y_OFFSET, STATUS_X, STATUS_Y, TOP_TOAST_X,
    TOP_TOAST_Y,
};
use super::dedicated_toast_stack_manager_style::{
    action_label, action_text, pause_label, pause_text, position_label, position_text, queue_label,
    state_label, toast_text, top_toast_label,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const POSITION_LABEL_X: usize = 36;
const POSITION_LABEL_Y: usize = 40;
const PAUSE_LABEL_X: usize = 36;
const PAUSE_LABEL_Y: usize = 70;
const TOAST_TITLE_Y: usize = TOP_TOAST_Y + 5;
const TOAST_BODY_Y: usize = TOP_TOAST_Y + 16;
const SECOND_TOAST_LABEL_X: usize = 224;
const SECOND_TOAST_LABEL_Y: usize = 66;
const THIRD_TOAST_LABEL_X: usize = 206;
const THIRD_TOAST_LABEL_Y: usize = 94;
const QUEUE_LABEL_X: usize = 206;
const QUEUE_LABEL_Y: usize = 112;
const LABEL_COUNT: usize = 9;

pub(super) fn labels(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            POSITION_LABEL_X,
            POSITION_LABEL_Y,
            m::FONT_8,
            position_text(palette, scenario),
            position_label(scenario),
        ),
        TextSpec::new(
            PAUSE_LABEL_X,
            PAUSE_LABEL_Y,
            m::FONT_8,
            pause_text(palette, scenario),
            pause_label(scenario),
        ),
        TextSpec::new(
            STATUS_X + LABEL_X_OFFSET,
            STATUS_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
        TextSpec::new(
            TOP_TOAST_X + LABEL_X_OFFSET,
            TOAST_TITLE_Y,
            m::FONT_8,
            toast_text(palette, scenario),
            top_toast_label(scenario),
        ),
        TextSpec::new(
            TOP_TOAST_X + LABEL_X_OFFSET,
            TOAST_BODY_Y,
            m::FONT_7,
            toast_text(palette, scenario),
            queue_label(scenario),
        ),
        TextSpec::new(
            SECOND_TOAST_LABEL_X,
            SECOND_TOAST_LABEL_Y,
            m::FONT_7,
            palette.muted,
            "Lint warning",
        ),
        TextSpec::new(
            THIRD_TOAST_LABEL_X,
            THIRD_TOAST_LABEL_Y,
            m::FONT_7,
            palette.muted,
            "Queued toast",
        ),
        TextSpec::new(
            ACTION_X + LABEL_X_OFFSET,
            ACTION_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            action_text(palette, scenario),
            action_label(scenario),
        ),
        TextSpec::new(
            QUEUE_LABEL_X,
            QUEUE_LABEL_Y,
            m::FONT_7,
            palette.muted,
            queue_label(scenario),
        ),
    ]
}
