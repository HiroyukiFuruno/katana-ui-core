use super::palette::VisualPalette;
use super::visual_interaction_test_support::require_some;
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_scroll_delta_at_for_test,
};
use super::{Canvas, StorybookVisual, layout_metrics, preset_tab_scroll, preview_detail};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const PAGE: &str = "text-input";
const LAST_PRESET: usize = 8;

#[test]
fn overflowing_preset_tabs_have_horizontal_scroll_range() {
    assert!(StoryPresetLabels::for_page(PAGE).len() > layout_metrics::PRESET_TAB_COUNT);
    assert_eq!(
        layout_metrics::PRESET_WIDTH * StoryPresetLabels::for_page(PAGE).len() - preview_width(),
        preset_tab_scroll::max_scroll_x_for_page(PAGE)
    );
    assert_eq!(0, preset_tab_scroll::max_scroll_x_for_page("button"));
}

#[test]
fn preset_tab_viewport_matches_preview_surface_width() {
    let viewport = preset_tab_scroll::viewport_rect();
    let (preview_x, _, preview_width, _) = preview_detail::selected_hero_rect();

    assert_eq!(preview_x, viewport.x);
    assert_eq!(preview_x + preview_width, viewport.right());
}

#[test]
fn visible_preset_tab_rects_stay_fully_inside_viewport() -> Result<(), String> {
    assert_visible_tabs_inside_viewport(0)?;
    assert_visible_tabs_inside_viewport(preset_tab_scroll::max_scroll_x_for_page(PAGE))?;
    Ok(())
}

#[test]
fn rendered_preset_tabs_are_clipped_at_preview_right_edge() {
    let canvas = StorybookVisual.render_preset("dark", PAGE, 0, 0);
    let palette = VisualPalette::from_theme(&ThemeSnapshot::dark());
    let viewport = preset_tab_scroll::viewport_rect();

    assert_eq!(
        Some(palette.background),
        pixel_at(
            &canvas,
            viewport.right() + 1,
            viewport.y + viewport.height / 2
        )
    );
}

#[test]
fn external_preset_selection_scrolls_current_tab_into_view() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    state.select_preset(LAST_PRESET);

    assert_eq!(LAST_PRESET, state.preset_index);
    assert_eq!(
        preset_tab_scroll::max_scroll_x_for_page(PAGE),
        state.preset_tab_scroll_x
    );
    assert!(active_tab_is_inside_viewport(&state));
}

#[test]
fn clicking_scrolled_preset_tab_uses_logical_tab_index() -> Result<(), String> {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        preset_tab_scroll_x: preset_tab_scroll::max_scroll_x_for_page(PAGE),
        ..StorybookWindowState::default()
    };
    let rect = require_some(
        preset_tab_scroll::visual_rect_for_index(
            PAGE,
            LAST_PRESET,
            false,
            state.preset_tab_scroll_x,
        ),
        "last preset should be visible after scrolling",
    )?;

    assert!(apply_click(&mut state, rect.x + 1, rect.y + 1));

    assert_eq!(LAST_PRESET, state.preset_index);
    assert!(active_tab_is_inside_viewport(&state));
    Ok(())
}

#[test]
fn wheel_over_preset_tabs_scrolls_tabs_without_scrolling_root() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let viewport = preset_tab_scroll::viewport_rect();

    assert!(apply_scroll_delta_at_for_test(
        &mut state,
        viewport.x + 1,
        viewport.y + 1,
        -1.0
    ));

    assert_eq!(layout_metrics::PRESET_WIDTH, state.preset_tab_scroll_x);
    assert_eq!(0, state.scroll_y);
}

#[test]
fn external_render_preset_scrolls_active_overflow_tab_into_view() -> Result<(), String> {
    let canvas = StorybookVisual.render_preset("dark", PAGE, LAST_PRESET, 0);
    let scroll_x = preset_tab_scroll::active_index_scroll_x(PAGE, LAST_PRESET);
    let rect = require_some(
        preset_tab_scroll::visual_rect_for_index(PAGE, LAST_PRESET, true, scroll_x),
        "active preset should be visible in external render",
    )?;
    let palette = VisualPalette::from_theme(&ThemeSnapshot::dark());

    assert_eq!(
        Some(palette.accent),
        pixel_at(&canvas, rect.x + rect.width / 2, rect.bottom() - 1)
    );
    Ok(())
}

fn active_tab_is_inside_viewport(state: &StorybookWindowState) -> bool {
    let viewport = preset_tab_scroll::viewport_rect();
    let Some(rect) = preset_tab_scroll::visual_rect_for_index(
        state.selected_page,
        state.preset_index,
        true,
        state.preset_tab_scroll_x,
    ) else {
        return false;
    };
    viewport.contains(rect.x, rect.y) && rect.right() <= viewport.right()
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}

fn assert_visible_tabs_inside_viewport(scroll_x: usize) -> Result<(), String> {
    let viewport = preset_tab_scroll::viewport_rect();
    for index in preset_tab_scroll::visible_index_range(PAGE, scroll_x) {
        let rect = require_some(
            preset_tab_scroll::visual_rect_for_index(PAGE, index, false, scroll_x),
            "visible range must expose a rect",
        )?;

        assert!(viewport.contains(rect.x, rect.y));
        assert!(rect.right() <= viewport.right());
    }
    Ok(())
}

fn preview_width() -> usize {
    let (_, _, width, _) = preview_detail::selected_hero_rect();
    width
}
