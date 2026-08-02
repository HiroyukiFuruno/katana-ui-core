pub(super) use super::inspector_row_text::ROW_MAX_CHARS;
use super::inspector_row_text::row_value;
use super::interaction_spec::StorybookInteractionSpec;
use super::render_context::ScenarioContext;
use super::storybook_ui_option_contract;
use crate::catalog::StoryExample;

#[path = "inspector_rows_fit.rs"]
mod inspector_rows_fit;
use katana_ui_core::render_model::UiNode;

const INSPECTOR_ROW_GROUP_COUNT: usize = 4;
pub(super) fn settings_rows(
    node: &UiNode,
    example: &StoryExample,
    scenario: ScenarioContext<'_>,
) -> Vec<String> {
    if example.page == "tree-view" {
        return with_option_rows(
            example.page,
            vec![
                "line: solid 1px enabled".to_string(),
                "node markers: branch/leaf visible".to_string(),
                "font/theme: body / dark".to_string(),
                "context menu: enabled".to_string(),
                "default open: true".to_string(),
                "trigger: icon+text chevron".to_string(),
                "virtual: on range + total".to_string(),
                "rows: overscan/provider".to_string(),
            ],
        );
    }
    if example.page == "panel" {
        let panel = scenario.screen_state.panel;
        let active = panel.child(panel.active_panel);
        return with_option_rows(
            example.page,
            vec![
                row_value(format!("panel.active: {}", panel.active_panel.label())),
                row_value(format!("panel.vertical_scroll: y={}", active.scroll_y)),
                row_value(format!("panel.horizontal_scroll: x={}", active.scroll_x)),
                row_value(format!(
                    "panel.scrollbar_visibility: {}",
                    if active.scrollbar_visible {
                        "on"
                    } else {
                        "off"
                    }
                )),
            ],
        );
    }
    if is_virtualized_page(example.page) {
        return with_option_rows(
            example.page,
            vec![
                "virtual: on -> off".to_string(),
                "overscan: 2 -> 4".to_string(),
                "row height: fixed -> variable".to_string(),
                "range/total: logged".to_string(),
            ],
        );
    }
    if example.page == "context-menu" {
        let props = &node.props().context_menu;
        return with_option_rows(
            example.page,
            vec![
                row_value(format!("anchor: {:?}", props.anchor)),
                row_value(format!("placement: {:?}", props.placement_used)),
                "items: section/action/submenu".to_string(),
                "log: opened/highlight/select".to_string(),
            ],
        );
    }
    if example.page == "popover" {
        return with_option_rows(
            example.page,
            vec![
                "placement: bottom-start".to_string(),
                "arrow: surface-raised".to_string(),
                "focus: first interactive".to_string(),
                "slot: heading/body/action".to_string(),
            ],
        );
    }
    if example.page == "hover-card" {
        return with_option_rows(
            example.page,
            vec![
                "delay: open100 close50".to_string(),
                "placement: pointer follow".to_string(),
                "focus: keep open".to_string(),
                "slot: heading/body/action".to_string(),
            ],
        );
    }
    if example.page == "toolbar" {
        return with_option_rows(
            example.page,
            vec![
                "actions: add/remove".to_string(),
                "priority: visible/hidden".to_string(),
                "overflow: Menu".to_string(),
                "mode/density: icon/default".to_string(),
            ],
        );
    }
    if example.page == "badge" {
        return with_option_rows(
            example.page,
            vec![
                "role: passive status".to_string(),
                "actions: none".to_string(),
                "dismiss: use Chip".to_string(),
                "tone: status token".to_string(),
            ],
        );
    }
    if example.page == "banner" {
        return with_option_rows(
            example.page,
            vec![
                "severity: warning -> danger".to_string(),
                "density: compact -> default".to_string(),
                "actions: primary+secondary".to_string(),
                "details/dismiss: open/yes".to_string(),
            ],
        );
    }
    if example.page == "toast-stack-manager" {
        return with_option_rows(
            example.page,
            vec![
                "position: bottom -> top".to_string(),
                "max: 2 -> 4 / gap 10->16".to_string(),
                "dedup: id -> id+severity".to_string(),
                "duration/pause: 8000/off".to_string(),
            ],
        );
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
        return with_option_rows(
            example.page,
            vec![
                row_value(format!("{}: {variant}", spec.option)),
                "click target: preview button".to_string(),
                "render: updates on click".to_string(),
                row_value(format!("event: {event}")),
            ],
        );
    }
    let spec = StorybookInteractionSpec::for_page(example.page);
    storybook_ui_option_contract::settings_rows_for(example.page)
        .into_iter()
        .map(|row| {
            if row.starts_with(spec.option) && scenario.screen_state.has_settings_override() {
                let setting = if is_binary_choice_page(example.page) {
                    format!("option.{}", spec.option)
                } else {
                    spec.option.to_string()
                };
                return row_value(format!("{setting}: active -> {}", spec.after));
            }
            let display_row = if is_binary_choice_page(example.page) {
                format!("option.{row}")
            } else {
                row
            };
            row_value(display_row)
        })
        .collect()
}

