use super::canvas::Canvas;
use super::dedicated_dod_form_binary_choice_live;
use super::dedicated_dod_form_input_live;
use super::dedicated_dod_form_segmented_live;
use super::dedicated_dod_form_select_live;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) fn input(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    dedicated_dod_form_input_live::input(canvas, text, palette, scenario, x, y);
}
pub(super) fn search(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    dedicated_dod_form_input_live::search(canvas, text, palette, scenario, x, y);
}
pub(super) fn select_box(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    dedicated_dod_form_select_live::select_box(canvas, text, palette, scenario, x, y);
}
pub(super) fn checkbox(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    dedicated_dod_form_binary_choice_live::checkbox(canvas, text, palette, scenario, x, y);
}
pub(super) fn radio(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    dedicated_dod_form_binary_choice_live::radio(canvas, text, palette, scenario, x, y);
}
pub(super) fn segmented(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    dedicated_dod_form_segmented_live::segmented(canvas, text, palette, scenario, x, y);
}
