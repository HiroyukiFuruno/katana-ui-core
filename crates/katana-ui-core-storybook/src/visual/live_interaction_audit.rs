use super::dedicated_dod_form_binary_choice_live;
use super::layout_metrics::LayoutRect;
use super::preview_detail;
use super::render;
use super::window_interaction::{StorybookWindowState, apply_click, apply_hover_at};
use crate::requirements::StoryRequirements;
use crate::visual::interaction_spec::StorybookInteractionSpec;
#[cfg(test)]
use std::sync::OnceLock;
#[path = "live_interaction_audit_accordion.rs"]
mod live_interaction_audit_accordion;
#[path = "live_interaction_audit_breadcrumb.rs"]
mod live_interaction_audit_breadcrumb;
#[path = "live_interaction_audit_card.rs"]
mod live_interaction_audit_card;
#[path = "live_interaction_audit_checkbox.rs"]
mod live_interaction_audit_checkbox;
#[path = "live_interaction_audit_checkbox_disabled.rs"]
mod live_interaction_audit_checkbox_disabled;
#[path = "live_interaction_audit_checkbox_multi.rs"]
mod live_interaction_audit_checkbox_multi;
#[path = "live_interaction_audit_checkbox_snapshot.rs"]
mod live_interaction_audit_checkbox_snapshot;
#[path = "live_interaction_audit_checkbox_state_read.rs"]
mod live_interaction_audit_checkbox_state_read;
#[path = "live_interaction_audit_checkbox_visual.rs"]
mod live_interaction_audit_checkbox_visual;
#[path = "live_interaction_audit_chip.rs"]
mod live_interaction_audit_chip;
#[path = "live_interaction_audit_clickable.rs"]
mod live_interaction_audit_clickable;
#[path = "live_interaction_audit_code_diff.rs"]
mod live_interaction_audit_code_diff;
#[path = "live_interaction_audit_color_picker.rs"]
mod live_interaction_audit_color_picker;
#[path = "live_interaction_audit_command_palette.rs"]
mod live_interaction_audit_command_palette;
#[path = "live_interaction_audit_context.rs"]
mod live_interaction_audit_context;
#[path = "live_interaction_audit_form_field.rs"]
mod live_interaction_audit_form_field;
#[path = "live_interaction_audit_list.rs"]
mod live_interaction_audit_list;
#[path = "live_interaction_audit_list_navigation.rs"]
mod live_interaction_audit_list_navigation;
#[path = "live_interaction_audit_modal.rs"]
mod live_interaction_audit_modal;
#[path = "live_interaction_audit_popover.rs"]
mod live_interaction_audit_popover;
#[path = "live_interaction_audit_progress.rs"]
mod live_interaction_audit_progress;
#[path = "live_interaction_audit_radio.rs"]
mod live_interaction_audit_radio;
#[path = "live_interaction_audit_report.rs"]
mod live_interaction_audit_report;
#[path = "live_interaction_audit_scroll.rs"]
mod live_interaction_audit_scroll;
#[path = "live_interaction_audit_search.rs"]
mod live_interaction_audit_search;
#[path = "live_interaction_audit_selection.rs"]
mod live_interaction_audit_selection;
#[path = "live_interaction_audit_slide_control.rs"]
mod live_interaction_audit_slide_control;
#[path = "live_interaction_audit_split_pane.rs"]
mod live_interaction_audit_split_pane;
#[path = "live_interaction_audit_summary.rs"]
mod live_interaction_audit_summary;
#[path = "live_interaction_audit_tabs.rs"]
mod live_interaction_audit_tabs;
#[path = "live_interaction_audit_text_controls.rs"]
mod live_interaction_audit_text_controls;
#[path = "live_interaction_audit_text_entry.rs"]
mod live_interaction_audit_text_entry;
#[path = "live_interaction_audit_text_selection.rs"]
mod live_interaction_audit_text_selection;
#[path = "live_interaction_audit_toast_stack.rs"]
mod live_interaction_audit_toast_stack;
#[path = "live_interaction_audit_toggle.rs"]
mod live_interaction_audit_toggle;
#[path = "live_interaction_audit_toolbar.rs"]
mod live_interaction_audit_toolbar;
#[path = "live_interaction_audit_tooltip.rs"]
mod live_interaction_audit_tooltip;
#[cfg(test)]
pub(super) use live_interaction_audit_progress::{
    progress_indeterminate_segment_motion_scenario, progress_timed_cycle_scenario,
    progress_timed_tick_scenario,
};
pub use live_interaction_audit_report::{
    StorybookLiveInteractionAuditReport, StorybookLiveInteractionScenario,
};
#[cfg(test)]
pub(super) use live_interaction_audit_text_selection::scenarios as text_selection_scenarios;
const DARK_THEME: &str = "dark";
const CHECKBOX_PAGE: &str = "checkbox";
const RADIO_PAGE: &str = "radio";
const CLICK_OFFSET: usize = 4;
#[cfg(test)]
static LIVE_INTERACTION_AUDIT_REPORT: OnceLock<StorybookLiveInteractionAuditReport> =
    OnceLock::new();

