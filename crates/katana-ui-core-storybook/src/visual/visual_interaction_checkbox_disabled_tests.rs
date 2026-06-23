use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_hover_at, focus_clickable_at_for_audit,
};
use super::{dedicated_dod_form_binary_choice_live, palette, preview_detail, render};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const PAGE: &str = "checkbox";
const CLICK_OFFSET: usize = 4;
const DISABLED_PRESET: usize = 2;
const CHECKBOX_ACCENT: u32 = 0x569cd6;
const CHECKBOX_GLYPH: u32 = 0xf8fafc;

#[test]
fn checkbox_disabled_preset_blocks_focus_and_keyboard_toggle() {
    let mut state = checkbox_state();
    state.select_preset(DISABLED_PRESET);
    let row = checkbox_row();
    let mark = checkbox_mark();
    let before = render_checkbox(&state);
    let before_mark_accent = count_color_in_rect(&before, mark, CHECKBOX_ACCENT);
    let before_mark_glyph = count_color_in_rect(&before, mark, CHECKBOX_GLYPH);

    assert!(focus_clickable_at_for_audit(
        &mut state,
        row.x + CLICK_OFFSET,
        row.y + CLICK_OFFSET
    ));
    assert_eq!("checkbox_focus_blocked", state.screen_state.last_action);
    assert_eq!("checkbox_focus_ignored", state.screen_state.last_event);
    assert_eq!("disabled=true", state.screen_state.state_label);
    assert!(!state.screen_state.is_checkbox_focused());

    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    assert_eq!("checkbox_keyboard_blocked", state.screen_state.last_action);
    assert_eq!("checkbox_keyboard_ignored", state.screen_state.last_event);
    assert_eq!("disabled=true", state.screen_state.state_label);
    assert!(!state.screen_state.is_checkbox_checked());

    let after = render_checkbox(&state);
    assert_eq!(
        before_mark_accent,
        count_color_in_rect(&after, mark, CHECKBOX_ACCENT),
        "disabled checkbox preset must not add or remove checked accent from blocked focus or keyboard activation"
    );
    assert_eq!(
        before_mark_glyph,
        count_color_in_rect(&after, mark, CHECKBOX_GLYPH),
        "disabled checkbox preset must not add or remove check glyph from blocked focus or keyboard activation"
    );
}

#[test]
fn checkbox_disabled_preset_blocks_pointer_toggle_and_preserves_mark() {
    let mut state = checkbox_state();
    state.select_preset(DISABLED_PRESET);
    let row = checkbox_row();
    let mark = checkbox_mark();
    let before = render_checkbox(&state);
    let before_mark_accent = count_color_in_rect(&before, mark, CHECKBOX_ACCENT);
    let before_mark_glyph = count_color_in_rect(&before, mark, CHECKBOX_GLYPH);

    assert!(apply_click(
        &mut state,
        row.x + CLICK_OFFSET,
        row.y + CLICK_OFFSET
    ));

    assert_eq!(0, state.screen_state.action_count);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!("none", state.screen_state.last_event);
    assert_eq!("disabled=true", state.screen_state.state_label);
    assert!(!state.screen_state.is_checkbox_checked());

    let after = render_checkbox(&state);
    assert_eq!(
        before_mark_accent,
        count_color_in_rect(&after, mark, CHECKBOX_ACCENT),
        "disabled checkbox preset must not add or remove checked accent from blocked pointer activation"
    );
    assert_eq!(
        before_mark_glyph,
        count_color_in_rect(&after, mark, CHECKBOX_GLYPH),
        "disabled checkbox preset must not add or remove check glyph from blocked pointer activation"
    );
}

#[test]
fn checkbox_disabled_preset_does_not_render_hover_as_enabled() {
    let mut state = checkbox_state();
    state.select_preset(DISABLED_PRESET);
    let row = checkbox_row();
    let hover_border = palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).hover_border;

    assert!(apply_hover_at(
        &mut state,
        row.x + CLICK_OFFSET,
        row.y + CLICK_OFFSET
    ));
    let hovered = render_checkbox(&state);

    assert_eq!(
        0,
        count_color_in_rect(&hovered, row, hover_border),
        "disabled checkbox preset must not paint enabled hover feedback"
    );
}

fn checkbox_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn render_checkbox(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn checkbox_row() -> super::layout_metrics::LayoutRect {
    let component = preview_detail::component_action_hit_rect(PAGE);
    dedicated_dod_form_binary_choice_live::checkbox_row_rect(0, component.x, component.y)
}

fn checkbox_mark() -> super::layout_metrics::LayoutRect {
    let component = preview_detail::component_action_hit_rect(PAGE);
    dedicated_dod_form_binary_choice_live::checkbox_mark_rect(0, component.x, component.y)
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
