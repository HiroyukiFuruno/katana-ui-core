use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, palette, preview_detail};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const CHECKBOX_PAGE: &str = "checkbox";
const RADIO_PAGE: &str = "radio";
const THEME_PAGE: &str = "theme-tokens";
const TEXT_PAGE: &str = "text";
const ICON_PAGE: &str = "icon";
const LOADING_DOTS_PAGE: &str = "loading-dots";
const SPINNER_PAGE: &str = "spinner";
const PROGRESS_PAGE: &str = "progress-bar";
const KEY_CAP_PAGE: &str = "key-cap";
const INPUT_PAGE: &str = "text-input";
const SEARCH_PAGE: &str = "search-box";
const SELECT_BOX_PAGE: &str = "select-box";
const SEGMENTED_PAGE: &str = "segmented-toggle";
const COLOR_SWATCH_PAGE: &str = "color-swatch";
const TOOLTIP_PAGE: &str = "tooltip";
const POPOVER_PAGE: &str = "popover";
const ACCORDION_PAGE: &str = "accordion";
const SPLIT_PANE_PAGE: &str = "split-pane";
const MODAL_PAGE: &str = "modal";
const MODAL_OVERLAY_PAGE: &str = "modal-overlay";
const COLOR_PICKER_PAGE: &str = "color-picker-rgba";
const CODE_DIFF_PAGE: &str = "code-diff";
const BADGE_PAGE: &str = "badge";
const CARD_PAGE: &str = "card";
const TOGGLE_PAGE: &str = "toggle";
const TREE_VIEW_PAGE: &str = "tree-view";
const DEFAULT_PRESET: usize = 0;
const EDGE_PRESET: usize = 2;
const COMPONENT_BODY_DIFF_THRESHOLD: usize = 80;
const TREE_SCROLL_TRACK_X_OFFSET: usize = 186;
const TREE_SCROLL_TRACK_Y_OFFSET: usize = 32;
const TREE_SCROLL_THUMB_EDGE_OFFSET: usize = 24;
const BRIGHT_PIXEL_THRESHOLD: u32 = 180;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const BLUE_SHIFT: u32 = 0;
const CHANNEL_MASK: u32 = 0xff;
const LUMINANCE_RED_WEIGHT: u32 = 299;
const LUMINANCE_GREEN_WEIGHT: u32 = 587;
const LUMINANCE_BLUE_WEIGHT: u32 = 114;
const LUMINANCE_SCALE: u32 = 1000;

#[test]
fn settings_change_updates_passive_atom_preview_bodies() {
    for page in [
        THEME_PAGE,
        TEXT_PAGE,
        ICON_PAGE,
        LOADING_DOTS_PAGE,
        SPINNER_PAGE,
        PROGRESS_PAGE,
        KEY_CAP_PAGE,
    ] {
        assert_settings_page_changes_body(page);
    }
}

#[test]
fn clicked_toggle_updates_visible_switch_body() {
    let before = StorybookVisual.render_preset(DARK_THEME, TOGGLE_PAGE, DEFAULT_PRESET, 0);
    let after = StorybookVisual.render_clicked_preset_with_scrollbar(
        DARK_THEME,
        TOGGLE_PAGE,
        DEFAULT_PRESET,
        0,
        true,
    );
    let rect = preview_detail::component_action_hit_rect(TOGGLE_PAGE);
    let switch_rect = super::dedicated_dod_atom_buttons::toggle_switch_rect_for_test();

    assert!(rect.width >= super::dedicated_dod_atom_buttons::TOGGLE_ROW_WIDTH);
    assert!(
        left_bright_pixel_count(switch_rect, &before)
            > right_bright_pixel_count(switch_rect, &before)
    );
    assert!(
        right_bright_pixel_count(switch_rect, &after)
            > left_bright_pixel_count(switch_rect, &after)
    );
    assert!(
        component_body_pixel_diff(TOGGLE_PAGE, &before, &after) > COMPONENT_BODY_DIFF_THRESHOLD
    );
}

#[test]
fn checkbox_and_radio_clicks_change_material_control_bodies() {
    for page in [CHECKBOX_PAGE, RADIO_PAGE] {
        assert_clicked_page_changes_body(page);
    }
}

#[test]
fn clicked_operable_pages_update_preview_body() {
    for page in [
        INPUT_PAGE,
        SEARCH_PAGE,
        SELECT_BOX_PAGE,
        SEGMENTED_PAGE,
        COLOR_SWATCH_PAGE,
        TOOLTIP_PAGE,
        POPOVER_PAGE,
        ACCORDION_PAGE,
        SPLIT_PANE_PAGE,
        MODAL_PAGE,
        MODAL_OVERLAY_PAGE,
        COLOR_PICKER_PAGE,
        CODE_DIFF_PAGE,
        BADGE_PAGE,
        KEY_CAP_PAGE,
        CARD_PAGE,
    ] {
        assert_clicked_page_changes_body(page);
    }
}

#[test]
fn tree_view_preview_has_independent_vertical_scroll_thumb() {
    let before = StorybookVisual.render_preset(DARK_THEME, TREE_VIEW_PAGE, DEFAULT_PRESET, 0);
    let after = StorybookVisual.render_preset(DARK_THEME, TREE_VIEW_PAGE, EDGE_PRESET, 0);
    let rect = preview_detail::component_action_hit_rect(TREE_VIEW_PAGE);
    let thumb_x = rect.x + TREE_SCROLL_TRACK_X_OFFSET;
    let thumb_y = rect.y + TREE_SCROLL_TRACK_Y_OFFSET + TREE_SCROLL_THUMB_EDGE_OFFSET;

    assert_eq!(
        Some(palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).accent),
        pixel_at(&after, thumb_x, thumb_y)
    );
    assert!(
        component_body_pixel_diff(TREE_VIEW_PAGE, &before, &after) > COMPONENT_BODY_DIFF_THRESHOLD
    );
}

fn left_bright_pixel_count(
    rect: super::layout_metrics::LayoutRect,
    canvas: &super::Canvas,
) -> usize {
    bright_pixel_count(rect.x, rect.y, rect.width / 2, rect.height, canvas)
}

fn right_bright_pixel_count(
    rect: super::layout_metrics::LayoutRect,
    canvas: &super::Canvas,
) -> usize {
    bright_pixel_count(
        rect.x + rect.width / 2,
        rect.y,
        rect.width / 2,
        rect.height,
        canvas,
    )
}

fn bright_pixel_count(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    canvas: &super::Canvas,
) -> usize {
    let mut count = 0;
    for current_y in y..y + height {
        for current_x in x..x + width {
            let index = current_y * canvas.width() + current_x;
            if luminance(canvas.pixels()[index]) > BRIGHT_PIXEL_THRESHOLD {
                count += 1;
            }
        }
    }
    count
}

fn luminance(color: u32) -> u32 {
    let red = (color >> RED_SHIFT) & CHANNEL_MASK;
    let green = (color >> GREEN_SHIFT) & CHANNEL_MASK;
    let blue = (color >> BLUE_SHIFT) & CHANNEL_MASK;
    (red * LUMINANCE_RED_WEIGHT + green * LUMINANCE_GREEN_WEIGHT + blue * LUMINANCE_BLUE_WEIGHT)
        / LUMINANCE_SCALE
}
