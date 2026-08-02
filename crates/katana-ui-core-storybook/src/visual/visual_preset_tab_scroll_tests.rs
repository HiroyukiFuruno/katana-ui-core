use super::palette::VisualPalette;
use super::visual_interaction_test_support::require_some;
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_scroll_delta_at_for_test,
};
use super::{Canvas, StorybookVisual, layout_metrics, preset_tab_scroll, preview_detail};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const PAGE: &str = "text-input";

#[test]
fn overflowing_preset_tabs_have_horizontal_scroll_range() {
    assert!(StoryPresetLabels::for_page(PAGE).len() > layout_metrics::PRESET_TAB_COUNT);
    assert_eq!(
        layout_metrics::PRESET_WIDTH * StoryPresetLabels::for_page(PAGE).len() - preview_width(),
        preset_tab_scroll::max_scroll_x_for_page(PAGE)
    );
    assert_eq!(0, preset_tab_scroll::max_scroll_x_for_page("theme-tokens"));
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

    let last_preset = last_preset_index();
    state.select_preset(last_preset);

    assert_eq!(last_preset, state.preset_index);
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
    let last_preset = last_preset_index();
    let rect = require_some(
        preset_tab_scroll::visual_rect_for_index(
            PAGE,
            last_preset,
            false,
            state.preset_tab_scroll_x,
        ),
        "last preset should be visible after scrolling",
    )?;

    assert!(apply_click(&mut state, rect.x + 1, rect.y + 1));

    assert_eq!(last_preset, state.preset_index);
    assert!(active_tab_is_inside_viewport(&state));
    Ok(())
}

#[test]
fn preset_tab_hit_bounds_reject_gap_and_clipped_edges() -> Result<(), String> {
    let viewport = preset_tab_scroll::viewport_rect();
    let scroll_x = 1;
    let first_visible = require_some(
        preset_tab_scroll::visual_rect_for_index(PAGE, 1, false, scroll_x),
        "first fully visible tab should be exposed after a partial scroll",
    )?;
    let hit_y = first_visible.y + first_visible.height / 2;

    assert_eq!(
        None,
        preset_tab_scroll::hit_index_at(PAGE, viewport.x, hit_y, scroll_x)
    );
    assert_eq!(
        Some(1),
        preset_tab_scroll::hit_index_at(PAGE, first_visible.x, hit_y, scroll_x)
    );
    assert_eq!(
        None,
        preset_tab_scroll::hit_index_at(PAGE, viewport.right(), hit_y, scroll_x)
    );
    assert_eq!(
        None,
        preset_tab_scroll::hit_index_at(PAGE, first_visible.x, viewport.y - 1, scroll_x)
    );
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
fn preset_tab_scroll_handles_empty_pages_zero_delta_reverse_wheel_and_missing_labels() {
    let viewport = preset_tab_scroll::viewport_rect();
    assert_eq!(0, preset_tab_scroll::ensure_index_visible("unknown", 4, 8));
    assert_eq!(0, preset_tab_scroll::max_scroll_x_for_page("unknown"));
    assert_eq!(0, preset_tab_scroll::scroll_delta("unknown", 8, -1.0));
    assert_eq!(10, preset_tab_scroll::scroll_delta(PAGE, 10, 0.0));
    assert_eq!(
        0,
        preset_tab_scroll::scroll_delta(PAGE, layout_metrics::PRESET_WIDTH, 1.0)
    );
    assert_eq!(
        Some(0),
        preset_tab_scroll::hit_index_at("unknown", viewport.x + 1, viewport.y + 1, 0)
    );
    assert_eq!(
        None,
        preset_tab_scroll::hit_index_at("unknown", viewport.right() - 1, viewport.y + 1, 0)
    );
}

#[test]
fn external_render_preset_scrolls_active_overflow_tab_into_view() -> Result<(), String> {
    let last_preset = last_preset_index();
    let canvas = StorybookVisual.render_preset("dark", PAGE, last_preset, 0);
    let scroll_x = preset_tab_scroll::active_index_scroll_x(PAGE, last_preset);
    let rect = require_some(
        preset_tab_scroll::visual_rect_for_index(PAGE, last_preset, true, scroll_x),
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

fn last_preset_index() -> usize {
    StoryPresetLabels::for_page(PAGE).len() - 1
}
