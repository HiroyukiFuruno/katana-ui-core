use super::dedicated_dod_form_input_live_text_area_geometry as geometry;
use super::{Canvas, ScenarioContext, VisualPalette, m};
#[cfg(test)]
pub(in crate::visual) use geometry::text_area_rect_for_state;
pub(super) use geometry::{
    AUTO_GROW_PRESET_INDEX, HORIZONTAL_SCROLL_PRESET_INDEX, HORIZONTAL_SCROLLBAR_PRESET_INDEX,
    RESIZE_PRESET_INDEX, TAB_BEHAVIOR_PRESET_INDEX, VERTICAL_SCROLL_PRESET_INDEX,
    VERTICAL_SCROLLBAR_PRESET_INDEX, text_area_status_rects_for_instance,
};
pub(in crate::visual) use geometry::{
    horizontal_scroll_enabled_for_instance, resize_enabled_for_instance,
    text_area_rect_for_instance, text_area_resize_delta_for_pointer,
    text_area_resize_grip_rect_for_instance, vertical_scroll_enabled_for_instance,
};

const SCROLLBAR_THUMB_X_OFFSET: usize = 5;
const SCROLLBAR_TRACK_SIZE: usize = 2;
const SCROLLBAR_THUMB_SIZE: usize = 4;
const RESIZE_GRIP_SIZE: usize = 10;

pub(super) fn draw_vertical_scrollbar(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    if !vertical_scrollbar_visible(scenario) {
        return;
    }
    let rect = geometry::text_area_rect_for_instance(
        x,
        y,
        scenario.screen_state,
        scenario.selected_instance_id,
    );
    canvas.fill_rect(
        rect.right() - m::PX_4,
        rect.y + m::PX_8,
        SCROLLBAR_TRACK_SIZE,
        rect.height - m::PX_16,
        palette.panel,
    );
    canvas.fill_rect(
        rect.right() - SCROLLBAR_THUMB_X_OFFSET,
        rect.y + thumb_y(scenario),
        SCROLLBAR_THUMB_SIZE,
        m::PX_24,
        palette.accent,
    );
}

pub(super) fn draw_horizontal_scrollbar(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    if !horizontal_scrollbar_visible(scenario) {
        return;
    }
    let rect = geometry::text_area_rect_for_instance(
        x,
        y,
        scenario.screen_state,
        scenario.selected_instance_id,
    );
    canvas.fill_rect(
        rect.x + m::PX_8,
        rect.bottom() - m::PX_4,
        rect.width - m::PX_16,
        SCROLLBAR_TRACK_SIZE,
        palette.panel,
    );
    canvas.fill_rect(
        rect.x + m::PX_12,
        rect.bottom() - SCROLLBAR_THUMB_SIZE,
        m::PX_36,
        SCROLLBAR_THUMB_SIZE,
        palette.accent,
    );
}

pub(super) fn draw_resize_grip(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    if !resize_enabled(scenario) {
        return;
    }
    let rect = geometry::text_area_rect_for_instance(
        x,
        y,
        scenario.screen_state,
        scenario.selected_instance_id,
    );
    let right = rect.right() - m::PX_4;
    let bottom = rect.bottom() - m::PX_4;
    for offset in [0, m::PX_4, m::PX_8] {
        canvas.fill_rect(
            right - offset,
            bottom - (RESIZE_GRIP_SIZE - offset),
            m::PX_2,
            m::PX_2,
            palette.accent,
        );
    }
}

pub(super) fn horizontal_scroll_offset(scenario: ScenarioContext<'_>) -> usize {
    if scenario
        .screen_state
        .text_area_wrap_enabled_for(scenario.selected_instance_id)
        && !matches!(
            scenario.preset_index,
            HORIZONTAL_SCROLL_PRESET_INDEX | HORIZONTAL_SCROLLBAR_PRESET_INDEX
        )
    {
        return 0;
    }
    scenario
        .screen_state
        .text_area_scroll_x_offset_for(scenario.selected_instance_id)
}

pub(super) fn vertical_scroll_offset(scenario: ScenarioContext<'_>) -> usize {
    if vertical_scroll_enabled(scenario) {
        return scenario
            .screen_state
            .text_area_scroll_offset_for(scenario.selected_instance_id);
    }
    0
}

pub(super) fn vertical_scroll_enabled(scenario: ScenarioContext<'_>) -> bool {
    geometry::vertical_scroll_enabled_for_instance(
        scenario.preset_index,
        scenario.screen_state,
        scenario.selected_instance_id,
    )
}

pub(super) fn horizontal_scroll_enabled(scenario: ScenarioContext<'_>) -> bool {
    geometry::horizontal_scroll_enabled_for_instance(
        scenario.preset_index,
        scenario.screen_state,
        scenario.selected_instance_id,
    )
}

fn resize_enabled(scenario: ScenarioContext<'_>) -> bool {
    geometry::resize_enabled_for_instance(
        scenario.preset_index,
        scenario.screen_state,
        scenario.selected_instance_id,
    )
}

fn vertical_scrollbar_visible(scenario: ScenarioContext<'_>) -> bool {
    vertical_scroll_enabled(scenario)
        && (scenario
            .screen_state
            .text_area_vertical_scrollbar_visible_for(scenario.selected_instance_id)
            || scenario.preset_index == VERTICAL_SCROLLBAR_PRESET_INDEX)
}

fn horizontal_scrollbar_visible(scenario: ScenarioContext<'_>) -> bool {
    horizontal_scroll_enabled(scenario)
        && (scenario
            .screen_state
            .text_area_horizontal_scrollbar_visible_for(scenario.selected_instance_id)
            || scenario.preset_index == HORIZONTAL_SCROLLBAR_PRESET_INDEX)
}

fn thumb_y(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == VERTICAL_SCROLLBAR_PRESET_INDEX {
        return m::PX_34;
    }
    m::PX_12
}

#[cfg(test)]
mod tests {
    use super::{m, thumb_y};
    use crate::visual::render_context::ScenarioContext;
    use crate::visual::screen_state::StorybookScreenState;

    #[test]
    fn live_vertical_scrollbar_uses_the_near_thumb_origin() {
        let state = StorybookScreenState::default();

        assert_eq!(
            m::PX_12,
            thumb_y(ScenarioContext::for_test("text-area", 0, &state))
        );
    }
}
