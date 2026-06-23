use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, focus_clickable_at_for_audit,
};
use crate::visual::{
    dedicated_context_menu_popup, dedicated_dod_molecule_menu, dedicated_menu_button,
    layout_metrics::LayoutRect, preview_detail,
};

use super::{
    StorybookLiveInteractionScenario, component_body_pixel_diff, page_state, render_state, scenario,
};

const BUTTON_PAGES: &[&str] = &["button", "text-button", "svg-button", "icon-text-button"];
const CONTEXT_MENU_PAGE: &str = "context-menu";
const MENU_PAGE: &str = "menu";
const MENU_BUTTON_PAGE: &str = "menu-button";
const SHORTCUT_COMBO_PAGE: &str = "shortcut-combo";
const CLICK_OFFSET: usize = 4;

#[path = "live_interaction_audit_clickable_shortcut.rs"]
mod shortcut;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page == SHORTCUT_COMBO_PAGE {
        return shortcut::scenarios();
    }
    if page == CONTEXT_MENU_PAGE {
        return vec![
            context_menu_focus_scenario(),
            context_menu_keyboard_select_scenario(),
        ];
    }
    if page == MENU_PAGE {
        return vec![menu_focus_scenario(), menu_keyboard_open_scenario()];
    }
    if page == MENU_BUTTON_PAGE {
        return vec![
            menu_button_focus_scenario(),
            menu_button_keyboard_open_scenario(),
        ];
    }
    if !BUTTON_PAGES.contains(&page) {
        return Vec::new();
    }
    vec![button_focus_scenario(page), button_keyboard_scenario(page)]
}

fn context_menu_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CONTEXT_MENU_PAGE);
    let before = render_state(CONTEXT_MENU_PAGE, &state);
    let target = context_menu_focus_target();
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(CONTEXT_MENU_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CONTEXT_MENU_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "context_menu_focus"
        && state.screen_state.last_event == "context_menu_focused"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        CONTEXT_MENU_PAGE,
        "context_menu_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn context_menu_keyboard_select_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CONTEXT_MENU_PAGE);
    let target = context_menu_focus_target();
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(CONTEXT_MENU_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(CONTEXT_MENU_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CONTEXT_MENU_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "context_menu_keyboard_select"
        && state.screen_state.last_event == "context_menu_item_selected"
        && state.screen_state.state_label == "context_menu.selected=[1]"
        && body_pixel_diff > 0;
    scenario(
        CONTEXT_MENU_PAGE,
        "context_menu_keyboard_select",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn context_menu_focus_target() -> LayoutRect {
    let component = preview_detail::component_action_hit_rect(CONTEXT_MENU_PAGE);
    dedicated_context_menu_popup::insert_row_rect(component.x, component.y)
}

fn menu_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(MENU_PAGE);
    let before = render_state(MENU_PAGE, &state);
    let target = dedicated_dod_molecule_menu::first_row_rect(
        preview_detail::component_action_hit_rect(MENU_PAGE),
    );
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(MENU_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(MENU_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "menu_focus"
        && state.screen_state.last_event == "menu_focused"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        MENU_PAGE,
        "menu_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn menu_keyboard_open_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(MENU_PAGE);
    let target = dedicated_dod_molecule_menu::first_row_rect(
        preview_detail::component_action_hit_rect(MENU_PAGE),
    );
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(MENU_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(MENU_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(MENU_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "menu_keyboard_open"
        && state.screen_state.last_event == "menu_opened"
        && state.screen_state.selection.select_open
        && body_pixel_diff > 0;
    scenario(
        MENU_PAGE,
        "menu_keyboard_open",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn menu_button_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(MENU_BUTTON_PAGE);
    let before = render_state(MENU_BUTTON_PAGE, &state);
    let target = dedicated_menu_button::trigger_rect(preview_detail::component_action_hit_rect(
        MENU_BUTTON_PAGE,
    ));
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(MENU_BUTTON_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(MENU_BUTTON_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "menu_button_focus"
        && state.screen_state.last_event == "menu_button_focused"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        MENU_BUTTON_PAGE,
        "menu_button_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn menu_button_keyboard_open_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(MENU_BUTTON_PAGE);
    let target = dedicated_menu_button::trigger_rect(preview_detail::component_action_hit_rect(
        MENU_BUTTON_PAGE,
    ));
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(MENU_BUTTON_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(MENU_BUTTON_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(MENU_BUTTON_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "menu_button_keyboard_open"
        && state.screen_state.last_event == "menu_button_opened"
        && state.screen_state.selection.select_open
        && body_pixel_diff > 0;
    scenario(
        MENU_BUTTON_PAGE,
        "menu_button_keyboard_open",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn button_focus_scenario(page: &'static str) -> StorybookLiveInteractionScenario {
    let mut state = page_state(page);
    let before = render_state(page, &state);
    let target = preview_detail::button_action_hit_rect(page);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(page, &state);
    let body_pixel_diff = component_body_pixel_diff(page, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "button_focus"
        && state.screen_state.last_event == "button_focused"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        page,
        "button_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn button_keyboard_scenario(page: &'static str) -> StorybookLiveInteractionScenario {
    let mut state = page_state(page);
    let target = preview_detail::button_action_hit_rect(page);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(page, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(page, &state);
    let body_pixel_diff = component_body_pixel_diff(page, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_event.ends_with("_clicked")
        && state.screen_state.is_button_pressed()
        && body_pixel_diff > 0;
    scenario(
        page,
        "button_keyboard_activate",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}
