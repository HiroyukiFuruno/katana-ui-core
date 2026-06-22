use super::StorybookVisual;
use super::{dedicated_dod_form_binary_choice_live, preview_detail};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const PAGE: &str = "checkbox";
const UNCHECKED_PRESET: usize = 0;
const FOCUS_PRESET: usize = 3;
const DISABLED_PRESET: usize = 2;
const MIN_READABLE_TEXT_RUN_HEIGHT: usize = 16;
const MIN_CONTROL_BOTTOM_PADDING: usize = 8;
const MIN_FOCUS_BORDER_PIXELS: usize = 8;

#[test]
fn checkbox_labels_controls_and_status_use_readable_text_runs() {
    let unchecked = StorybookVisual.render_preset(DARK_THEME, PAGE, UNCHECKED_PRESET, 0);
    for label in [
        "Markdown Linter",
        "Strict mode",
        "state read",
        "toggle",
        "reset",
        "checked=false",
        "event ready",
    ] {
        let run = unchecked
            .text_runs()
            .iter()
            .find(|run| run.text() == label)
            .kuc_expect("missing checkbox text run");
        assert!(
            run.height() >= MIN_READABLE_TEXT_RUN_HEIGHT,
            "{label} text run is too small for checkbox manual review"
        );
    }
}

#[test]
fn checkbox_preview_does_not_draw_storybook_runtime_overlay_over_controls() {
    let clicked = StorybookVisual.render_clicked_preset_with_scrollbar(
        DARK_THEME,
        PAGE,
        UNCHECKED_PRESET,
        0,
        true,
    );

    assert!(
        clicked
            .text_runs()
            .iter()
            .all(|run| run.text() != "clicked 1"),
        "checkbox preview must not draw Storybook runtime overlay text over core controls"
    );
}

#[test]
fn checkbox_controls_have_bottom_padding_inside_component_frame() {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let reset =
        dedicated_dod_form_binary_choice_live::checkbox_reset_button_rect(component.x, component.y);

    assert!(
        reset.bottom() + MIN_CONTROL_BOTTOM_PADDING <= component.bottom(),
        "checkbox control row must not sit flush against or outside the component frame"
    );
}

#[test]
fn checkbox_focus_preset_keeps_row_labels_visible() {
    let focused = StorybookVisual.render_preset(DARK_THEME, PAGE, FOCUS_PRESET, 0);
    let component = preview_detail::component_action_hit_rect(PAGE);
    let first_label =
        dedicated_dod_form_binary_choice_live::checkbox_label_rect(0, component.x, component.y);
    let second_label =
        dedicated_dod_form_binary_choice_live::checkbox_label_rect(1, component.x, component.y);
    let text_color = super::palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).text;

    assert!(count_color_in_rect(&focused, first_label, text_color) > 0);
    assert!(count_color_in_rect(&focused, second_label, text_color) > 0);
}

#[test]
fn checkbox_focus_preset_only_draws_focus_ring_on_active_row() {
    let focused = StorybookVisual.render_preset(DARK_THEME, PAGE, FOCUS_PRESET, 0);
    let component = preview_detail::component_action_hit_rect(PAGE);
    let first_row =
        dedicated_dod_form_binary_choice_live::checkbox_row_rect(0, component.x, component.y);
    let second_row =
        dedicated_dod_form_binary_choice_live::checkbox_row_rect(1, component.x, component.y);
    let accent = super::palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).accent;

    assert!(count_color_in_rect(&focused, first_row, accent) >= MIN_FOCUS_BORDER_PIXELS);
    assert_eq!(
        0,
        count_color_in_rect(&focused, second_row, accent),
        "checkbox focus preset must not show focus on more than one row"
    );
}

#[test]
fn checkbox_disabled_preset_mutes_control_button_labels() {
    let disabled = StorybookVisual.render_preset(DARK_THEME, PAGE, DISABLED_PRESET, 0);
    let palette = super::palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let component = preview_detail::component_action_hit_rect(PAGE);
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

    for rect in controls {
        assert_eq!(
            0,
            count_color_in_rect(&disabled, rect, palette.text),
            "disabled checkbox controls must not render enabled text color"
        );
        assert!(
            count_color_in_rect(&disabled, rect, palette.muted) > 0,
            "disabled checkbox controls must render muted text color"
        );
    }
}

fn count_color_in_rect(
    canvas: &super::Canvas,
    rect: super::layout_metrics::LayoutRect,
    color: u32,
) -> usize {
    (rect.y..rect.bottom())
        .flat_map(|y| (rect.x..rect.right()).map(move |x| (x, y)))
        .filter(|(x, y)| pixel_at(canvas, *x, *y) == Some(color))
        .count()
}

fn pixel_at(canvas: &super::Canvas, x: usize, y: usize) -> Option<u32> {
    if x >= canvas.width() || y >= canvas.height() {
        return None;
    }
    Some(canvas.pixels()[y * canvas.width() + x])
}
use crate::test_assert::KucTestExpect;
