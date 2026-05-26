use super::{
    Canvas, layout_metrics, palette, panel_scroll_state, panel_scrollbars, preview_detail, render,
    screen_state,
};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const PANEL_PAGE: &str = "panel";
const DEFAULT_PRESET: usize = 0;
const PANEL_DIFF_THRESHOLD: usize = 180;
const PREVIEW_SLOT_X: usize = 174;
const PREVIEW_SLOT_Y: usize = 64;
const PREVIEW_SLOT_WIDTH: usize = 296;
const PREVIEW_SLOT_HEIGHT: usize = 192;
const CHILD_SCROLLBAR_RIGHT_INSET: usize = 12;
const CHILD_SCROLLBAR_TOP_INSET: usize = 8;
const CHILD_SCROLLBAR_TRACK_WIDTH: usize = 5;
const CHILD_SCROLLBAR_VERTICAL_INSET: usize = 16;

#[test]
fn panel_story_keeps_storybook_preview_scrollbars_hidden_for_panel_playground() {
    let canvas = render_panel_with_offsets(Default::default(), true);
    let accent = palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).accent;

    assert_eq!(
        Some(accent),
        pixel_at_rect(
            &canvas,
            panel_scrollbars::thumb_rect_for(
                panel_scroll_state::PanelScrollRegion::Navigation,
                Default::default(),
            ),
        )
    );
    assert_ne!(
        Some(accent),
        pixel_at_rect(
            &canvas,
            panel_scrollbars::thumb_rect_for_state(
                panel_scroll_state::PanelScrollRegion::Preview,
                Default::default(),
                PANEL_PAGE,
                Default::default(),
            ),
        )
    );
    assert_ne!(
        Some(accent),
        pixel_at_rect(
            &canvas,
            panel_scrollbars::horizontal_thumb_rect_for_state(
                panel_scroll_state::PanelScrollRegion::Preview,
                Default::default(),
                PANEL_PAGE,
                Default::default(),
            ),
        )
    );
}

#[test]
fn panel_story_scrollbar_toggle_hides_preview_scrollbar_pixels() {
    let visible = render_panel_with_offsets(Default::default(), true);
    let mut state = screen_state::StorybookScreenState::default();
    state.register_settings_change(PANEL_PAGE);
    let hidden = render_panel_with_state(state);
    let accent = palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).accent;

    assert!(color_count(&visible, preview_component_vertical_track(), accent) > 0);
    assert_eq!(
        0,
        color_count(&hidden, preview_component_vertical_track(), accent)
    );
}

#[test]
fn panel_story_preview_action_moves_inner_panel_surface() {
    let baseline = render_panel_with_offsets(Default::default(), true);
    let mut state = screen_state::StorybookScreenState::default();
    state.register_preview_action(PANEL_PAGE);
    let scrolled = render_panel_with_state(state);
    let rect = preview_detail::component_action_hit_rect(PANEL_PAGE);

    assert_eq!(
        0,
        region_pixel_diff(
            &baseline,
            &scrolled,
            0,
            0,
            layout_metrics::PREVIEW_X,
            render::HEIGHT
        )
    );
    assert!(
        region_pixel_diff(
            &baseline,
            &scrolled,
            rect.x,
            rect.y,
            rect.width,
            rect.height
        ) > PANEL_DIFF_THRESHOLD
    );
}

fn preview_component_vertical_track() -> layout_metrics::LayoutRect {
    layout_metrics::LayoutRect::new(
        preview_detail::HERO_PREVIEW_X_FOR_TEST + PREVIEW_SLOT_X + PREVIEW_SLOT_WIDTH
            - CHILD_SCROLLBAR_RIGHT_INSET,
        preview_detail::HERO_PREVIEW_Y_FOR_TEST + PREVIEW_SLOT_Y + CHILD_SCROLLBAR_TOP_INSET,
        CHILD_SCROLLBAR_TRACK_WIDTH,
        PREVIEW_SLOT_HEIGHT - CHILD_SCROLLBAR_VERTICAL_INSET,
    )
}

fn render_panel_with_offsets(
    offsets: panel_scroll_state::PanelScrollOffsets,
    scrollbar_visible: bool,
) -> Canvas {
    render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: DARK_THEME,
        selected_page: PANEL_PAGE,
        preset_index: DEFAULT_PRESET,
        scroll_y: 0,
        scrollbar_visible,
        panel_scroll: offsets,
        tree_expansion: Default::default(),
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
        screen_state: Default::default(),
    })
}

fn render_panel_with_state(screen_state: screen_state::StorybookScreenState) -> Canvas {
    render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: DARK_THEME,
        selected_page: PANEL_PAGE,
        preset_index: DEFAULT_PRESET,
        scroll_y: 0,
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        tree_expansion: Default::default(),
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
        screen_state,
    })
}

fn region_pixel_diff(
    before: &Canvas,
    after: &Canvas,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> usize {
    let mut diff = 0;
    for current_y in y..y + height {
        for current_x in x..x + width {
            let index = current_y * before.width() + current_x;
            if before.pixels()[index] != after.pixels()[index] {
                diff += 1;
            }
        }
    }
    diff
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}

fn pixel_at_rect(canvas: &Canvas, rect: layout_metrics::LayoutRect) -> Option<u32> {
    pixel_at(canvas, rect.x + rect.width / 2, rect.y + rect.height / 2)
}

fn color_count(canvas: &Canvas, rect: layout_metrics::LayoutRect, color: u32) -> usize {
    let mut count = 0;
    for current_y in rect.y..rect.bottom() {
        for current_x in rect.x..rect.right() {
            if pixel_at(canvas, current_x, current_y) == Some(color) {
                count += 1;
            }
        }
    }
    count
}
