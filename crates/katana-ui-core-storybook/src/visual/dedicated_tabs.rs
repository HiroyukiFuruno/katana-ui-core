use super::canvas::Canvas;
use super::dedicated_dod_common as common;
use super::dedicated_tabs_controls;
use super::dedicated_tabs_metrics::control_rects;
use super::dedicated_tabs_strip;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::screen_state_tabs::{TabsScreenAction, TabsScreenState};
use super::text::TextRenderer;

pub(super) fn tabs(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let render_state = render_state(scenario);
    common::frame(canvas, text, palette, x, y, "Katana workspace tabs");
    dedicated_tabs_strip::draw_strip(canvas, text, palette, &render_state, x, y);
    dedicated_tabs_controls::draw_controls(canvas, text, palette, scenario, x, y);
    dedicated_tabs_controls::draw_overflow_button(canvas, text, palette, &render_state, x, y);
    dedicated_tabs_controls::draw_status(canvas, text, palette, scenario, &render_state, x, y);
    if render_state.overflow_open {
        dedicated_tabs_controls::draw_overflow_menu(canvas, text, palette, x, y);
    }
}

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

fn render_state(scenario: ScenarioContext<'_>) -> TabsScreenState {
    if scenario.screen_state.has_widget_action() || scenario.screen_state.has_settings_override() {
        return scenario.screen_state.tabs.clone();
    }
    TabsScreenState::for_preset(scenario.preset_index)
}

#[cfg(test)]
pub(super) fn control_rect_for_test(
    action: TabsScreenAction,
) -> Option<super::layout_metrics::LayoutRect> {
    control_rects(0, 0)
        .into_iter()
        .find(|(candidate, _)| *candidate == action)
        .map(|(_, rect)| rect)
}

#[cfg(test)]
pub(super) fn strip_rect_for_test() -> common::Rect {
    super::dedicated_tabs_metrics::rect_to_common(super::layout_metrics::LayoutRect::new(
        super::dedicated_tabs_metrics::STRIP_X,
        super::dedicated_tabs_metrics::STRIP_Y,
        super::dedicated_tabs_metrics::STRIP_WIDTH,
        super::dedicated_tabs_metrics::STRIP_HEIGHT,
    ))
}
