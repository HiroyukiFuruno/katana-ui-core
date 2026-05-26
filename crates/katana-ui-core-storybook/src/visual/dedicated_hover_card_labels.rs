use super::dedicated_dod_common::TextSpec;
use super::dedicated_dod_metrics as m;
use super::dedicated_hover_card::{
    ACTION_X, ACTION_Y, ANCHOR_X, ANCHOR_Y, CARD_Y, FIRST_LABEL_Y_OFFSET, LABEL_GAP, STATUS_TEXT_X,
    STATUS_TEXT_Y, STATUS_WIDTH, STATUS_X, STATUS_Y, TITLE_X_OFFSET, action_label, action_text,
    anchor_label, anchor_text, body_label, card_text, card_x, footer_label, state_label,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const LABEL_COUNT: usize = 7;

pub(super) fn labels(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            ANCHOR_X + TITLE_X_OFFSET,
            ANCHOR_Y + FIRST_LABEL_Y_OFFSET,
            m::FONT_8,
            anchor_text(palette, scenario),
            anchor_label(scenario),
        ),
        TextSpec::new(
            card_x(scenario) + TITLE_X_OFFSET,
            CARD_Y + FIRST_LABEL_Y_OFFSET,
            m::FONT_9,
            card_text(palette, scenario),
            "Capability",
        ),
        TextSpec::new(
            card_x(scenario) + TITLE_X_OFFSET,
            CARD_Y + FIRST_LABEL_Y_OFFSET + LABEL_GAP,
            m::FONT_8,
            card_text(palette, scenario),
            body_label(scenario),
        ),
        TextSpec::new(
            card_x(scenario) + TITLE_X_OFFSET,
            CARD_Y + FIRST_LABEL_Y_OFFSET + LABEL_GAP + LABEL_GAP,
            m::FONT_8,
            card_text(palette, scenario),
            footer_label(scenario),
        ),
        TextSpec::new(
            ACTION_X + STATUS_TEXT_X,
            ACTION_Y + STATUS_TEXT_Y,
            m::FONT_7,
            action_text(palette, scenario),
            "Configure",
        ),
        TextSpec::new(
            STATUS_X + STATUS_TEXT_X,
            STATUS_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
            action_label(scenario),
        ),
        TextSpec::new(
            STATUS_X + STATUS_WIDTH + STATUS_TEXT_X,
            STATUS_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
    ]
}
