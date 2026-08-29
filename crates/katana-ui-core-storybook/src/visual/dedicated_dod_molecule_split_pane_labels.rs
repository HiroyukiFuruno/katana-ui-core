use super::canvas::Canvas;
use super::dedicated_dod_common::TextSpec;
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const AXIS_PRESET_INDEX: usize = 0;
pub(super) const GAP_PRESET_INDEX: usize = 1;
pub(super) const ALIGN_PRESET_INDEX: usize = 2;
pub(super) const OVERFLOW_PRESET_INDEX: usize = 3;
pub(super) const RATIO_PRESET_INDEX: usize = 4;
pub(super) const MIN_PRESET_INDEX: usize = 5;
pub(super) const MAX_PRESET_INDEX: usize = 6;
pub(super) const RESET_PRESET_INDEX: usize = 7;
pub(super) const HANDLE_PRESET_INDEX: usize = 8;
pub(super) const KEYBOARD_PRESET_INDEX: usize = 9;
pub(super) const DEFAULT_RATIO_PERCENT: u8 = 50;

const LABEL_X: usize = m::PX_284;
const STATUS_Y: usize = m::PX_96;
const STATUS_WIDTH: usize = m::PX_96;
const STATUS_HEIGHT: usize = m::PX_18;
const STATUS_GAP: usize = m::PX_8;
const STATUS_TEXT_X: usize = m::PX_6;
const STATUS_TEXT_Y: usize = m::PX_4;
const SPLIT_LABEL_COUNT: usize = 4;
const STATUS_LABEL_COUNT: usize = 3;

pub(super) fn split_labels(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [TextSpec; SPLIT_LABEL_COUNT] {
    [
        TextSpec::new(
            LABEL_X,
            m::PX_42,
            m::FONT_9,
            palette.text,
            split_preset_label(scenario),
        ),
        TextSpec::new(LABEL_X, m::PX_58, m::FONT_8, palette.muted, "resize handle"),
        TextSpec::new(
            LABEL_X,
            m::PX_74,
            m::FONT_8,
            palette.muted,
            "state/action via settings",
        ),
        TextSpec::new(m::PX_32, m::PX_54, m::FONT_8, palette.background, "A | B"),
    ]
}

pub(super) fn draw_status(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    for (index, label) in status_labels(scenario).into_iter().enumerate() {
        let row_x = x + m::PX_18 + index * (STATUS_WIDTH + STATUS_GAP);
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

fn split_preset_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        AXIS_PRESET_INDEX => "axis vertical",
        GAP_PRESET_INDEX => "wide gap",
        ALIGN_PRESET_INDEX => "center alignment",
        OVERFLOW_PRESET_INDEX => "overflow scroll",
        RATIO_PRESET_INDEX => "ratio 64",
        MIN_PRESET_INDEX => "min clamp",
        MAX_PRESET_INDEX => "max clamp",
        RESET_PRESET_INDEX => "reset 55",
        HANDLE_PRESET_INDEX => "wide handle",
        KEYBOARD_PRESET_INDEX => "keyboard resize",
        _ => "split pane",
    }
}

fn status_labels(scenario: ScenarioContext<'_>) -> [&'static str; STATUS_LABEL_COUNT] {
    if scenario.screen_state.split_pane.dragging() {
        return ["action drag", "event ratio", "state drag"];
    }
    if scenario.screen_state.split_pane.resized() {
        return ["action resize", "event ratio", "state clamp"];
    }
    if scenario.screen_state.split_pane.ratio_percent() != DEFAULT_RATIO_PERCENT {
        return ["action key", "event ratio", "state ratio"];
    }
    if scenario.screen_state.split_pane.focused() {
        return ["action focus", "event focus", "state handle"];
    }
    if scenario.screen_state.split_pane.hovered() {
        return ["action hover", "event hover", "state handle"];
    }
    if scenario.screen_state.has_widget_action() || scenario.screen_state.has_settings_override() {
        return ["action resize", "event ratio", "state changed"];
    }
    match scenario.preset_index {
        AXIS_PRESET_INDEX => ["axis vertical", "event axis", "state local"],
        GAP_PRESET_INDEX => ["gap 12", "event layout", "state local"],
        ALIGN_PRESET_INDEX => ["align center", "event layout", "state local"],
        OVERFLOW_PRESET_INDEX => ["overflow", "event scroll", "state local"],
        RATIO_PRESET_INDEX => ["ratio 64", "event ratio", "state local"],
        MIN_PRESET_INDEX => ["min clamp", "event reject", "state local"],
        MAX_PRESET_INDEX => ["max clamp", "event reject", "state local"],
        RESET_PRESET_INDEX => ["reset 55", "event reset", "state local"],
        HANDLE_PRESET_INDEX => ["handle 10", "event hover", "state local"],
        KEYBOARD_PRESET_INDEX => ["key resize", "event ratio", "focus handle"],
        _ => ["action ready", "event ready", "state idle"],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visual::screen_state::StorybookScreenState;
    use crate::visual::window_interaction::split_pane_operation::SplitPaneStoryAction;

    #[test]
    fn split_labels_cover_fallback_focus_and_resize_runtime_states() {
        let idle = StorybookScreenState::default();
        let fallback = ScenarioContext::for_test("split-pane", usize::MAX, &idle);
        assert_eq!("split pane", split_preset_label(fallback));
        assert_eq!(
            ["action ready", "event ready", "state idle"],
            status_labels(fallback)
        );

        let mut focused = StorybookScreenState::default();
        focused.register_split_pane_action(SplitPaneStoryAction::Focus);
        assert_eq!(
            ["action focus", "event focus", "state handle"],
            status_labels(ScenarioContext::for_test("split-pane", 0, &focused))
        );

        let mut resized = StorybookScreenState::default();
        resized.register_split_pane_action(SplitPaneStoryAction::Resize);
        assert_eq!(
            ["action resize", "event ratio", "state clamp"],
            status_labels(ScenarioContext::for_test("split-pane", 0, &resized))
        );
    }
}
