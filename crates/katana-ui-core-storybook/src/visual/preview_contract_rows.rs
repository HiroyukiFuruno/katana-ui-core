use crate::catalog::StoryExample;
use crate::catalog::StoryPresetLabels;
use katana_ui_core::render_model::{UiNode, UiNodeKind};

const CONTRACT_ROW_COUNT: usize = 8;
const STATUS_ROW_COUNT: usize = 8;
const ROW_VALUE_MAX_CHARS: usize = 84;
const CLIP_SUFFIX: &str = "...";

#[cfg(test)]
mod tests;

pub(super) fn contract_rows(
    node: &UiNode,
    example: &StoryExample,
) -> [(&'static str, String); CONTRACT_ROW_COUNT] {
    [
        ("option", row_value(option_summary(node))),
        ("action", row_value(action_summary(example))),
        ("event", row_value(event_summary(example))),
        ("state", row_value(state_summary(node))),
        ("preset", row_value(preset_summary(example.page))),
        (
            "preview",
            row_value(format!("page={} kind={:?}", example.page, node.kind())),
        ),
        ("settings", row_value(settings_summary(node))),
        (
            "test",
            row_value("unit + numeric render contract".to_string()),
        ),
    ]
}

pub(super) fn preset_summary(page: &str) -> String {
    StoryPresetLabels::for_page(page).join(" / ")
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
        ("render", "numeric contract".to_string()),
    ]
}

pub(super) fn rows_fit(examples: &[StoryExample]) -> bool {
    examples.iter().all(|example| {
        contract_rows(example.tree.root(), example)
            .iter()
            .chain(status_rows(example).iter())
            .all(|(_, value)| value.chars().count() <= ROW_VALUE_MAX_CHARS)
    })
}

fn option_summary(node: &UiNode) -> String {
    let props = node.props();
    let base = format!(
        "variant={:?} tone={:?} size={:?} font={}",
        props.variant, props.tone, props.size, props.font_role
    );
    match node.kind() {
        UiNodeKind::Text => format!(
            "{base} role={} color={} line={} baseline={} centered={}",
            props.text.role,
            props.text.color_token,
            props.text.line_height_px,
            props.text.baseline_offset_px,
            props.text.vertical_centered
        ),
        UiNodeKind::Icon | UiNodeKind::SvgButton => {
            format!(
                "{base} svg={} role={}",
                props.icon.svg_source, props.icon.role
            )
        }
        UiNodeKind::Input | UiNodeKind::SearchBox => format!(
            "{base} ime={} emoji={} submit={}",
            props.text_entry.ime_enabled,
            props.text_entry.emoji_enabled,
            props.text_entry.submit_on_enter
        ),
        UiNodeKind::LoadingDots | UiNodeKind::Spinner => format!(
            "{base} speed={} dots={} reduced={}",
            props.loading_indicator.speed_ms,
            props.loading_indicator.dot_count,
            props.loading_indicator.reduced_motion
        ),
        UiNodeKind::ColorSwatch => format!(
            "{base} selected={} palette={}",
            props.color_swatch.selected_color,
            props.color_swatch.palette.len()
        ),
        UiNodeKind::KeyCap => {
            format!(
                "{base} platform={} combo={}",
                props.shortcut.platform, props.shortcut.combo
            )
        }
        _ => base,
    }
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

fn state_summary(node: &UiNode) -> String {
    let interaction = &node.props().interaction;
    format!(
        "open={} selected={} index={} value={} hover={} focus={}",
        interaction.open,
        interaction.has_selection,
        interaction.selected_index,
        visible_value(interaction.value.as_str()),
        interaction.hovered,
        interaction.focused
    )
}

fn settings_summary(node: &UiNode) -> String {
    let props = node.props();
    format!(
        "disabled={} readonly={} invalid={} focusable={} loading={}",
        props.disabled, props.readonly, props.invalid, props.focusable, props.loading
    )
}

fn status(value: bool) -> String {
    let label = if value { "implemented" } else { "missing" };
    label.to_string()
}

fn row_value(value: String) -> String {
    if value.chars().count() <= ROW_VALUE_MAX_CHARS {
        return value;
    }
    let keep = ROW_VALUE_MAX_CHARS - CLIP_SUFFIX.len();
    let clipped: String = value.chars().take(keep).collect();
    format!("{clipped}{CLIP_SUFFIX}")
}

fn visible_value(value: &str) -> &str {
    if value.is_empty() {
        return "-";
    }
    value
}
