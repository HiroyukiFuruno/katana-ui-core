use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookWindowState, apply_hover_at, focus_clickable_at_for_audit,
};
use super::{StorybookVisual, palette, preview_detail, render, storybook_ui_option_contract};
use crate::StoryCatalog;
use crate::catalog::StoryPresetLabels;
use crate::test_assert::KucTestExpect;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "tooltip";
const ANCHOR_PRESET: usize = 0;
const HOVER_PRESET: usize = 1;
const EDGE_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const BUBBLE_X: usize = 112;
const BUBBLE_Y: usize = 34;
const BUBBLE_WIDTH: usize = 132;
const BUBBLE_HEIGHT: usize = 26;
const BUBBLE_SAMPLE_OFFSET: usize = 8;
const ANCHOR_X: usize = 134;
const ANCHOR_Y: usize = 72;
const ANCHOR_WIDTH: usize = 80;
const ANCHOR_HEIGHT: usize = 22;

#[test]
fn tooltip_exposes_leaf_presets_options_and_hover_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("tooltip_hover", spec.action);
    assert_eq!("tooltip_opened", spec.event);
    assert_eq!("interaction.hovered", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("hover=true", spec.state);
}

#[test]
fn tooltip_presets_render_distinct_anchor_hover_edge_and_theme_bodies() {
    let anchor = StorybookVisual.render_preset(DARK_THEME, PAGE, ANCHOR_PRESET, 0);
    let hover = StorybookVisual.render_preset(DARK_THEME, PAGE, HOVER_PRESET, 0);
    let edge = StorybookVisual.render_preset(DARK_THEME, PAGE, EDGE_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &anchor, &hover) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &hover, &edge) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &anchor, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn tooltip_setting_option_updates_overlay_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn tooltip_preview_action_opens_hover_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn tooltip_hover_and_focus_open_core_tooltip_state() {
    let mut hover_state = page_state();
    let before_hover = render_state(&hover_state);
    let anchor = tooltip_anchor_rect();
    assert!(apply_hover_at(&mut hover_state, anchor.x + 1, anchor.y + 1));
    let after_hover = render_state(&hover_state);
    assert_eq!("tooltip_hover", hover_state.screen_state.last_action);
    assert_eq!("tooltip_opened", hover_state.screen_state.last_event);
    assert_eq!(
        "hover=true focus=true",
        hover_state.screen_state.state_label
    );
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let mut focus_state = page_state();
    let before_focus = render_state(&focus_state);
    assert!(focus_clickable_at_for_audit(
        &mut focus_state,
        anchor.x + 1,
        anchor.y + 1
    ));
    let after_focus = render_state(&focus_state);
    assert_eq!("tooltip_focus", focus_state.screen_state.last_action);
    assert_eq!("tooltip_focused", focus_state.screen_state.last_event);
    assert_eq!(
        "hover=true focus=true",
        focus_state.screen_state.state_label
    );
    assert!(focus_state.screen_state.is_button_focused());
    assert!(component_body_pixel_diff(PAGE, &before_focus, &after_focus) > 0);
}

#[test]
fn tooltip_hover_bubble_is_visible_positioned_and_connected_to_anchor() {
    let mut state = page_state();
    let anchor = tooltip_anchor_rect();

    assert!(apply_hover_at(&mut state, anchor.x + 1, anchor.y + 1));

    let canvas = render_state(&state);
    let component = preview_detail::component_action_hit_rect(PAGE);
    let bubble = super::layout_metrics::LayoutRect::new(
        component.x + BUBBLE_X,
        component.y + BUBBLE_Y,
        BUBBLE_WIDTH,
        BUBBLE_HEIGHT,
    );
    let bubble_sample = pixel_at(
        &canvas,
        bubble.x + BUBBLE_SAMPLE_OFFSET,
        bubble.y + BUBBLE_SAMPLE_OFFSET,
    );
    assert!(component.contains(bubble.x, bubble.y));
    assert!(component.contains(bubble.x + bubble.width - 1, bubble.y + bubble.height - 1));
    assert!(
        bubble.y + bubble.height < anchor.y,
        "tooltip bubble must be visibly separated above its anchor"
    );
    assert!(
        bubble.x <= anchor.x + anchor.width / 2
            && anchor.x + anchor.width / 2 <= bubble.x + bubble.width,
        "tooltip bubble must horizontally cover the anchor center"
    );
    assert_ne!(
        pixel_at(
            &StorybookVisual.render_preset(DARK_THEME, PAGE, ANCHOR_PRESET, 0),
            bubble.x,
            bubble.y
        ),
        pixel_at(&canvas, bubble.x, bubble.y),
        "hover must draw a visible bubble border"
    );
    assert!(bubble_sample.is_some());
}

