use super::canvas::Canvas;
use super::dedicated_dod_atom_button_live;
use super::dedicated_dod_atom_swatch_live;
use super::dedicated_dod_common as common;
use super::dedicated_dod_form_binary_choice_chrome as choice_chrome;
use super::dedicated_dod_metrics as m;
use super::layout_metrics::LayoutRect;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::switch_control;
use super::text::TextRenderer;

const TOGGLE_ROW_X: usize = 18;
const TOGGLE_ROW_Y: usize = 36;
pub(super) const TOGGLE_ROW_WIDTH: usize = 294;
const TOGGLE_ROW_HEIGHT: usize = 34;
const TOGGLE_SWITCH_WIDTH: usize = 48;
const TOGGLE_SWITCH_HEIGHT: usize = 22;
const TOGGLE_SWITCH_RIGHT_INSET: usize = 14;
const TOGGLE_SWITCH_Y_INSET: usize = 6;

pub(super) fn button_matrix(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
    title: &str,
) {
    dedicated_dod_atom_button_live::draw(canvas, text, palette, scenario, x, y, title);
}
pub(super) fn toggle(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "Toggle switch");
    draw_toggle_row(canvas, text, palette, scenario, x, y);
}
pub(super) fn swatch(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    dedicated_dod_atom_swatch_live::draw(canvas, text, palette, scenario, x, y);
}

fn draw_toggle_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let on = if scenario.screen_state.toggle_checked_overridden {
        scenario.screen_state.toggle_checked
    } else {
        scenario.preset_index == m::PX_1
    };
    let disabled = scenario.preset_index == m::PX_2;
    let themed = scenario.preset_index == m::PX_3;
    let row = LayoutRect::new(
        x + TOGGLE_ROW_X,
        y + TOGGLE_ROW_Y,
        TOGGLE_ROW_WIDTH,
        TOGGLE_ROW_HEIGHT,
    );
    let active = themed || scenario.screen_state.is_button_focused();
    let border = choice_chrome::choice_row_border(
        palette,
        disabled,
        scenario.screen_state.preview_hovered,
        active,
    );
    choice_chrome::draw_choice_row_with_border(
        canvas,
        text,
        palette,
        row,
        "Markdown Linter",
        disabled,
        border,
    );
    let switch = toggle_switch_rect(x, y);
    switch_control::draw_switch_with_disabled(
        canvas,
        palette,
        switch.x,
        switch.y,
        switch.width,
        switch.height,
        on,
        disabled,
    );
}

#[cfg(test)]
pub(super) const fn toggle_switch_rect_for_test() -> super::layout_metrics::LayoutRect {
    toggle_switch_rect(
        super::preview_detail::HERO_PREVIEW_X_FOR_TEST,
        super::preview_detail::HERO_PREVIEW_Y_FOR_TEST,
    )
}

#[cfg(test)]
pub(super) const fn toggle_row_rect_for_test() -> super::layout_metrics::LayoutRect {
    toggle_row_rect(
        super::preview_detail::HERO_PREVIEW_X_FOR_TEST,
        super::preview_detail::HERO_PREVIEW_Y_FOR_TEST,
    )
}

pub(super) const fn toggle_row_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    super::layout_metrics::LayoutRect::new(
        x + TOGGLE_ROW_X,
        y + TOGGLE_ROW_Y,
        TOGGLE_ROW_WIDTH,
        TOGGLE_ROW_HEIGHT,
    )
}

pub(super) const fn toggle_switch_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    let row = toggle_row_rect(x, y);
    super::layout_metrics::LayoutRect::new(
        row.x + row.width - TOGGLE_SWITCH_RIGHT_INSET - TOGGLE_SWITCH_WIDTH,
        row.y + TOGGLE_SWITCH_Y_INSET,
        TOGGLE_SWITCH_WIDTH,
        TOGGLE_SWITCH_HEIGHT,
    )
}
