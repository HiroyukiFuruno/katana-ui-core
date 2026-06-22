use crate::visual::dedicated_tabs_metrics::{
    STRIP_LEADING_INSET, STRIP_X, TAB_GAP, TAB_HEIGHT, TAB_Y, tab_width,
};
use crate::visual::preview_detail;
use crate::visual::screen_state_tabs::TabsScreenTab;
use crate::visual::window_interaction::{
    apply_context_click, apply_tabs_drag_at_for_audit, apply_tabs_keyboard_shortcut,
    focus_tabs_at_for_audit, release_tabs_drag_for_audit, start_tabs_drag_at_for_audit,
};
use katana_ui_core::widget::molecules::{CloseableTabKey, CloseableTabKeyboardShortcut};

use super::{
    StorybookLiveInteractionScenario, component_body_pixel_diff, page_state, render_state, scenario,
};

const TABS_PAGE: &str = "tabs";
const CLOSEABLE_TAB_STRIP_PAGE: &str = "closeable-tab-strip";
const TAB_POINTER_INSET: usize = 4;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    match page {
        TABS_PAGE => vec![
            tab_keyboard_select_scenario(TABS_PAGE),
            tab_focus_scenario(TABS_PAGE),
            tab_context_menu_scenario(TABS_PAGE),
            tab_drag_reorder_scenario(TABS_PAGE),
        ],
        CLOSEABLE_TAB_STRIP_PAGE => vec![
            tab_keyboard_select_scenario(CLOSEABLE_TAB_STRIP_PAGE),
            tab_focus_scenario(CLOSEABLE_TAB_STRIP_PAGE),
            tab_context_menu_scenario(CLOSEABLE_TAB_STRIP_PAGE),
            tab_drag_reorder_scenario(CLOSEABLE_TAB_STRIP_PAGE),
        ],
        _ => Vec::new(),
    }
}

fn tab_focus_scenario(page: &'static str) -> StorybookLiveInteractionScenario {
    let mut state = page_state(page);
    let before = render_state(page, &state);
    let component = preview_detail::component_action_hit_rect(page);
    let focused = focus_tabs_at_for_audit(
        &mut state,
        component.x + STRIP_X + STRIP_LEADING_INSET + TAB_POINTER_INSET,
        component.y + TAB_Y + TAB_HEIGHT / 2,
    );
    let after = render_state(page, &state);
    let body_pixel_diff = component_body_pixel_diff(page, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "tab_focus"
        && state.screen_state.last_event == "closeable_tab_focused"
        && state.screen_state.tabs.focused_tab_id.as_deref() == Some("readme.md")
        && body_pixel_diff > 0;
    scenario(
        page,
        "tab_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn tab_drag_reorder_scenario(page: &'static str) -> StorybookLiveInteractionScenario {
    let mut state = page_state(page);
    state
        .screen_state
        .tabs
        .tabs
        .push(TabsScreenTab::new("guide.md", "guide").pinned(true));
    state.screen_state.tabs.active_tab_id = "readme.md".to_string();
    let before = render_state(page, &state);
    let component = preview_detail::component_action_hit_rect(page);
    let readme = TabsScreenTab::new("readme.md", "readme").pinned(true);
    let guide = TabsScreenTab::new("guide.md", "guide").pinned(true);
    let readme_width = tab_width(&readme);
    let guide_width = tab_width(&guide);
    let tab_y = component.y + TAB_Y + TAB_HEIGHT / 2;
    let first_tab_x = component.x + STRIP_X + STRIP_LEADING_INSET;
    let guide_x = first_tab_x + readme_width + TAB_GAP;
    let started = start_tabs_drag_at_for_audit(&mut state, first_tab_x + readme_width / 2, tab_y);
    let moved = apply_tabs_drag_at_for_audit(&mut state, guide_x + guide_width - 1, tab_y);
    let released = release_tabs_drag_for_audit(&mut state);
    let after = render_state(page, &state);
    let body_pixel_diff = component_body_pixel_diff(page, &before, &after);
    let passed = started
        && moved
        && released
        && state.screen_state.last_action == "tab_drag_end"
        && state.screen_state.last_event == "closeable_tab_drag_ended"
        && state.screen_state.state_label == "tabs.dragging=false"
        && body_pixel_diff > 0;
    scenario(
        page,
        "tab_drag_reorder",
        "drag",
        moved,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn tab_keyboard_select_scenario(page: &'static str) -> StorybookLiveInteractionScenario {
    let mut state = page_state(page);
    let before = render_state(page, &state);
    let selected = apply_tabs_keyboard_shortcut(
        &mut state,
        CloseableTabKeyboardShortcut::new(CloseableTabKey::Digit(2), true, false),
    );
    let after = render_state(page, &state);
    let body_pixel_diff = component_body_pixel_diff(page, &before, &after);
    let passed = selected
        && state.screen_state.last_action == "tab_keyboard_select_visible"
        && state.screen_state.last_event == "closeable_tab_selected"
        && state.screen_state.tabs.active_tab_id == "editor.rs"
        && body_pixel_diff > 0;
    scenario(
        page,
        "tab_keyboard_select_visible",
        "keyboard",
        selected,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn tab_context_menu_scenario(page: &'static str) -> StorybookLiveInteractionScenario {
    let mut state = page_state(page);
    let before = render_state(page, &state);
    let component = preview_detail::component_action_hit_rect(page);
    let opened = apply_context_click(
        &mut state,
        component.x + STRIP_X + STRIP_LEADING_INSET + TAB_POINTER_INSET,
        component.y + TAB_Y + TAB_HEIGHT / 2,
    );
    let after = render_state(page, &state);
    let body_pixel_diff = component_body_pixel_diff(page, &before, &after);
    let passed = opened
        && state.screen_state.last_action == "tab_context_menu"
        && state.screen_state.last_event == "closeable_tab_context_menu_opened"
        && body_pixel_diff > 0;
    scenario(
        page,
        "tab_context_menu_open",
        "context_menu",
        opened,
        passed,
        body_pixel_diff,
        &state,
    )
}
