use super::legacy_dod_options::{
    option_state_summary, option_value, props_with_option, resolved_after_value,
};
use super::legacy_dod_specs::{LegacyDodSpec, legacy_dod_specs};
use crate::catalog::StoryExample;

#[path = "legacy_detail_core.rs"]
mod legacy_detail_core;
#[path = "legacy_detail_settings_primary.rs"]
mod legacy_detail_settings_primary;
#[path = "legacy_detail_settings_secondary.rs"]
mod legacy_detail_settings_secondary;
#[path = "legacy_detail_types.rs"]
mod legacy_detail_types;

pub use legacy_detail_types::StoryDetailContent;

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

        Self {
            page: example.page.to_string(),
            settings: legacy_detail_settings_primary::settings_line(
                example, &marker, option, value_type, &before, &after,
            ),
            state: legacy_detail_core::state_line(example, &marker, option, &after_props),
            event: legacy_detail_core::event_line(example, &marker),
            action: legacy_detail_core::action_line(example, &marker),
            preset: legacy_detail_core::preset_line(example.page, &marker),
            quality: legacy_detail_core::quality_line(spec, example.page, &marker),
        }
    }
}

fn spec_for(page: &str) -> Option<&'static LegacyDodSpec> {
    legacy_dod_specs().find(|it| it.page == page)
}

fn fallback_option(page: &str) -> &'static str {
    match page {
        "context-menu" => "context_menu.anchor",
        _ => "theme_id",
    }
}

fn fallback_after(page: &str) -> &'static str {
    match page {
        "context-menu" => "Pointer(192,128)",
        _ => "dark",
    }
}

fn marker_for(spec: Option<&LegacyDodSpec>, page: &str) -> String {
    spec.map_or_else(
        || format!("catalog-{page}"),
        |it| format!("legacy-{}", it.marker),
    )
}
