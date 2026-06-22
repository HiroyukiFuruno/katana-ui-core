use crate::visual::window_interaction::{apply_click, apply_context_click};
use crate::visual::{dedicated_dod_molecule_menu, dedicated_menu_button, preview_detail};

use super::{
    CLICK_OFFSET, StorybookLiveInteractionScenario, component_body_pixel_diff, page_state,
    render_state, scenario,
};

const CONTEXT_MENU_PAGE: &str = "context-menu";
const MENU_PAGE: &str = "menu";
const MENU_BUTTON_PAGE: &str = "menu-button";
const TREE_VIEW_PAGE: &str = "tree-view";

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    match page {
        CONTEXT_MENU_PAGE => vec![
            context_menu_open_scenario(),
            context_menu_outside_dismiss_scenario(),
        ],
        MENU_PAGE => vec![menu_context_dismiss_scenario()],
        MENU_BUTTON_PAGE => vec![menu_button_context_open_scenario()],
        TREE_VIEW_PAGE => vec![tree_view_context_menu_scenario()],
        _ => Vec::new(),
    }
}

fn context_menu_outside_dismiss_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CONTEXT_MENU_PAGE);
    let component = preview_detail::component_action_hit_rect(CONTEXT_MENU_PAGE);
    let opened = apply_context_click(
        &mut state,
        component.x + CLICK_OFFSET,
        component.y + CLICK_OFFSET,
    );
    let before = render_state(CONTEXT_MENU_PAGE, &state);
    let dismissed = apply_context_click(
        &mut state,
        component.x + component.width + CLICK_OFFSET,
        component.y + CLICK_OFFSET,
    );
    let after = render_state(CONTEXT_MENU_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CONTEXT_MENU_PAGE, &before, &after);
    let passed = opened
        && dismissed
        && state.screen_state.last_action == "context_menu_outside_dismiss"
        && state.screen_state.last_event == "context_menu_closed"
        && state.screen_state.state_label == "context_menu=closed"
        && body_pixel_diff > 0;
    scenario(
        CONTEXT_MENU_PAGE,
        "context_menu_outside_dismiss",
        "context_menu",
        dismissed,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn menu_context_dismiss_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(MENU_PAGE);
    let component = preview_detail::component_action_hit_rect(MENU_PAGE);
    let row = dedicated_dod_molecule_menu::first_row_rect(component);
    let opened = apply_click(&mut state, row.x + CLICK_OFFSET, row.y + CLICK_OFFSET);
    let before = render_state(MENU_PAGE, &state);
    let dismissed = apply_context_click(
        &mut state,
        component.x + component.width + CLICK_OFFSET,
        component.y + CLICK_OFFSET,
    );
    let after = render_state(MENU_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(MENU_PAGE, &before, &after);
    let passed = opened
        && dismissed
        && state.screen_state.last_action == "menu_context_dismiss"
        && state.screen_state.last_event == "menu_closed"
        && state.screen_state.state_label == "open=false"
        && !state.screen_state.selection.select_open
        && body_pixel_diff > 0;
    scenario(
        MENU_PAGE,
        "menu_context_dismiss",
        "context_menu",
        dismissed,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn menu_button_context_open_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(MENU_BUTTON_PAGE);
    let before = render_state(MENU_BUTTON_PAGE, &state);
    let target = dedicated_menu_button::trigger_rect(preview_detail::component_action_hit_rect(
        MENU_BUTTON_PAGE,
    ));
    let opened = apply_context_click(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(MENU_BUTTON_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(MENU_BUTTON_PAGE, &before, &after);
    let passed = opened
        && state.screen_state.last_action == "menu_button_context_open"
        && state.screen_state.last_event == "menu_button_opened"
        && state.screen_state.selection.select_open
        && body_pixel_diff > 0;
    scenario(
        MENU_BUTTON_PAGE,
        "menu_button_context_open",
        "context_menu",
        opened,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn context_menu_open_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CONTEXT_MENU_PAGE);
    let before = render_state(CONTEXT_MENU_PAGE, &state);
    let component = preview_detail::component_action_hit_rect(CONTEXT_MENU_PAGE);
    let opened = apply_context_click(
        &mut state,
        component.x + CLICK_OFFSET,
        component.y + CLICK_OFFSET,
    );
    let after = render_state(CONTEXT_MENU_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CONTEXT_MENU_PAGE, &before, &after);
    let passed = opened
        && state.screen_state.last_action == "context_menu_open"
        && state.screen_state.last_event == "context_menu_opened"
        && state.screen_state.state_label == "context_menu=open"
        && body_pixel_diff > 0;
    scenario(
        CONTEXT_MENU_PAGE,
        "context_menu_open",
        "context_menu",
        opened,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn tree_view_context_menu_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(TREE_VIEW_PAGE);
    let before = render_state(TREE_VIEW_PAGE, &state);
    let component = preview_detail::component_action_hit_rect(TREE_VIEW_PAGE);
    let opened = apply_context_click(
        &mut state,
        component.x + CLICK_OFFSET,
        component.y + CLICK_OFFSET,
    );
    let after = render_state(TREE_VIEW_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(TREE_VIEW_PAGE, &before, &after);
    let passed = opened
        && state.screen_state.last_action == "tree_context_menu"
        && state.screen_state.last_event == "tree_context_opened"
        && state.screen_state.state_label == "context_menu=open"
        && body_pixel_diff > 0;
    scenario(
        TREE_VIEW_PAGE,
        "tree_view_context_menu",
        "context_menu",
        opened,
        passed,
        body_pixel_diff,
        &state,
    )
}
