use super::window_interaction::StorybookWindowState;
use super::{dedicated_dod_layout_scroll_area, preview_detail, render};

const SCROLL_AREA_PAGE: &str = "scroll-area";

#[test]
fn scroll_area_geometry_guard_keeps_debug_status_outside_viewport() {
    let state = StorybookWindowState {
        selected_page: SCROLL_AREA_PAGE,
        scrollbar_visible: true,
        ..StorybookWindowState::default()
    };
    let rendered = render_layout_window_state(&state);
    let frame = preview_detail::component_action_hit_rect(SCROLL_AREA_PAGE);
    let expected_frame = dedicated_dod_layout_scroll_area::frame_rect(frame.x, frame.y);
    let viewport = dedicated_dod_layout_scroll_area::viewport_rect(frame.x, frame.y);
    let content = dedicated_dod_layout_scroll_area::content_clip_rect(frame.x, frame.y);
    let scrollbar = dedicated_dod_layout_scroll_area::scrollbar_drag_rect(frame.x, frame.y);
    let resize = dedicated_dod_layout_scroll_area::resize_handle_rect(frame.x, frame.y);

    assert_eq!(
        expected_frame, frame,
        "scroll-area hit rect must cover the rendered component frame"
    );
    assert!(viewport.inside_content());
    assert!(content.inside_content());
    assert!(
        frame.bottom() >= viewport.bottom(),
        "viewport must be inside the component frame"
    );
    assert!(
        content.x >= viewport.x
            && content.y >= viewport.y
            && content.right() <= viewport.right()
            && content.bottom() <= viewport.bottom(),
        "scroll content clip must stay inside the viewport"
    );

    for status in dedicated_dod_layout_scroll_area::status_rects(frame.x, frame.y) {
        assert!(
            status.bottom() <= frame.bottom(),
            "status/debug row must fit inside the component frame"
        );
        assert!(
            status.y >= viewport.bottom() + 8,
            "status/debug row must not be visually attached to the viewport body"
        );
        assert!(
            !status.overlaps(viewport),
            "status/debug row must not overlap scroll viewport"
        );
        assert!(
            !status.overlaps(scrollbar),
            "status/debug row must not overlap scrollbar"
        );
    }

    for run in rendered
        .text_runs()
        .iter()
        .filter(|run| frame.overlaps(run.rect()))
    {
        assert!(
            !run.rect().overlaps(scrollbar),
            "text run {:?} must not overlap the scrollbar",
            run.text()
        );
        assert!(
            !run.rect().overlaps(resize),
            "text run {:?} must not overlap the resize handle",
            run.text()
        );
        if matches!(
            run.text(),
            "action ready"
                | "event ready"
                | "state idle"
                | "action scroll"
                | "event scrolled"
                | "state offset"
                | "action drag"
                | "state drag"
                | "action resize"
                | "event resized"
                | "state viewport"
        ) {
            assert!(
                !run.rect().overlaps(viewport),
                "storybook status text {:?} must not be drawn inside the scroll viewport",
                run.text()
            );
        }
    }
}

fn render_layout_window_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: state.theme_id,
        selected_page: state.selected_page,
        selected_instance_id: state.selected_instance_id,
        preset_index: state.preset_index,
        preset_tab_scroll_x: state.preset_tab_scroll_x,
        scroll_y: state.scroll_y,
        scrollbar_visible: state.scrollbar_visible,
        panel_scroll: state.panel_scroll,
        tree_expansion: state.tree_expansion,
        show_navigation_lines: state.show_navigation_lines,
        show_navigation_text_connectors: state.show_navigation_text_connectors,
        screen_state: state.screen_state.clone(),
    })
}
