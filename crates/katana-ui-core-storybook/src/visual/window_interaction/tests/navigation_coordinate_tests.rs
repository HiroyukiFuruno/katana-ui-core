use super::super::{StorybookWindowState, apply_click, click_content_y};
use crate::catalog::story_map::StoryGroup;
use crate::visual::navigation_tree::{NavigationRow, row_from_click};
use crate::visual::{
    layout_metrics, panel_scroll_state::PanelScrollOffsets, render, window_coordinates,
};

const RETINA_WINDOW_WIDTH: usize = 2879;
const RETINA_WINDOW_HEIGHT: usize = 1728;
const NAV_TARGET_SCROLL_OFFSET: usize = 320;

#[test]
fn click_mapping_remains_stable_with_retina_letterboxed_window_and_scrolled_navigation_tree() {
    let mut state = StorybookWindowState {
        panel_scroll: PanelScrollOffsets {
            root_y: layout_metrics::SCROLL_STEP,
            navigation_y: layout_metrics::SCROLL_STEP + 12,
            ..PanelScrollOffsets::default()
        },
        scroll_y: layout_metrics::SCROLL_STEP,
        ..StorybookWindowState::default()
    };
    let target = navigation_page_target_after_scroll(NAV_TARGET_SCROLL_OFFSET, &state);
    let Some((target_page, target_group, logical_y)) = target else {
        unreachable!("navigation row is not found for test scenario");
    };
    assert!(matches!(
        row_from_click(layout_metrics::NAV_ROW_X + 1, logical_y, state.tree_expansion),
        Some(
            NavigationRow::Page {
                page: actual_page,
                group: actual_group,
                ..
            } | NavigationRow::PageWithoutSection {
                page: actual_page,
                group: actual_group,
            }
        ) if actual_page == target_page && actual_group == target_group
    ));

    let visible_y =
        logical_y.saturating_sub(state.panel_scroll.root_y + state.panel_scroll.navigation_y);
    assert!(visible_y < render::HEIGHT);

    let canvas_point = WindowPoint2d {
        x: (layout_metrics::NAV_ROW_X + 1) as f32 + 0.25,
        y: visible_y as f32 + 0.25,
    };
    let window_point = canvas_to_window_point(
        canvas_point,
        SurfaceSize2d {
            width: RETINA_WINDOW_WIDTH,
            height: RETINA_WINDOW_HEIGHT,
        },
        SurfaceSize2d {
            width: render::WIDTH,
            height: render::HEIGHT,
        },
    );

    let normalized = window_coordinates::window_point_to_canvas_point(
        window_coordinates::WindowPoint::new(window_point.x, window_point.y),
        window_coordinates::SurfaceSize::new(RETINA_WINDOW_WIDTH, RETINA_WINDOW_HEIGHT),
        window_coordinates::SurfaceSize::new(render::WIDTH, render::HEIGHT),
    )
    .unwrap_or(window_coordinates::CanvasPoint { x: 0, y: 0 });
    assert_ne!(
        normalized,
        window_coordinates::CanvasPoint { x: 0, y: 0 },
        "click point must stay within scaled canvas"
    );

    assert_eq!(
        (
            canvas_point.x.floor() as usize,
            canvas_point.y.floor() as usize
        ),
        (normalized.x, normalized.y),
    );

    let content_y = click_content_y(&state, normalized.x, normalized.y);
    let row_from_canvas = row_from_click(
        normalized.x,
        content_y + state.panel_scroll.navigation_y,
        state.tree_expansion,
    );
    assert!(matches!(
        row_from_canvas,
        Some(
            NavigationRow::Page { page: matched_page, .. }
            | NavigationRow::PageWithoutSection { page: matched_page, .. }
        ) if matched_page == target_page
    ));

    assert!(apply_click(&mut state, normalized.x, content_y));
    assert_eq!(target_page, state.selected_page);
}

#[test]
fn row_hit_rect_keeps_right_edge_exclusive_under_scaled_window_coordinates() {
    let state = StorybookWindowState::default();
    let x_inside = layout_metrics::NAV_ROW_X + layout_metrics::NAV_ROW_WIDTH - 1;
    let x_outside = layout_metrics::NAV_ROW_X + layout_metrics::NAV_ROW_WIDTH;
    let y = layout_metrics::NAV_FIRST_ROW_Y + 1;

    assert!(matches!(
        row_from_click(x_inside, y, state.tree_expansion),
        Some(NavigationRow::Group(_))
    ));
    assert_eq!(None, row_from_click(x_outside, y, state.tree_expansion));
}

fn navigation_page_target_after_scroll(
    minimum_y: usize,
    state: &StorybookWindowState,
) -> Option<(&'static str, StoryGroup, usize)> {
    for y in minimum_y..layout_metrics::CONTENT_HEIGHT {
        if let Some(
            NavigationRow::Page { page, group, .. }
            | NavigationRow::PageWithoutSection { page, group },
        ) = row_from_click(layout_metrics::NAV_ROW_X + 1, y, state.tree_expansion)
        {
            return Some((page, group, y));
        }
    }
    None
}

#[derive(Debug, Clone, Copy)]
struct WindowPoint2d {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy)]
struct SurfaceSize2d {
    width: usize,
    height: usize,
}

fn canvas_to_window_point(
    point: WindowPoint2d,
    window: SurfaceSize2d,
    canvas: SurfaceSize2d,
) -> WindowPoint2d {
    let canvas_rect = rendered_canvas_rect(window, canvas);
    WindowPoint2d {
        x: canvas_rect.x + point.x * canvas_rect.width / canvas.width as f32,
        y: canvas_rect.y + point.y * canvas_rect.height / canvas.height as f32,
    }
}

#[derive(Debug, Clone, Copy)]
struct RenderedCanvasRect2d {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

fn rendered_canvas_rect(window: SurfaceSize2d, canvas: SurfaceSize2d) -> RenderedCanvasRect2d {
    let window_aspect = window.width as f32 / window.height as f32;
    let canvas_aspect = canvas.width as f32 / canvas.height as f32;
    if canvas_aspect > window_aspect {
        let height = window.width as f32 / canvas_aspect;
        return RenderedCanvasRect2d {
            x: 0.0,
            y: (window.height as f32 - height) / 2.0,
            width: window.width as f32,
            height,
        };
    }
    let width = window.height as f32 * canvas_aspect;
    RenderedCanvasRect2d {
        x: (window.width as f32 - width) / 2.0,
        y: 0.0,
        width,
        height: window.height as f32,
    }
}
