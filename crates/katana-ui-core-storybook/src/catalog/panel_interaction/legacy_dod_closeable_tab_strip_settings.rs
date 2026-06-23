use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

const PAGE: &str = "closeable-tab-strip";
const MARKER: &str = "catalog-closeable-tab-strip";
const CLOSEABLE_TAB_STRIP_OPTION_COUNT: usize = 5;

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == PAGE) else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    closeable_tab_strip_options()
        .into_iter()
        .map(|option| closeable_tab_strip_report(option, &state_id))
        .collect()
}

fn closeable_tab_strip_report(
    option: CloseableTabStripOption,
    state_id: &str,
) -> SettingsMutationReport {
    SettingsMutationReport {
        page: PAGE.to_string(),
        ui_marker: MARKER.to_string(),
        action: option.action.to_string(),
        event: option.event.to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("tabs option:{}={}", option.name, option.before),
            after: format!("tabs option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{MARKER}:preview:{}={}", option.name, option.before),
            after: format!("{MARKER}:preview:{}={}", option.name, option.after),
        },
    }
}

fn closeable_tab_strip_options() -> [CloseableTabStripOption; CLOSEABLE_TAB_STRIP_OPTION_COUNT] {
    [
        CloseableTabStripOption {
            name: "tab.count",
            value_type: "usize",
            before: "6",
            after: "7",
            action: "add_tab",
            event: "closeable_tab_strip_tab_added",
        },
        CloseableTabStripOption {
            name: "tab.deleted",
            value_type: "bool",
            before: "false",
            after: "true",
            action: "delete_tab",
            event: "closeable_tab_strip_tab_deleted",
        },
        CloseableTabStripOption {
            name: "tab.pinned",
            value_type: "bool",
            before: "false",
            after: "true",
            action: "pin_tab",
            event: "closeable_tab_strip_pin_changed",
        },
        CloseableTabStripOption {
            name: "tab.dirty",
            value_type: "bool",
            before: "false",
            after: "true",
            action: "dirty_toggle",
            event: "closeable_tab_strip_dirty_changed",
        },
        CloseableTabStripOption {
            name: "tab.group",
            value_type: "TabGroup",
            before: "docs",
            after: "preview",
            action: "group_toggle",
            event: "closeable_tab_strip_group_changed",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct CloseableTabStripOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
    action: &'static str,
    event: &'static str,
}
