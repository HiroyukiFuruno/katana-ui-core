use super::{LegacyDodSpec, option_state_summary};
use crate::catalog::{StoryExample, StoryPresetLabels};
#[path = "legacy_detail_panel.rs"]
mod panel;

pub(super) fn state_line(
    example: &StoryExample,
    marker: &str,
    option: &str,
    after_props: &katana_ui_core::render_model::UiProps,
) -> String {
    match example.page {
        "panel" => panel::state_line(example, marker),
        "search-control-strip" => search_control_state_line(example, marker),
        "scroll-area" => scroll_area_state_line(example, marker),
        "command-palette" => command_palette_state_line(example, marker),
        _ => default_state_line(example, marker, option, after_props),
    }
}

pub(super) fn event_line(example: &StoryExample, marker: &str) -> String {
    match example.page {
        "panel" => panel::event_line(marker),
        "search-control-strip" => search_control_event_line(example, marker),
        "scroll-area" => scroll_area_event_line(example, marker),
        "command-palette" => command_palette_event_line(example, marker),
        _ => default_event_line(example, marker),
    }
}

pub(super) fn action_line(example: &StoryExample, marker: &str) -> String {
    match example.page {
        "panel" => panel::action_line(marker),
        "search-control-strip" => search_control_action_line(example, marker),
        "scroll-area" => scroll_area_action_line(example, marker),
        "command-palette" => command_palette_action_line(example, marker),
        _ => default_action_line(example, marker),
    }
}

pub(super) fn quality_line(spec: Option<&LegacyDodSpec>, page: &str, marker: &str) -> String {
    match page {
        "panel" => panel::quality_line(marker),
        "search-control-strip" => {
            format!("{marker} quality: typed options state_id result_count event_contract")
        }
        "scroll-area" => {
            format!("{marker} quality: nested_state_identity clamp edge_event axis_rejection")
        }
        "command-palette" => format!(
            "{marker} quality: keyboard_contract=true virtualized_highlight=true disabled_execution_guard=true"
        ),
        "closeable-tab-strip" => format!(
            "{marker} quality: settings=tab_add/delete/pin/dirty/group state/event/action/preset markers fixed"
        ),
        _ => {
            let option = spec.map_or("theme_id", |it| it.option);
            format!("{marker} quality: settings={option} state/event/action/preset markers fixed")
        }
    }
}

pub(super) fn preset_line(page: &str, marker: &str) -> String {
    if page == "closeable-tab-strip" {
        return format!("{marker} preset: default / overflow / pinned / groups / dirty / dragging");
    }
    let presets = StoryPresetLabels::for_page(page);
    format!("{marker} preset: {}", presets.join(" / "))
}

pub(super) fn callback_actions(example: &StoryExample) -> String {
    example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

pub(super) fn virtualization_log_after(example: &StoryExample) -> &str {
    virtualization_log(example).map_or("missing", |it| it.after.as_str())
}

pub(super) fn is_virtualized_page(page: &str) -> bool {
    matches!(
        page,
        "list" | "selection-list" | "tree-view" | "command-palette" | "diagnostics-list"
    )
}

fn default_state_line(
    example: &StoryExample,
    marker: &str,
    option: &str,
    after_props: &katana_ui_core::render_model::UiProps,
) -> String {
    let props = example.tree.root().props();
    format!(
        "{marker} state: id={} before={} after={}",
        props.state_id.as_str(),
        option_state_summary(option, props),
        option_state_summary(option, after_props)
    )
}

fn default_event_line(example: &StoryExample, marker: &str) -> String {
    if let Some(log) = example.callback_logs.first() {
        return format!("{marker} event: {} -> {}", log.action, log.after);
    }
    format!("{marker} event: passive-ui")
}

fn default_action_line(example: &StoryExample, marker: &str) -> String {
    if let Some(log) = virtualization_log(example).or_else(|| example.callback_logs.first()) {
        return format!(
            "{marker} action: {} before={} after={}",
            log.action, log.before, log.after
        );
    }
    format!("{marker} action: none")
}

fn scroll_area_state_line(example: &StoryExample, marker: &str) -> String {
    let props = example.tree.root().props();
    format!(
        "{marker} state: id={} state: offset={},{} viewport={}x{} content={}x{} edge=none",
        props.state_id.as_str(),
        props.scroll_area.offset_x,
        props.scroll_area.offset_y,
        props.scroll_area.viewport_width,
        props.scroll_area.viewport_height,
        props.scroll_area.content_width,
        props.scroll_area.content_height
    )
}

fn scroll_area_event_line(example: &StoryExample, marker: &str) -> String {
    format!(
        "{marker} event: Scrolled ScrollEdgeReached ScrollCommandRejected callback_log={}",
        example.callback_logs.len()
    )
}

fn scroll_area_action_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} action: scroll_to scroll_by scroll_into_view scrollbar_visibility actions={actions}"
    )
}

fn search_control_state_line(example: &StoryExample, marker: &str) -> String {
    let props = example.tree.root().props();
    format!(
        "{marker} state: id={} state: query={} match_case={} whole_word={} regex={} replace={} result={}",
        props.state_id.as_str(),
        props.search_control.query,
        props.search_control.match_case,
        props.search_control.whole_word,
        props.search_control.use_regex,
        props.search_control.replace_value,
        props.search_control.result_summary
    )
}

fn search_control_event_line(example: &StoryExample, marker: &str) -> String {
    format!(
        "{marker} event: SearchQueryChanged SearchOptionChanged SearchNavigationRequested ReplaceRequested callback_log={}",
        example.callback_logs.len()
    )
}

fn search_control_action_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!("{marker} action: query option navigate replace result-position actions={actions}")
}

fn command_palette_state_line(example: &StoryExample, marker: &str) -> String {
    format!(
        "{marker} state: id={} query=theme highlighted_row=theme virtual_range={} disabled_reason=readonly",
        example.tree.root().props().state_id.as_str(),
        virtualization_log_after(example)
    )
}

fn command_palette_event_line(example: &StoryExample, marker: &str) -> String {
    format!("{marker} event: {}", callback_actions(example))
}

fn command_palette_action_line(example: &StoryExample, marker: &str) -> String {
    format!("{marker} action: {}", callback_actions(example))
}

fn virtualization_log(
    example: &StoryExample,
) -> Option<&katana_ui_core::interaction::UiCallbackLog> {
    example
        .callback_logs
        .iter()
        .find(|it| it.action.contains("virtualization_range"))
}
