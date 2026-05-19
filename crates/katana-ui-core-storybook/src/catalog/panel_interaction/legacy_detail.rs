use super::legacy_dod_options::{
    option_state_summary, option_value, props_with_option, resolved_after_value,
};
use super::legacy_dod_specs::{LegacyDodSpec, legacy_dod_specs};
use crate::catalog::{StoryExample, StoryPresetLabels};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryDetailContent {
    pub page: String,
    pub settings: String,
    pub state: String,
    pub event: String,
    pub action: String,
    pub preset: String,
    pub quality: String,
}

impl StoryDetailContent {
    #[must_use]
    pub fn from_example(example: &StoryExample) -> Self {
        let spec = spec_for(example.page);
        let marker = marker_for(spec, example.page);
        let option = spec.map_or(fallback_option(example.page), |it| it.option);
        let value_type = spec.map_or("StorybookOption", |it| it.value_type);
        let props = example.tree.root().props();
        let before = option_value(option, props);
        let configured_after = spec.map_or(fallback_after(example.page), |it| it.after);
        let resolved_after = resolved_after_value(option, value_type, configured_after, &before);
        let after_props = props_with_option(props, option, &resolved_after);
        let after = option_value(option, &after_props);
        let action = action_line(example, &marker);

        Self {
            page: example.page.to_string(),
            settings: format!("{marker} settings: {option} ({value_type}) {before} -> {after}"),
            state: state_line(example, &marker, option, &after_props),
            event: event_line(example, &marker),
            action,
            preset: preset_line(example.page, &marker),
            quality: quality_line(spec, &marker),
        }
    }
}

fn spec_for(page: &str) -> Option<&'static LegacyDodSpec> {
    legacy_dod_specs().find(|it| it.page == page)
}

fn fallback_option(page: &str) -> &'static str {
    if page == "context-menu" {
        return "context_menu.anchor";
    }
    "theme_id"
}

fn fallback_after(page: &str) -> &'static str {
    if page == "context-menu" {
        return "Pointer(192,128)";
    }
    "dark"
}

fn marker_for(spec: Option<&LegacyDodSpec>, page: &str) -> String {
    spec.map_or_else(
        || format!("catalog-{page}"),
        |it| format!("legacy-{}", it.marker),
    )
}

fn state_line(
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

fn event_line(example: &StoryExample, marker: &str) -> String {
    if let Some(log) = example.callback_logs.first() {
        return format!("{marker} event: {} -> {}", log.action, log.after);
    }
    format!("{marker} event: passive-ui")
}

fn action_line(example: &StoryExample, marker: &str) -> String {
    if let Some(log) = example.callback_logs.first() {
        return format!(
            "{marker} action: {} before={} after={}",
            log.action, log.before, log.after
        );
    }
    format!("{marker} action: none")
}

fn preset_line(page: &str, marker: &str) -> String {
    let presets = StoryPresetLabels::for_page(page);
    format!("{marker} preset: {}", presets.join(" / "))
}

fn quality_line(spec: Option<&LegacyDodSpec>, marker: &str) -> String {
    let option = spec.map_or("theme_id", |it| it.option);
    format!("{marker} quality: settings={option} state/event/action/preset markers fixed")
}
