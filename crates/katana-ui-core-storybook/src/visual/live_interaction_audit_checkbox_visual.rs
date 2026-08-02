use crate::visual::{
    Canvas, StorybookVisual, dedicated_dod_form_binary_choice_live, palette, preview_detail,
    render,
    window_interaction::{StorybookWindowState, apply_hover_at},
};
use katana_ui_core::theme::ThemeSnapshot;

use super::{StorybookLiveInteractionScenario, page_state, scenario};

const CHECKBOX_PAGE: &str = "checkbox";
const CHECKED_PRESET_INDEX: usize = 1;
const DISABLED_PRESET_INDEX: usize = 2;
const FOCUS_PRESET_INDEX: usize = 3;
const PREVIEW_RIGHT_EDGE: usize = 1020;
const MIN_FOCUS_BORDER_PIXELS: usize = 8;
const MIN_MARK_SIZE: usize = 20;
const MIN_ROW_HEIGHT: usize = 36;
const MIN_ROW_STATUS_GAP: usize = 16;
const HOVER_OFFSET: usize = 4;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page != CHECKBOX_PAGE {
        return Vec::new();
    }
    vec![
        checkbox_focus_labels_visible_scenario(),
        checkbox_focus_single_active_row_scenario(),
        checkbox_focus_preset_state_consistency_scenario(),
        checkbox_checked_preset_state_consistency_scenario(),
        checkbox_disabled_preset_state_consistency_scenario(),
        checkbox_inspector_options_are_labeled_as_options_scenario(),
        checkbox_disabled_controls_are_muted_scenario(),
        checkbox_disabled_hover_is_muted_scenario(),
        checkbox_modern_spacing_scenario(),
    ]
}

fn checkbox_focus_labels_visible_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    state.select_preset(FOCUS_PRESET_INDEX);
    let focused = StorybookVisual.render_preset("dark", CHECKBOX_PAGE, FOCUS_PRESET_INDEX, 0);
    let component = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    let first_label =
        dedicated_dod_form_binary_choice_live::checkbox_label_rect(0, component.x, component.y);
    let second_label =
        dedicated_dod_form_binary_choice_live::checkbox_label_rect(1, component.x, component.y);
    let text_color = palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).text;
    let passed = count_color_in_rect(&focused, first_label, text_color) > 0
        && count_color_in_rect(&focused, second_label, text_color) > 0;
    scenario(
        CHECKBOX_PAGE,
        "checkbox_focus_labels_visible",
        "visual",
        true,
        passed,
        0,
        &state,
    )
}

fn checkbox_focus_single_active_row_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    state.select_preset(FOCUS_PRESET_INDEX);
    let focused = StorybookVisual.render_preset("dark", CHECKBOX_PAGE, FOCUS_PRESET_INDEX, 0);
    let component = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    let first_row = dedicated_dod_form_binary_choice_live::row_rect(0, component.x, component.y);
    let second_row = dedicated_dod_form_binary_choice_live::row_rect(1, component.x, component.y);
    let accent = palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).accent;
    let passed = count_color_in_rect(&focused, first_row, accent) >= MIN_FOCUS_BORDER_PIXELS
        && count_color_in_rect(&focused, second_row, accent) == 0;
    scenario(
        CHECKBOX_PAGE,
        "checkbox_focus_single_active_row",
        "visual",
        true,
        passed,
        0,
        &state,
    )
}

fn checkbox_focus_preset_state_consistency_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    state.select_preset(FOCUS_PRESET_INDEX);
    let focused = StorybookVisual.render_preset("dark", CHECKBOX_PAGE, FOCUS_PRESET_INDEX, 0);
    let passed = has_preview_text(&focused, "focused=true")
        && has_inspector_text(&focused, "screen: focused=true")
        && !has_preview_text(&focused, "before=false after=false")
        && !has_inspector_text(&focused, "screen: idle");
    scenario(
        CHECKBOX_PAGE,
        "checkbox_focus_preset_state_consistency",
        "visual",
        true,
        passed,
        0,
        &state,
    )
}

fn checkbox_checked_preset_state_consistency_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    state.select_preset(CHECKED_PRESET_INDEX);
    let checked = StorybookVisual.render_preset("dark", CHECKBOX_PAGE, CHECKED_PRESET_INDEX, 0);
    let passed = has_preview_text(&checked, "checked=true")
        && has_inspector_text(&checked, "screen: checked=true")
        && !has_preview_text(&checked, "before=false after=false")
        && !has_inspector_text(&checked, "screen: idle");
    scenario(
        CHECKBOX_PAGE,
        "checkbox_checked_preset_state_consistency",
        "visual",
        true,
        passed,
        0,
        &state,
    )
}