pub(super) fn live_interaction_audit_report() -> StorybookLiveInteractionAuditReport {
    #[cfg(test)]
    {
        LIVE_INTERACTION_AUDIT_REPORT
            .get_or_init(live_interaction_audit_report_uncached)
            .clone()
    }
    #[cfg(not(test))]
    live_interaction_audit_report_uncached()
}

fn live_interaction_audit_report_uncached() -> StorybookLiveInteractionAuditReport {
    let scenarios = StoryRequirements::required_pages()
        .iter()
        .copied()
        .flat_map(live_interaction_scenarios)
        .collect();
    StorybookLiveInteractionAuditReport { scenarios }
}

fn live_interaction_scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    let pointer = match page {
        CHECKBOX_PAGE => checkbox_row_click_scenario(),
        RADIO_PAGE => radio_row_click_scenario(),
        _ => preview_click_scenario(page),
    };
    let mut scenarios = vec![pointer];
    let hover = hover_scenario(page);
    if hover.passed {
        scenarios.push(hover);
    }
    scenarios.extend(live_interaction_audit_card::scenarios(page));
    scenarios.extend(live_interaction_audit_breadcrumb::scenarios(page));
    scenarios.extend(live_interaction_audit_accordion::scenarios(page));
    scenarios.extend(live_interaction_audit_chip::scenarios(page));
    scenarios.extend(live_interaction_audit_clickable::scenarios(page));
    scenarios.extend(live_interaction_audit_code_diff::scenarios(page));
    scenarios.extend(live_interaction_audit_color_picker::scenarios(page));
    scenarios.extend(live_interaction_audit_command_palette::scenarios(page));
    scenarios.extend(live_interaction_audit_checkbox::scenarios(page));
    scenarios.extend(live_interaction_audit_checkbox_disabled::scenarios(page));
    scenarios.extend(live_interaction_audit_checkbox_multi::scenarios(page));
    scenarios.extend(live_interaction_audit_checkbox_snapshot::scenarios(page));
    scenarios.extend(live_interaction_audit_checkbox_state_read::scenarios(page));
    scenarios.extend(live_interaction_audit_checkbox_visual::scenarios(page));
    scenarios.extend(live_interaction_audit_radio::scenarios(page));
    scenarios.extend(live_interaction_audit_text_entry::scenarios(page));
    scenarios.extend(live_interaction_audit_text_selection::scenarios(page));
    scenarios.extend(live_interaction_audit_context::scenarios(page));
    scenarios.extend(live_interaction_audit_form_field::scenarios(page));
    scenarios.extend(live_interaction_audit_list::scenarios(page));
    scenarios.extend(live_interaction_audit_list_navigation::scenarios(page));
    scenarios.extend(live_interaction_audit_modal::scenarios(page));
    scenarios.extend(live_interaction_audit_popover::scenarios(page));
    scenarios.extend(live_interaction_audit_progress::scenarios(page));
    scenarios.extend(live_interaction_audit_scroll::scenarios(page));
    scenarios.extend(live_interaction_audit_search::scenarios(page));
    scenarios.extend(live_interaction_audit_selection::scenarios(page));
    scenarios.extend(live_interaction_audit_slide_control::scenarios(page));
    scenarios.extend(live_interaction_audit_split_pane::scenarios(page));
    scenarios.extend(live_interaction_audit_tabs::scenarios(page));
    scenarios.extend(live_interaction_audit_toggle::scenarios(page));
    scenarios.extend(live_interaction_audit_toast_stack::scenarios(page));
    scenarios.extend(live_interaction_audit_toolbar::scenarios(page));
    scenarios.extend(live_interaction_audit_tooltip::scenarios(page));
    scenarios
}

