use super::dedicated_dod_common::TextSpec;
use super::dedicated_dod_metrics as m;
use super::dedicated_modal::{
    DIALOG_X, DIALOG_Y, FIRST_LABEL_Y_OFFSET, LABEL_GAP, LABEL_X_OFFSET, NATIVE_X, NATIVE_Y,
    STATUS_GAP, STATUS_TEXT_X, STATUS_TEXT_Y, STATUS_WIDTH, STATUS_X, STATUS_Y, action_label,
    dialog_label, dialog_text, native_body_label, native_text, state_label,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const LABEL_COUNT: usize = 6;

pub(super) fn labels(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            DIALOG_X + LABEL_X_OFFSET,
            DIALOG_Y + FIRST_LABEL_Y_OFFSET,
            m::FONT_8,
            dialog_text(palette, scenario),
            dialog_label(scenario),
        ),
        TextSpec::new(
            NATIVE_X + LABEL_X_OFFSET,
            NATIVE_Y + FIRST_LABEL_Y_OFFSET,
            m::FONT_9,
            native_text(palette, scenario),
            "native modal",
        ),
        TextSpec::new(
            NATIVE_X + LABEL_X_OFFSET,
            NATIVE_Y + FIRST_LABEL_Y_OFFSET + LABEL_GAP,
            m::FONT_9,
            native_text(palette, scenario),
            native_body_label(scenario),
        ),
        TextSpec::new(
            NATIVE_X + LABEL_X_OFFSET,
            NATIVE_Y + FIRST_LABEL_Y_OFFSET + LABEL_GAP + LABEL_GAP,
            m::FONT_9,
            native_text(palette, scenario),
            "footer / return focus",
        ),
        TextSpec::new(
            STATUS_X + STATUS_TEXT_X,
            STATUS_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
            action_label(scenario),
        ),
        TextSpec::new(
            STATUS_X + STATUS_WIDTH + STATUS_GAP + STATUS_TEXT_X,
            STATUS_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
    ]
}
