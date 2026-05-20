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
    if example.page == "toolbar" {
        return vec![
            "actions: add/remove".to_string(),
            "priority: visible/hidden".to_string(),
            "overflow: Menu".to_string(),
            "mode/density: icon/default".to_string(),
        ];
    }
    if example.page == "text-area" {
        let props = &node.props().text_area;
        return vec![
            row_value(format!(
                "submit/newline: {:?}/{:?}",
                props.submit_key, props.newline_key
            )),
            row_value(format!(
                "tab/wrap: {:?}/{:?}",
                props.tab_behavior, props.wrap_policy
            )),
            row_value(format!(
                "rows: {}..{} auto={}",
                props.min_rows, props.max_rows, props.auto_grow
            )),
            row_value(format!(
                "ime/scroll: {} / {}",
                props.ime_enabled, props.internal_scroll
            )),
        ];
    }
    if example.page == "badge" {
        return vec![
            "role: passive status".to_string(),
            "actions: none".to_string(),
            "dismiss: use Chip".to_string(),
            "tone: status token".to_string(),
        ];
    }
    if example.page == "banner" {
        return vec![
            "severity: warning -> danger".to_string(),
            "density: compact -> default".to_string(),
            "actions: primary+secondary".to_string(),
            "details/dismiss: open/yes".to_string(),
        ];
    }
    if example.page == "toast-stack-manager" {
        return vec![
            "position: bottom -> top".to_string(),
            "max: 2 -> 4 / gap 10->16".to_string(),
            "dedup: id -> id+severity".to_string(),
            "duration/pause: 8000/off".to_string(),
        ];
    }
    if example.page == "status-bar" {
        return vec![
            "mode: single -> multi".to_string(),
            "segments: 1 -> 4".to_string(),
            "density: default -> compact".to_string(),
            "popover/progress: enabled".to_string(),
        ];
    }
    if example.page == "shortcut-combo" {
        return vec![
            "platform: auto -> macOS".to_string(),
            "separator: plus -> none".to_string(),
            "size: medium -> large".to_string(),
            "tone: neutral -> accent".to_string(),
        ];
    }
    if example.page == "chip" {
        return vec![
            "variant: outline -> filled".to_string(),
            "tone: accent -> danger".to_string(),
            "size: medium -> large".to_string(),
            "dismiss: backspace/delete".to_string(),
        ];
    }
    if example.page == "attachment-chip" {
        return vec![
            "kind: file/image/url".to_string(),
            "status: uploading -> error".to_string(),
            "progress: 42 -> 100".to_string(),
            "retry: child button".to_string(),
        ];
    }
    if example.page == "chip-group" {
        return vec![
            "overflow: menu -> scroll".to_string(),
            "wrap: false -> true".to_string(),
            "reorder: on/off".to_string(),
            "menu: hidden chips".to_string(),
        ];
    }
    if example.page == "diagnostics-list" {
        return vec![
            "group_by: severity".to_string(),
            "sort_by: severity".to_string(),
            "filter: error+warning".to_string(),
            "bulk/fix: preview on".to_string(),
        ];
    }
    if example.page == "empty-state" {
        return vec![
            "tone: accent -> danger".to_string(),
            "size: default -> large".to_string(),
            "align: center -> leading".to_string(),
            "actions: primary+secondary".to_string(),
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
    if scenario.selected_page == "text-area" {
        return [
            row_value(format!("state: {}", props.state_id.as_str())),
            row_value(format!("value len: {}", props.interaction.value.len())),
            row_value(format!("caret: {}", props.interaction.cursor)),
            row_value(format!("rows: {}", props.text_area.measured_rows)),
        ];
    }
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
        "numeric gate: required".to_string(),
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
