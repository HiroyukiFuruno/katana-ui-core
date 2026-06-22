use super::canvas::Canvas;
use super::dedicated_dod_common as common;
use super::dedicated_tabs_context_menu;
use super::dedicated_tabs_controls;
use super::dedicated_tabs_layout;
use super::dedicated_tabs_metrics::control_rects;
use super::dedicated_tabs_strip;
use super::layout_metrics::LayoutRect;
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
    dedicated_tabs_strip::draw_strip(
        canvas,
        text,
        palette,
        &render_state,
        scenario.screen_state.preview_hovered,
        x,
        y,
    );
    dedicated_tabs_controls::draw_controls(canvas, text, palette, scenario, x, y);
    dedicated_tabs_controls::draw_overflow_button(canvas, text, palette, &render_state, x, y);
    dedicated_tabs_controls::draw_status(canvas, text, palette, scenario, &render_state, x, y);
    if render_state.overflow_open {
        dedicated_tabs_controls::draw_overflow_menu(canvas, text, palette, x, y);
    }
    dedicated_tabs_context_menu::draw_context_menu(canvas, text, palette, &render_state, x, y);
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

pub(super) fn tab_hit_at(
    origin_x: usize,
    origin_y: usize,
    x: usize,
    y: usize,
    state: &TabsScreenState,
) -> Option<(String, LayoutRect)> {
    dedicated_tabs_layout::tab_hit_at(origin_x, origin_y, x, y, state)
}

pub(super) fn group_hit_at(
    origin_x: usize,
    origin_y: usize,
    x: usize,
    y: usize,
    state: &TabsScreenState,
) -> Option<(String, LayoutRect)> {
    dedicated_tabs_layout::group_hit_at(origin_x, origin_y, x, y, state)
}

pub(super) fn pin_icon_hit_at(
    origin_x: usize,
    origin_y: usize,
    x: usize,
    y: usize,
    state: &TabsScreenState,
) -> Option<String> {
    dedicated_tabs_layout::pin_icon_hit_at(origin_x, origin_y, x, y, state)
}

pub(super) fn context_menu_command_at(
    origin_x: usize,
    origin_y: usize,
    x: usize,
    y: usize,
    state: &TabsScreenState,
) -> Option<super::screen_state_tabs::TabsContextMenuCommand> {
    dedicated_tabs_context_menu::command_at(origin_x, origin_y, x, y, state)
}

pub(super) fn strip_hit_at(origin_x: usize, origin_y: usize, x: usize, y: usize) -> bool {
    super::layout_metrics::LayoutRect::new(
        origin_x + super::dedicated_tabs_metrics::STRIP_X,
        origin_y + super::dedicated_tabs_metrics::STRIP_Y,
        super::dedicated_tabs_metrics::STRIP_WIDTH,
        super::dedicated_tabs_metrics::STRIP_HEIGHT,
    )
    .contains(x, y)
}

fn render_state(scenario: ScenarioContext<'_>) -> TabsScreenState {
    if scenario.screen_state.has_widget_action() || scenario.screen_state.has_settings_override() {
        return scenario.screen_state.tabs.clone();
    }
    TabsScreenState::for_preset(scenario.preset_index)
}

#[cfg(test)]
pub(super) fn context_menu_rect_for_test(
    state: &TabsScreenState,
) -> Option<super::layout_metrics::LayoutRect> {
    state
        .context_menu
        .as_ref()
        .map(|_| dedicated_tabs_context_menu::menu_rect(0, 0, state))
}

#[cfg(test)]
pub(super) fn context_menu_labels_for_test(state: &TabsScreenState) -> Vec<&str> {
    dedicated_tabs_context_menu::menu_labels_for_test(state)
}

#[cfg(test)]
pub(super) fn group_rect_for_test(
    state: &TabsScreenState,
    group_id: &str,
) -> Option<super::layout_metrics::LayoutRect> {
    dedicated_tabs_layout::group_rect_for_id(0, 0, state, group_id)
}

#[cfg(test)]
pub(super) fn tab_rect_for_test(
    state: &TabsScreenState,
    tab_id: &str,
) -> Option<super::layout_metrics::LayoutRect> {
    dedicated_tabs_layout::tab_rect_for_id(0, 0, state, tab_id)
}

#[cfg(test)]
pub(super) fn tab_ids_for_test(state: &TabsScreenState) -> Vec<&str> {
    let mut ids = Vec::new();
    for item in dedicated_tabs_layout::layout_items(0, 0, state) {
        if let dedicated_tabs_layout::TabsLayoutItem::Tab { tab, .. } = item {
            ids.push(tab.id.as_str());
        }
    }
    ids
}

#[cfg(test)]
pub(super) fn layout_item_ids_for_test(state: &TabsScreenState) -> Vec<String> {
    dedicated_tabs_layout::layout_items(0, 0, state)
        .into_iter()
        .map(|item| match item {
            dedicated_tabs_layout::TabsLayoutItem::GroupHeader { group, .. } => {
                format!("group:{}", group.id)
            }
            dedicated_tabs_layout::TabsLayoutItem::Tab { tab, .. } => tab.id.clone(),
        })
        .collect()
}

#[cfg(test)]
pub(super) fn pin_icon_rect_for_test(
    state: &TabsScreenState,
    tab_id: &str,
) -> Option<super::layout_metrics::LayoutRect> {
    dedicated_tabs_layout::pin_icon_rect_for_id(0, 0, state, tab_id)
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

#[cfg(test)]
pub(super) fn scroll_x_for_test(state: &TabsScreenState) -> usize {
    super::dedicated_tabs_scroll::scroll_x(state)
}

#[cfg(test)]
pub(super) fn measured_item_ids_for_test(state: &TabsScreenState) -> Vec<String> {
    super::dedicated_tabs_scroll::measured_item_ids_for_test(state)
}