fn preview_click_scenario(page: &'static str) -> StorybookLiveInteractionScenario {
    let mut state = page_state(page);
    let before = render_state(page, &state);
    let target = preview_detail::component_action_hit_rect(page);
    let clicked = apply_click(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(page, &state);
    let body_pixel_diff = component_body_pixel_diff(page, &before, &after);
    let spec = StorybookInteractionSpec::for_page(page);
    let passed = clicked
        && state.screen_state.last_action == spec.action
        && state.screen_state.last_event == spec.event
        && state.screen_state.state_label == spec.state
        && body_pixel_diff > 0;
    scenario(
        page,
        "preview_click",
        "pointer",
        clicked,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn hover_scenario(page: &'static str) -> StorybookLiveInteractionScenario {
    let mut state = page_state(page);
    let before = render_state(page, &state);
    let target = preview_detail::component_action_hit_rect(page);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(page, &state);
    let body_pixel_diff = component_body_pixel_diff(page, &before, &after);
    let passed = hovered && body_pixel_diff > 0;
    scenario(
        page,
        "preview_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn checkbox_row_click_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    let before = render_state(CHECKBOX_PAGE, &state);
    let row = binary_choice_row(CHECKBOX_PAGE);
    let clicked = apply_click(&mut state, row.x + CLICK_OFFSET, row.y + CLICK_OFFSET);
    let after = render_state(CHECKBOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CHECKBOX_PAGE, &before, &after);
    let passed = clicked
        && state.screen_state.is_checkbox_checked()
        && state.screen_state.last_action == "checkbox_toggle"
        && state.screen_state.last_event == "checked_changed"
        && body_pixel_diff > 0;
    scenario(
        CHECKBOX_PAGE,
        "row_click",
        "pointer",
        clicked,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn radio_row_click_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(RADIO_PAGE);
    let before = render_state(RADIO_PAGE, &state);
    let row = binary_choice_row(RADIO_PAGE);
    let clicked = apply_click(&mut state, row.x + CLICK_OFFSET, row.y + CLICK_OFFSET);
    let after = render_state(RADIO_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(RADIO_PAGE, &before, &after);
    let passed = clicked
        && state.screen_state.is_radio_selected()
        && state.screen_state.last_action == "radio_select"
        && state.screen_state.last_event == "radio_selected"
        && body_pixel_diff > 0;
    scenario(
        RADIO_PAGE,
        "row_click",
        "pointer",
        clicked,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn scenario(
    page: &'static str,
    operation: &'static str,
    operation_kind: &'static str,
    clicked: bool,
    passed: bool,
    body_pixel_diff: usize,
    state: &StorybookWindowState,
) -> StorybookLiveInteractionScenario {
    StorybookLiveInteractionScenario {
        page,
        operation,
        operation_kind,
        clicked,
        passed,
        action: state.screen_state.last_action,
        event: state.screen_state.last_event,
        state: state.screen_state.state_label,
        checked: state.screen_state.is_checkbox_checked(),
        selected: state.screen_state.is_radio_selected(),
        body_pixel_diff,
        clipboard_text_len: state.clipboard_text.len(),
    }
}

fn binary_choice_row(page: &str) -> super::layout_metrics::LayoutRect {
    let origin = preview_detail::component_action_hit_rect(page);
    dedicated_dod_form_binary_choice_live::row_rect(0, origin.x, origin.y)
}

fn render_state(page: &'static str, state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn component_body_pixel_diff(page: &str, before: &super::Canvas, after: &super::Canvas) -> usize {
    rect_pixel_diff(
        preview_detail::component_action_hit_rect(page),
        before,
        after,
    )
}

fn rect_pixel_diff(rect: LayoutRect, before: &super::Canvas, after: &super::Canvas) -> usize {
    let mut count = 0;
    let max_y = (rect.y + rect.height)
        .min(before.height())
        .min(after.height());
    let max_x = (rect.x + rect.width).min(before.width()).min(after.width());
    for y in rect.y..max_y {
        for x in rect.x..max_x {
            let index = y * before.width() + x;
            if before.pixels()[index] != after.pixels()[index] {
                count += 1;
            }
        }
    }
    count
}

fn page_state(page: &'static str) -> StorybookWindowState {
    StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    }
}

#[cfg(test)]
mod tests {
    use super::live_interaction_audit_report_uncached;

    #[test]
    fn live_interaction_audit_report_has_no_failed_scenarios() {
        let report = live_interaction_audit_report_uncached();
        let failed: Vec<_> = report
            .scenarios
            .iter()
            .filter(|scenario| !scenario.passed)
            .map(|scenario| {
                format!(
                    "{}:{} kind={} diff={}",
                    scenario.page,
                    scenario.operation,
                    scenario.operation_kind,
                    scenario.body_pixel_diff
                )
            })
            .collect();

        assert_eq!(Vec::<String>::new(), failed);
    }
}
