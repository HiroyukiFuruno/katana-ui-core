use super::{StorybookVisual, preview_detail};

const DARK_THEME: &str = "dark";
const PAGE: &str = "checkbox";
const CHECKED_PRESET: usize = 1;
const DISABLED_PRESET: usize = 2;
const FOCUS_PRESET: usize = 3;
const PREVIEW_RIGHT_EDGE: usize = 1020;
const CHECK_GLYPH_COLOR: u32 = 0xf8fafc;

#[test]
fn checkbox_initial_snapshot_does_not_render_operation_history_as_current_state() {
    let unchecked = StorybookVisual.render_preset(DARK_THEME, PAGE, 0, 0);

    assert!(has_preview_text(&unchecked, "idle"));
    assert!(
        !has_preview_text(&unchecked, "before=false after=false"),
        "unchecked initial snapshot must not present a no-op before/after transition as if it were real operation history"
    );
    assert!(has_inspector_text(&unchecked, "screen: idle"));
}

#[test]
fn checkbox_checked_preset_reports_current_checked_state_in_preview_and_inspector() {
    let checked = StorybookVisual.render_preset(DARK_THEME, PAGE, CHECKED_PRESET, 0);

    assert!(
        !has_preview_text(&checked, "operation after / callback log visible"),
        "checked preset initial render must not claim an operation/callback happened"
    );
    assert!(has_preview_text(
        &checked,
        "preset state / public API visible"
    ));
    assert!(has_preview_text(&checked, "checked=true"));
    assert!(has_inspector_text(&checked, "screen: checked=true"));
    assert!(
        !has_preview_text(&checked, "before=false after=false"),
        "checked preset must not report false state while the mark is checked"
    );
    assert!(
        !has_inspector_text(&checked, "screen: idle"),
        "checked preset Inspector must not report idle while the mark is checked"
    );
}

#[test]
fn checkbox_disabled_preset_reports_current_disabled_state_in_preview_and_inspector() {
    let disabled = StorybookVisual.render_preset(DARK_THEME, PAGE, DISABLED_PRESET, 0);

    assert!(has_preview_text(&disabled, "disabled=true"));
    assert!(has_inspector_text(&disabled, "screen: disabled=true"));
    assert!(
        !has_inspector_text(&disabled, "screen: idle"),
        "disabled preset Inspector must not report idle while the rows are disabled"
    );
}

#[test]
fn checkbox_focus_preset_reports_current_focus_state_in_preview_and_inspector() {
    let focused = StorybookVisual.render_preset(DARK_THEME, PAGE, FOCUS_PRESET, 0);

    assert!(has_preview_text(&focused, "focused=true"));
    assert!(has_inspector_text(&focused, "screen: focused=true"));
    assert!(
        !has_preview_text(&focused, "before=false after=false"),
        "focus preset must not report checked false state while the row is focused"
    );
    assert!(
        !has_inspector_text(&focused, "screen: idle"),
        "focus preset Inspector must not report idle while the row is focused"
    );
}

#[test]
fn checkbox_clicked_checked_preset_renders_unchecked_state() {
    let clicked = StorybookVisual.render_clicked_preset_with_scrollbar(
        DARK_THEME,
        PAGE,
        CHECKED_PRESET,
        0,
        true,
    );
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let mark = super::dedicated_dod_form_binary_choice_live::checkbox_mark_rect(0, rect.x, rect.y);

    assert_eq!(0, count_color_in_rect(&clicked, mark, CHECK_GLYPH_COLOR));
}

#[test]
fn checkbox_clicked_disabled_preset_does_not_mutate_or_render_checked_state() {
    let clicked = StorybookVisual.render_clicked_preset_with_scrollbar(
        DARK_THEME,
        PAGE,
        DISABLED_PRESET,
        0,
        true,
    );
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let mark = super::dedicated_dod_form_binary_choice_live::checkbox_mark_rect(0, rect.x, rect.y);

    assert_eq!(0, count_color_in_rect(&clicked, mark, CHECK_GLYPH_COLOR));
    assert!(
        clicked
            .text_runs()
            .iter()
            .any(|run| run.text() == "checked=false"),
        "disabled clicked snapshot must keep public checked state false"
    );
    assert!(
        clicked
            .text_runs()
            .iter()
            .any(|run| run.text() == "count 0"),
        "disabled clicked snapshot must not increment action count through a Storybook-only shortcut"
    );
}

#[test]
fn checkbox_clicked_snapshot_keeps_preview_status_and_inspector_state_consistent() {
    let clicked =
        StorybookVisual.render_clicked_preset_with_scrollbar(DARK_THEME, PAGE, 0, 0, true);

    assert!(has_preview_text(&clicked, "checked=true"));
    assert!(has_preview_text(&clicked, "before=false after=true"));
    assert!(has_inspector_text(
        &clicked,
        "screen: before=false after=true"
    ));
    assert!(has_inspector_text(&clicked, "action: checkbox_toggle"));
    assert!(has_inspector_text(&clicked, "event: checked_changed"));
}

#[test]
fn checkbox_inspector_settings_rows_are_not_rendered_as_current_state_values() {
    let unchecked = StorybookVisual.render_preset(DARK_THEME, PAGE, 0, 0);

    assert!(
        !has_inspector_text(&unchecked, "disabled: false -> true"),
        "checkbox Inspector must not render configurable options as if they were current state values"
    );
    assert!(has_inspector_text(
        &unchecked,
        "option.disabled: false -> true"
    ));
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

fn has_preview_text(canvas: &super::Canvas, text: &str) -> bool {
    canvas
        .text_runs()
        .iter()
        .any(|run| run.text() == text && run.x() < PREVIEW_RIGHT_EDGE)
}

fn has_inspector_text(canvas: &super::Canvas, text: &str) -> bool {
    canvas
        .text_runs()
        .iter()
        .any(|run| run.text() == text && run.x() > PREVIEW_RIGHT_EDGE)
}
