use super::{
    Canvas, layout_metrics, palette, panel_screen_state, preview_detail, render, screen_state,
    scrollbar, window_interaction,
};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const PANEL_PAGE: &str = "panel";
const NESTED_PRESET: usize = 3;
const PANEL_DIFF_THRESHOLD: usize = 1_800;
const PREVIEW_SLOT_X: usize = 174;
const PREVIEW_SLOT_Y: usize = 64;
const PREVIEW_SLOT_WIDTH: usize = 296;
const PREVIEW_SLOT_HEIGHT: usize = 192;
const CHILD_SCROLLBAR_RIGHT_INSET: usize = 12;
const CHILD_SCROLLBAR_TOP_INSET: usize = 8;
const CHILD_SCROLLBAR_TRACK_WIDTH: usize = 5;
const CHILD_SCROLLBAR_VERTICAL_INSET: usize = 16;

#[test]
fn storybook_root_scrollbar_is_hidden_by_default() {
    let canvas = render::render_storybook_canvas_for_preset(DARK_THEME, PANEL_PAGE, 0, 0);
    let accent = palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).accent;

    assert_ne!(
        Some(accent),
        pixel_at_rect(&canvas, scrollbar::thumb_rect(0)),
        "Storybook 全体の右端スクロールバーは既定で表示しない"
    );
}

#[test]
fn panel_inspector_setting_hides_panel_component_scrollbars() {
    let visible = render_panel(None);
    let mut state = screen_state::StorybookScreenState::default();
    state.register_settings_change(PANEL_PAGE);
    let hidden = render_panel(Some(state));
    let accent = palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).accent;

    assert!(color_count(&visible, preview_vertical_track(), accent) > 0);
    assert_eq!(
        0,
        color_count(&hidden, preview_vertical_track(), accent),
        "Panel の右メニュー設定は部品内スクロールバーを直接切り替える"
    );
}

#[test]
fn panel_right_inspector_click_toggles_panel_scrollbar_setting() {
    let mut state = window_interaction::StorybookWindowState {
        selected_page: PANEL_PAGE,
        preset_index: NESTED_PRESET,
        ..window_interaction::StorybookWindowState::default()
    };
    let setting = layout_metrics::panel_scrollbar_off_rect();

    assert!(window_interaction::apply_click(
        &mut state,
        setting.x + 1,
        setting.y + 1
    ));
    assert!(
        !state
            .screen_state
            .panel
            .child(panel_screen_state::PanelChildKey::Preview)
            .scrollbar_visible
    );
    assert_eq!(
        "panel.scrollbar_visibility",
        state.screen_state.last_setting
    );
}

#[test]
fn panel_preview_action_moves_inner_panel_scrollbar() {
    let before = render_panel(None);
    let mut state = screen_state::StorybookScreenState::default();
    state.register_preview_action(PANEL_PAGE);
    let after = render_panel(Some(state));
    let accent = palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).accent;

    let before_y = first_color_y(&before, preview_vertical_track(), accent);
    let after_y = first_color_y(&after, preview_vertical_track(), accent);

    assert!(
        after_y > before_y,
        "Panel In Panel のスクロールバーは操作後の scroll_y を反映する"
    );
}

#[test]
fn panel_story_presets_have_substantial_visual_differences() {
    let vertical = render_panel_preset(0);
    let horizontal = render_panel_preset(1);
    let toggle = render_panel_preset(2);
    let nested = render_panel_preset(NESTED_PRESET);
    let rect = preview_detail::component_action_hit_rect(PANEL_PAGE);

    for (label, diff) in [
        (
            "vertical/horizontal",
            region_pixel_diff(&vertical, &horizontal, rect),
        ),
        (
            "horizontal/toggle",
            region_pixel_diff(&horizontal, &toggle, rect),
        ),
        ("toggle/nested", region_pixel_diff(&toggle, &nested, rect)),
    ] {
        assert!(
            diff > PANEL_DIFF_THRESHOLD,
            "{label} preset diff was {diff}; Panel tabs must not be decorative"
        );
    }
}

fn render_panel(state: Option<screen_state::StorybookScreenState>) -> Canvas {
    render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: DARK_THEME,
        selected_page: PANEL_PAGE,
        preset_index: NESTED_PRESET,
        preset_tab_scroll_x: 0,
        scroll_y: 0,
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        tree_expansion: Default::default(),
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
        screen_state: state.unwrap_or_default(),
    })
}

fn render_panel_preset(preset_index: usize) -> Canvas {
    render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: DARK_THEME,
        selected_page: PANEL_PAGE,
        preset_index,
        preset_tab_scroll_x: 0,
        scroll_y: 0,
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        tree_expansion: Default::default(),
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
        screen_state: Default::default(),
    })
}

fn preview_vertical_track() -> layout_metrics::LayoutRect {
    layout_metrics::LayoutRect::new(
        preview_detail::HERO_PREVIEW_X_FOR_TEST + PREVIEW_SLOT_X + PREVIEW_SLOT_WIDTH
            - CHILD_SCROLLBAR_RIGHT_INSET,
        preview_detail::HERO_PREVIEW_Y_FOR_TEST + PREVIEW_SLOT_Y + CHILD_SCROLLBAR_TOP_INSET,
        CHILD_SCROLLBAR_TRACK_WIDTH,
        PREVIEW_SLOT_HEIGHT - CHILD_SCROLLBAR_VERTICAL_INSET,
    )
}

fn region_pixel_diff(before: &Canvas, after: &Canvas, rect: layout_metrics::LayoutRect) -> usize {
    let mut diff = 0;
    for current_y in rect.y..rect.bottom() {
        for current_x in rect.x..rect.right() {
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

fn first_color_y(canvas: &Canvas, rect: layout_metrics::LayoutRect, color: u32) -> usize {
    for current_y in rect.y..rect.bottom() {
        for current_x in rect.x..rect.right() {
            if pixel_at(canvas, current_x, current_y) == Some(color) {
                return current_y;
            }
        }
    }
    0
}
