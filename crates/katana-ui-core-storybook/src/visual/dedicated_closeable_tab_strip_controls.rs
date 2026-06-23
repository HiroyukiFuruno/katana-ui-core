use super::dedicated_dod_common::{Block, TextSpec};
use super::dedicated_dod_metrics as m;
use super::layout_metrics::LayoutRect;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::screen_state_tabs::TabsScreenAction;

pub(super) const ADD_CONTROL_INDEX: usize = 0;
pub(super) const CLOSE_CONTROL_INDEX: usize = 1;
pub(super) const PIN_CONTROL_INDEX: usize = 2;
pub(super) const MOVE_CONTROL_INDEX: usize = 3;
pub(super) const GROUP_CONTROL_INDEX: usize = 4;
pub(super) const OVERFLOW_CONTROL_INDEX: usize = 5;

const CONTROL_X: usize = 42;
const CONTROL_Y: usize = 124;
const CONTROL_WIDTH: usize = 48;
const CONTROL_HEIGHT: usize = 20;
const CONTROL_GAP: usize = 6;
const CONTROL_LABEL_X_OFFSET: usize = 7;
const CONTROL_LABEL_Y_OFFSET: usize = 6;
const CONTROL_COUNT: usize = 6;

pub(super) fn control_at(
    origin_x: usize,
    origin_y: usize,
    x: usize,
    y: usize,
) -> Option<TabsScreenAction> {
    for (action, rect) in control_rects(origin_x, origin_y) {
        if rect.contains(x, y) {
            return Some(action);
        }
    }
    None
}

pub(super) fn control_block(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    index: usize,
) -> Block {
    let rect = control_rect(0, 0, index);
    let fill = if scenario.screen_state.has_widget_action() && index == ADD_CONTROL_INDEX {
        palette.accent
    } else {
        palette.panel
    };
    Block::outlined(rect.x, rect.y, rect.width, rect.height, fill)
}

pub(super) fn control_label(
    palette: &VisualPalette,
    index: usize,
    value: &'static str,
) -> TextSpec {
    let rect = control_rect(0, 0, index);
    TextSpec::new(
        rect.x + CONTROL_LABEL_X_OFFSET,
        rect.y + CONTROL_LABEL_Y_OFFSET,
        m::FONT_7,
        palette.text,
        value,
    )
}

#[cfg(test)]
pub(super) fn control_rect_for_test(action: TabsScreenAction) -> Option<LayoutRect> {
    control_rects(0, 0)
        .into_iter()
        .find(|(candidate, _)| *candidate == action)
        .map(|(_, rect)| rect)
}

fn control_rects(
    origin_x: usize,
    origin_y: usize,
) -> [(TabsScreenAction, LayoutRect); CONTROL_COUNT] {
    [
        (
            TabsScreenAction::AddTab,
            control_rect(origin_x, origin_y, ADD_CONTROL_INDEX),
        ),
        (
            TabsScreenAction::CloseActive,
            control_rect(origin_x, origin_y, CLOSE_CONTROL_INDEX),
        ),
        (
            TabsScreenAction::TogglePinActive,
            control_rect(origin_x, origin_y, PIN_CONTROL_INDEX),
        ),
        (
            TabsScreenAction::MoveActiveRight,
            control_rect(origin_x, origin_y, MOVE_CONTROL_INDEX),
        ),
        (
            TabsScreenAction::GroupActive,
            control_rect(origin_x, origin_y, GROUP_CONTROL_INDEX),
        ),
        (
            TabsScreenAction::ToggleOverflow,
            control_rect(origin_x, origin_y, OVERFLOW_CONTROL_INDEX),
        ),
    ]
}

const fn control_rect(origin_x: usize, origin_y: usize, index: usize) -> LayoutRect {
    LayoutRect::new(
        origin_x + CONTROL_X + index * (CONTROL_WIDTH + CONTROL_GAP),
        origin_y + CONTROL_Y,
        CONTROL_WIDTH,
        CONTROL_HEIGHT,
    )
}