fn with_option_rows(page: &str, rows: Vec<String>) -> Vec<String> {
    let mut output = rows;
    for row in storybook_ui_option_contract::settings_rows_for(page) {
        let setting = row.split_once(':').map_or(row.as_str(), |(it, _)| it);
        if output.iter().any(|it| it.contains(setting)) {
            continue;
        }
        output.push(row_value(row));
    }
    output
}

pub(super) fn settings_title(example: &StoryExample) -> &'static str {
    if example.page == "tree-view" {
        return "Tree settings";
    }
    if example.page == "context-menu" {
        return "ContextMenu settings";
    }
    if example.page == "panel" {
        return "Panel settings";
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
    inspector_rows_fit::rows_fit(examples)
}

pub(super) fn is_button_page(page: &str) -> bool {
    matches!(
        page,
        "button" | "text-button" | "svg-button" | "icon-text-button"
    )
}

fn is_virtualized_page(page: &str) -> bool {
    matches!(
        page,
        "list" | "selection-list" | "command-palette" | "diagnostics-list"
    )
}

fn is_binary_choice_page(page: &str) -> bool {
    matches!(page, "checkbox" | "radio" | "toggle" | "segmented-toggle")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StoryCatalog;
    use crate::test_assert::KucTestExpect;
    use crate::visual::panel_screen_state::PanelOptionControl;
    use crate::visual::screen_state::StorybookScreenState;

    #[test]
    fn inspector_rows_cover_panel_hidden_scrollbar_and_active_button_state() {
        let examples = StoryCatalog.examples();
        let panel = examples
            .iter()
            .find(|example| example.page == "panel")
            .kuc_expect("panel story must exist");
        let mut panel_state = StorybookScreenState::default();
        panel_state
            .panel
            .apply_option(PanelOptionControl::ScrollbarVisible(false));
        let panel_rows = settings_rows(
            panel.tree.root(),
            panel,
            ScenarioContext::for_test("panel", 0, &panel_state),
        );
        assert!(panel_rows.iter().any(|row| row.contains("off")));
        assert_eq!("Panel settings", settings_title(panel));

        let button = examples
            .iter()
            .find(|example| example.page == "button")
            .kuc_expect("button story must exist");
        let button_state = StorybookScreenState {
            action_count: 1,
            settings_revision: 1,
            ..StorybookScreenState::default()
        };
        let spec = StorybookInteractionSpec::for_page("button");
        let button_rows = settings_rows(
            button.tree.root(),
            button,
            ScenarioContext::for_test("button", 0, &button_state),
        );
        assert!(button_rows.iter().any(|row| row.contains(spec.after)));
        assert!(button_rows.iter().any(|row| row.contains(spec.event)));
    }
}
