use super::dedicated_banner::{
    ACTION_X, ACTION_Y, BODY_Y, DETAILS_X, DETAILS_Y, DISMISS_X, DISMISS_Y, ICON_X, ICON_Y,
    LABEL_X_OFFSET, LABEL_Y_OFFSET, TEXT_X, TITLE_Y,
};
use super::dedicated_banner_style::icon_label;
use super::dedicated_banner_style::{
    action_label, action_text, banner_text, body_label, details_label, state_label, title_label,
};
use super::dedicated_dod_common::TextSpec;
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const DETAILS_TEXT_Y: usize = DETAILS_Y + 8;
const STATUS_X: usize = DETAILS_X + 282;
const STATUS_Y: usize = DETAILS_Y + 8;
const ICON_LABEL_X_OFFSET: usize = 5;
const ICON_LABEL_Y_OFFSET: usize = 3;
const LABEL_COUNT: usize = 7;

pub(super) fn labels(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            ICON_X + ICON_LABEL_X_OFFSET,
            ICON_Y + ICON_LABEL_Y_OFFSET,
            m::FONT_9,
            banner_text(palette, scenario),
            icon_label(scenario),
        ),
        TextSpec::new(
            TEXT_X,
            TITLE_Y,
            m::FONT_9,
            banner_text(palette, scenario),
            title_label(scenario),
        ),
        TextSpec::new(
            TEXT_X,
            BODY_Y,
            m::FONT_8,
            banner_text(palette, scenario),
            body_label(scenario),
        ),
        TextSpec::new(
            ACTION_X + LABEL_X_OFFSET,
            ACTION_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            action_text(palette, scenario),
            action_label(scenario),
        ),
        TextSpec::new(
            DISMISS_X + LABEL_X_OFFSET,
            DISMISS_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "x",
        ),
        TextSpec::new(
            DETAILS_X + LABEL_X_OFFSET,
            DETAILS_TEXT_Y,
            m::FONT_7,
            palette.muted,
            details_label(scenario),
        ),
        TextSpec::new(
            STATUS_X,
            STATUS_Y,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
    ]
}
