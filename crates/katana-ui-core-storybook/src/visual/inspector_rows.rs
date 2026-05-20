use super::interaction_spec::StorybookInteractionSpec;
use super::render_context::ScenarioContext;
use crate::catalog::StoryExample;
use katana_ui_core::render_model::UiNode;

const INSPECTOR_ROW_GROUP_COUNT: usize = 4;
const ROW_MAX_CHARS: usize = 34;
const CLIP_SUFFIX: &str = "...";

pub(super) fn settings_rows(
    node: &UiNode,
    example: &StoryExample,
    scenario: ScenarioContext<'_>,
) -> Vec<String> {
    if example.page == "tree-view" {
        return vec![
            "line: solid 1px enabled".to_string(),
            "icons: folder/file visible".to_string(),
            "font/theme: body / dark".to_string(),
            "context menu: enabled".to_string(),
            "default open: true".to_string(),
            "trigger: icon+text chevron".to_string(),
        ];
    }
    if example.page == "context-menu" {
        let props = &node.props().context_menu;
        return vec![
            row_value(format!("anchor: {:?}", props.anchor)),
            row_value(format!("placement: {:?}", props.placement_used)),
            "items: section/action/submenu".to_string(),
            "log: opened/highlight/select".to_string(),
        ];
    }
    if example.page == "popover" {
        return vec![
            "placement: bottom-start".to_string(),
            "arrow: surface-raised".to_string(),
            "focus: first interactive".to_string(),
            "slot: heading/body/action".to_string(),
        ];
    }
    if example.page == "hover-card" {
        return vec![
            "delay: open100 close50".to_string(),
            "placement: pointer follow".to_string(),
            "focus: keep open".to_string(),
            "slot: heading/body/action".to_string(),
        ];
    }
    if is_button_page(example.page) {
        let spec = StorybookInteractionSpec::for_page(example.page);
        let variant = if scenario.screen_state.has_settings_override() {
            spec.after
        } else {
            "Filled"
        };
        let event = if scenario.screen_state.has_widget_action() {
            spec.event
        } else {
            "waiting for click"
        };
        return vec![
            row_value(format!("{}: {variant}", spec.option)),
            "click target: preview button".to_string(),
            "render: updates on click".to_string(),
            row_value(format!("event: {event}")),
        ];
    }
    let props = node.props();
    let spec = StorybookInteractionSpec::for_page(example.page);
    let variant = if scenario.screen_state.has_settings_override() {
        spec.after.to_string()
    } else {
        format!("{:?}", props.variant)
    };
    vec![
        row_value(format!("{}: {variant}", spec.option)),
        row_value(format!("tone: {:?}", props.tone)),
        row_value(format!("size: {:?}", props.size)),
        row_value(format!("font: {}", props.font_role)),
    ]
}

pub(super) fn settings_title(example: &StoryExample) -> &'static str {
    if example.page == "tree-view" {
        return "Tree settings";
    }
    if example.page == "context-menu" {
        return "ContextMenu settings";
    }
    "Settings"
}

pub(super) fn state_rows(
    node: &UiNode,
    scenario: ScenarioContext<'_>,
) -> [String; INSPECTOR_ROW_GROUP_COUNT] {
    let props = node.props();
    [
        row_value(format!("state: {}", props.state_id.as_str())),
        row_value(format!("open: {}", props.interaction.open)),
        row_value(format!("selected: {}", props.interaction.has_selection)),
        row_value(format!("screen: {}", scenario.screen_state.state_label)),
    ]
}

pub(super) fn history_rows(example: &StoryExample, scenario: ScenarioContext<'_>) -> Vec<String> {
    if scenario.screen_state.last_action != "none" {
        return vec![
            row_value(format!("action: {}", scenario.screen_state.last_action)),
            row_value(format!("event: {}", scenario.screen_state.last_event)),
            row_value(format!("set: {}", scenario.screen_state.last_setting)),
        ];
    }
    vec![
        "action: none".to_string(),
        "event: waiting for operation".to_string(),
        row_value(format!("target: {}", example.page)),
    ]
}

pub(super) fn quality_rows(scenario: ScenarioContext<'_>) -> [String; INSPECTOR_ROW_GROUP_COUNT] {
    let screen_control = if scenario.screen_state.last_action == "none" {
        "screen controls: ready"
    } else {
        "screen controls: active"
    };
    [
        "preview: rendered".to_string(),
        "settings: visible".to_string(),
        screen_control.to_string(),
        "visual gate: required".to_string(),
    ]
}

pub(super) fn rows_fit(examples: &[StoryExample]) -> bool {
    let scenario = ScenarioContext {
        selected_page: "button",
        preset_index: 0,
        tree_expansion: Default::default(),
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        screen_state: Default::default(),
    };
    examples.iter().all(|example| {
        let node = example.tree.root();
        settings_rows(node, example, scenario)
            .iter()
            .chain(state_rows(node, scenario).iter())
            .chain(history_rows(example, scenario).iter())
            .chain(quality_rows(scenario).iter())
            .all(|value| value.chars().count() <= ROW_MAX_CHARS)
    })
}

fn is_button_page(page: &str) -> bool {
    matches!(
        page,
        "button" | "text-button" | "svg-button" | "icon-text-button"
    )
}

fn row_value(value: String) -> String {
    if value.chars().count() <= ROW_MAX_CHARS {
        return value;
    }
    let keep = ROW_MAX_CHARS - CLIP_SUFFIX.len();
    let clipped: String = value.chars().take(keep).collect();
    format!("{clipped}{CLIP_SUFFIX}")
}
