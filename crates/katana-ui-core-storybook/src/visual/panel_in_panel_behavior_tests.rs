use super::{
    Canvas, layout_metrics, palette, panel_screen_state, preview_detail, render, screen_state,
    scrollbar, storybook_ui_option_contract, window_interaction,
};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const PANEL_PAGE: &str = "panel";
const VERTICAL_PRESET: usize = 1;
const HORIZONTAL_PRESET: usize = 2;
const SCROLLBAR_PRESET: usize = 3;
const NESTED_PRESET: usize = 4;
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
    apply_scrollbar_hidden_setting(&mut state);
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
    let active = render_panel_preset(0);
    let vertical = render_panel_preset(VERTICAL_PRESET);
    let horizontal = render_panel_preset(HORIZONTAL_PRESET);
    let toggle = render_panel_preset(SCROLLBAR_PRESET);
    let nested = render_panel_preset(NESTED_PRESET);
    let rect = preview_detail::component_action_hit_rect(PANEL_PAGE);

    for (label, diff) in [
        (
            "active/vertical",
            region_pixel_diff(&active, &vertical, rect),
        ),
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

#[test]
fn panel_live_operations_update_component_state_and_body() {
    let target = preview_detail::component_action_hit_rect(PANEL_PAGE);
    let action_x = target.x + 4;
    let action_y = target.y + 4;

    let mut hover_state = panel_window_state();
    let before_hover = render_panel(Some(hover_state.screen_state.clone()));
    assert!(window_interaction::apply_hover_at(
        &mut hover_state,
        action_x,
        action_y
    ));
    let after_hover = render_panel(Some(hover_state.screen_state.clone()));
    assert_eq!("panel_hover", hover_state.screen_state.last_action);
    assert_eq!("panel_hovered", hover_state.screen_state.last_event);
    assert!(hover_state.screen_state.panel.hovered);
    assert!(
        region_pixel_diff(&before_hover, &after_hover, target) > 0,
        "panel hover must repaint the component body"
    );

    let mut focus_state = panel_window_state();
    let before_focus = render_panel(Some(focus_state.screen_state.clone()));
    assert!(window_interaction::focus_clickable_at_for_audit(
        &mut focus_state,
        action_x,
        action_y
    ));
    let after_focus = render_panel(Some(focus_state.screen_state.clone()));
    assert_eq!("panel_focus", focus_state.screen_state.last_action);
    assert_eq!("panel_focused", focus_state.screen_state.last_event);
    assert!(focus_state.screen_state.panel.focused);
    assert!(
        region_pixel_diff(&before_focus, &after_focus, target) > 0,
        "panel focus must repaint the component body"
    );

    let before_keyboard = render_panel(Some(focus_state.screen_state.clone()));
    assert!(window_interaction::apply_clickable_keyboard_activation_for_audit(&mut focus_state));
    let after_keyboard = render_panel(Some(focus_state.screen_state.clone()));
    assert_eq!(
        "panel_keyboard_scroll",
        focus_state.screen_state.last_action
    );
    assert_eq!("panel_scroll_changed", focus_state.screen_state.last_event);
    assert!(
        region_pixel_diff(&before_keyboard, &after_keyboard, target) > 0,
        "panel keyboard action must repaint the component body"
    );

    let mut resize_state = panel_window_state();
    let before_resize = render_panel(Some(resize_state.screen_state.clone()));
    assert!(window_interaction::apply_panel_resize_for_audit(
        &mut resize_state,
        action_x,
        action_y
    ));
    let after_resize = render_panel(Some(resize_state.screen_state.clone()));
    assert_eq!("panel_resize", resize_state.screen_state.last_action);
    assert_eq!("panel_resized", resize_state.screen_state.last_event);
    assert!(resize_state.screen_state.panel.resized);
    assert!(
        region_pixel_diff(&before_resize, &after_resize, target) > 0,
        "panel resize must repaint the component body"
    );
}

fn render_panel(state: Option<screen_state::StorybookScreenState>) -> Canvas {
    render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: DARK_THEME,
        selected_page: PANEL_PAGE,
        selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
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

fn panel_window_state() -> window_interaction::StorybookWindowState {
    window_interaction::StorybookWindowState {
        selected_page: PANEL_PAGE,
        preset_index: NESTED_PRESET,
        ..window_interaction::StorybookWindowState::default()
    }
}

fn apply_scrollbar_hidden_setting(state: &mut screen_state::StorybookScreenState) {
    state.register_settings_contract_change(
        PANEL_PAGE,
        storybook_ui_option_contract::StorybookUiOptionContract::new(
            "scrollbar_visibility",
            "on",
            "off",
        ),
    );
}

fn render_panel_preset(preset_index: usize) -> Canvas {
    render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: DARK_THEME,
        selected_page: PANEL_PAGE,
        selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
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
