use crate::catalog::StoryExample;
use katana_ui_core::render_model::UiNode;

const CONTRACT_ROW_COUNT: usize = 8;
const STATUS_ROW_COUNT: usize = 8;

pub(super) fn contract_rows(
    node: &UiNode,
    example: &StoryExample,
) -> [(&'static str, String); CONTRACT_ROW_COUNT] {
    let props = node.props();
    [
        ("option", option_summary(node)),
        ("action", action_summary(example)),
        ("event", event_summary(example)),
        ("state", props.interaction.summary()),
        ("preset", "default / interactive / edge / theme".to_string()),
        (
            "preview",
            format!("page={} kind={:?}", example.page, node.kind()),
        ),
        ("settings", settings_summary(node)),
        ("test", "unit + visual regression required".to_string()),
    ]
}

pub(super) fn status_rows(example: &StoryExample) -> [(&'static str, String); STATUS_ROW_COUNT] {
    let contract = example.contract;
    [
        ("preview", status(contract.preview)),
        ("settings", status(contract.settings)),
        ("state", status(contract.state_summary)),
        ("event", status(contract.event_history)),
        ("action", status(contract.action_history)),
        ("preset", status(contract.preset_tabs)),
        ("requirement", status(contract.requirement_status)),
        ("visual", "required".to_string()),
    ]
}

fn option_summary(node: &UiNode) -> String {
    let props = node.props();
    format!(
        "variant={:?} tone={:?} size={:?} font={}",
        props.variant, props.tone, props.size, props.font_role
    )
}

fn action_summary(example: &StoryExample) -> String {
    example
        .callback_logs
        .first()
        .map(|it| it.action.clone())
        .unwrap_or_else(|| "none".to_string())
}

fn event_summary(example: &StoryExample) -> String {
    if example.callback_logs.is_empty() {
        return "passive render".to_string();
    }
    example
        .callback_logs
        .iter()
        .map(|it| it.target.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn settings_summary(node: &UiNode) -> String {
    let props = node.props();
    format!(
        "disabled={} readonly={} invalid={} focusable={}",
        props.disabled, props.readonly, props.invalid, props.focusable
    )
}

fn status(value: bool) -> String {
    let label = if value { "implemented" } else { "missing" };
    label.to_string()
}