#[test]
fn tooltip_non_anchor_hover_and_focus_do_not_open_tooltip() {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let mut hover_state = page_state();

    assert!(!apply_hover_at(
        &mut hover_state,
        component.x + 1,
        component.y + 1
    ));
    assert_eq!("none", hover_state.screen_state.last_action);
    assert_eq!("none", hover_state.screen_state.last_event);
    assert_eq!("idle", hover_state.screen_state.state_label);

    let mut focus_state = page_state();
    assert!(!focus_clickable_at_for_audit(
        &mut focus_state,
        component.x + 1,
        component.y + 1
    ));
    assert_eq!("none", focus_state.screen_state.last_action);
    assert_eq!("none", focus_state.screen_state.last_event);
    assert_eq!("idle", focus_state.screen_state.state_label);
}

#[test]
fn tooltip_hover_leave_closes_open_bubble_without_click_event() {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let anchor = tooltip_anchor_rect();
    let mut state = page_state();

    assert!(apply_hover_at(&mut state, anchor.x + 1, anchor.y + 1));
    let opened = render_state(&state);
    assert_eq!("tooltip_hover", state.screen_state.last_action);
    assert_eq!("tooltip_opened", state.screen_state.last_event);

    assert!(apply_hover_at(&mut state, component.x + 1, component.y + 1));
    let closed = render_state(&state);

    assert_eq!("tooltip_hover", state.screen_state.last_action);
    assert_eq!("tooltip_closed", state.screen_state.last_event);
    assert_eq!("hover=false focus=false", state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &opened, &closed) > 0);
}

#[test]
fn tooltip_hover_is_idempotent_while_pointer_stays_inside_anchor() {
    let mut state = page_state();
    let anchor = tooltip_anchor_rect();

    assert!(apply_hover_at(&mut state, anchor.x + 1, anchor.y + 1));
    let action_count = state.screen_state.action_count;
    assert!(apply_hover_at(&mut state, anchor.x + 1, anchor.y + 1));

    assert_eq!(action_count, state.screen_state.action_count);
    assert_eq!("tooltip_hover", state.screen_state.last_action);
    assert_eq!("tooltip_opened", state.screen_state.last_event);
}

#[test]
fn tooltip_idle_does_not_render_open_bubble_until_hover_or_focus() {
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let idle = StorybookVisual.render_preset(DARK_THEME, PAGE, ANCHOR_PRESET, 0);
    let hover = StorybookVisual.render_preset(DARK_THEME, PAGE, HOVER_PRESET, 0);
    let component = preview_detail::component_action_hit_rect(PAGE);
    let border_x = component.x + BUBBLE_X;
    let border_y = component.y + BUBBLE_Y;
    let sample_x = component.x + BUBBLE_X + BUBBLE_SAMPLE_OFFSET;
    let sample_y = component.y + BUBBLE_Y + BUBBLE_SAMPLE_OFFSET;

    assert_ne!(
        Some(colors.border),
        pixel_at(&idle, border_x, border_y),
        "idle tooltip must not draw the closed bubble border"
    );
    assert_eq!(
        Some(colors.accent),
        pixel_at(&hover, sample_x, sample_y),
        "hover preset must reveal the bubble through the core hover-open state"
    );
}

#[test]
fn tooltip_story_connects_core_hover_callback_log() {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|example| example.page == PAGE)
        .kuc_expect("tooltip story exists");

    assert!(story.callback_logs.iter().any(|callback| {
        callback.action == "hover_start" && callback.after.contains("open=true")
    }));
}

#[test]
fn tooltip_light_and_dark_anchor_uses_theme_surface() {
    assert_anchor_token(DARK_THEME, ThemeSnapshot::dark());
    assert_anchor_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn page_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn tooltip_anchor_rect() -> super::layout_metrics::LayoutRect {
    let component = preview_detail::component_action_hit_rect(PAGE);
    super::layout_metrics::LayoutRect::new(
        component.x + ANCHOR_X,
        component.y + ANCHOR_Y,
        ANCHOR_WIDTH,
        ANCHOR_HEIGHT,
    )
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn assert_anchor_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, ANCHOR_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + ANCHOR_X + BUBBLE_SAMPLE_OFFSET,
            component.y + ANCHOR_Y + BUBBLE_SAMPLE_OFFSET
        )
    );
}
