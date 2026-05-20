use super::super::legacy_dod_options::{
    option_state_summary, option_value, props_with_option, resolved_after_value,
};
use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

const CONTEXT_MENU_PAGE: &str = "context-menu";
const CONTEXT_MENU_MARKER: &str = "catalog-context-menu";
const CONTEXT_MENU_EVENT: &str = "context_menu_settings_changed";

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == CONTEXT_MENU_PAGE) else {
        return Vec::new();
    };
    [
        ("context_menu.anchor", "ContextMenuAnchor", "Pointer(0,0)"),
        ("context_menu.placement", "ContextMenuPlacement", "AboveEnd"),
        ("context_menu.item_kind", "ContextMenuItemKind", "Toggle"),
    ]
    .into_iter()
    .map(|(option, value_type, after)| mutation_for_option(example, option, value_type, after))
    .collect()
}

fn mutation_for_option(
    example: &StoryExample,
    option: &str,
    value_type: &str,
    after: &str,
) -> SettingsMutationReport {
    let props = example.tree.root().props();
    let before = option_value(option, props);
    let resolved_after = resolved_after_value(option, value_type, after, &before);
    let after_props = props_with_option(props, option, &resolved_after);
    let actual_after = option_value(option, &after_props);
    SettingsMutationReport {
        page: example.page.to_string(),
        ui_marker: CONTEXT_MENU_MARKER.to_string(),
        action: format!("set_{option}"),
        event: CONTEXT_MENU_EVENT.to_string(),
        target_state_id: props.state_id.as_str().to_string(),
        option: TypedOptionMutationReport {
            name: option.to_string(),
            value_type: value_type.to_string(),
            before_value: before.clone(),
            after_value: actual_after.clone(),
        },
        state: BeforeAfterReport {
            before: option_state_summary(option, props),
            after: option_state_summary(option, &after_props),
        },
        preview: BeforeAfterReport {
            before: format!("{CONTEXT_MENU_MARKER}:preview:{option}={before}"),
            after: format!("{CONTEXT_MENU_MARKER}:preview:{option}={actual_after}"),
        },
    }
}