fn checkbox_disabled_preset_state_consistency_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    state.select_preset(DISABLED_PRESET_INDEX);
    let disabled = StorybookVisual.render_preset("dark", CHECKBOX_PAGE, DISABLED_PRESET_INDEX, 0);
    let passed = has_preview_text(&disabled, "disabled=true")
        && has_inspector_text(&disabled, "screen: disabled=true")
        && !has_inspector_text(&disabled, "screen: idle");
    scenario(
        CHECKBOX_PAGE,
        "checkbox_disabled_preset_state_consistency",
        "visual",
        true,
        passed,
        0,
        &state,
    )
}

fn checkbox_inspector_options_are_labeled_as_options_scenario() -> StorybookLiveInteractionScenario
{
    let state = page_state(CHECKBOX_PAGE);
    let unchecked = StorybookVisual.render_preset("dark", CHECKBOX_PAGE, 0, 0);
    let passed = !has_inspector_text(&unchecked, "disabled: false -> true")
        && has_inspector_text(&unchecked, "option.disabled: false -> true");
    scenario(
        CHECKBOX_PAGE,
        "checkbox_inspector_options_are_labeled",
        "visual",
        true,
        passed,
        0,
        &state,
    )
}

fn checkbox_disabled_controls_are_muted_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    state.select_preset(DISABLED_PRESET_INDEX);
    let disabled = StorybookVisual.render_preset("dark", CHECKBOX_PAGE, DISABLED_PRESET_INDEX, 0);
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let component = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    let controls = [
        dedicated_dod_form_binary_choice_live::checkbox_state_read_button_rect(
            component.x,
            component.y,
        ),
        dedicated_dod_form_binary_choice_live::checkbox_toggle_button_rect(
            component.x,
            component.y,
        ),
        dedicated_dod_form_binary_choice_live::checkbox_reset_button_rect(component.x, component.y),
    ];
    let passed = controls.iter().all(|rect| {
        count_color_in_rect(&disabled, *rect, palette.text) == 0
            && count_color_in_rect(&disabled, *rect, palette.muted) > 0
    });
    scenario(
        CHECKBOX_PAGE,
        "checkbox_disabled_controls_are_muted",
        "visual",
        true,
        passed,
        0,
        &state,
    )
}

fn checkbox_disabled_hover_is_muted_scenario() -> StorybookLiveInteractionScenario {
    let mut window_state = StorybookWindowState {
        selected_page: CHECKBOX_PAGE,
        ..StorybookWindowState::default()
    };
    window_state.select_preset(DISABLED_PRESET_INDEX);
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let component = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    let row = dedicated_dod_form_binary_choice_live::row_rect(0, component.x, component.y);
    let changed = apply_hover_at(
        &mut window_state,
        row.x + HOVER_OFFSET,
        row.y + HOVER_OFFSET,
    );
    let hovered = render::render_storybook_canvas_with_screen_state(
        "dark",
        CHECKBOX_PAGE,
        window_state.preset_index,
        window_state.screen_state.clone(),
    );
    let passed = changed && count_color_in_rect(&hovered, row, palette.hover_border) == 0;
    scenario(
        CHECKBOX_PAGE,
        "checkbox_disabled_hover_is_muted",
        "visual",
        true,
        passed,
        0,
        &window_state,
    )
}

fn checkbox_modern_spacing_scenario() -> StorybookLiveInteractionScenario {
    let state = page_state(CHECKBOX_PAGE);
    let component = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    let row = dedicated_dod_form_binary_choice_live::row_rect(0, component.x, component.y);
    let mark =
        dedicated_dod_form_binary_choice_live::checkbox_mark_rect(0, component.x, component.y);
    let status =
        dedicated_dod_form_binary_choice_live::checkbox_state_row_rect(component.x, component.y);
    let passed = mark.width >= MIN_MARK_SIZE
        && mark.height >= MIN_MARK_SIZE
        && row.height >= MIN_ROW_HEIGHT
        && status.x >= row.right() + MIN_ROW_STATUS_GAP;
    scenario(
        CHECKBOX_PAGE,
        "checkbox_modern_spacing",
        "visual",
        true,
        passed,
        0,
        &state,
    )
}

fn count_color_in_rect(
    canvas: &Canvas,
    rect: crate::visual::layout_metrics::LayoutRect,
    color: u32,
) -> usize {
    (rect.y..rect.bottom())
        .flat_map(|y| (rect.x..rect.right()).map(move |x| (x, y)))
        .filter(|(x, y)| pixel_at(canvas, *x, *y) == Some(color))
        .count()
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    (x < canvas.width() && y < canvas.height())
        .then(|| y * canvas.width() + x)
        .and_then(|index| canvas.pixels().get(index))
        .copied()
}

fn has_preview_text(canvas: &Canvas, text: &str) -> bool {
    canvas
        .text_runs()
        .iter()
        .any(|run| run.text() == text && run.x() < PREVIEW_RIGHT_EDGE)
}

fn has_inspector_text(canvas: &Canvas, text: &str) -> bool {
    canvas
        .text_runs()
        .iter()
        .any(|run| run.text() == text && run.x() > PREVIEW_RIGHT_EDGE)
}
