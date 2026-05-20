use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

#[derive(Clone, Copy)]
struct OverlaySetting {
    page: &'static str,
    option: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    overlay_settings()
        .into_iter()
        .filter_map(|setting| {
            examples
                .iter()
                .find(|it| it.page == setting.page)
                .map(|example| report(setting, example))
        })
        .collect()
}

fn overlay_settings() -> [OverlaySetting; 9] {
    [
        setting(
            "popover",
            "popover.placement",
            "Placement",
            "BottomStart",
            "TopStart",
        ),
        setting("popover", "popover.arrow", "bool", "true", "false"),
        setting(
            "popover",
            "popover.focus_management",
            "Focus",
            "FirstInteractive",
            "None",
        ),
        setting(
            "popover",
            "popover.slot",
            "Slot",
            "heading/body/action",
            "footer/action",
        ),
        setting(
            "hover-card",
            "hover_card.delay",
            "Delay",
            "open=100 close=50",
            "open=0 close=80",
        ),
        setting(
            "hover-card",
            "hover_card.placement",
            "Placement",
            "Pointer",
            "TopStart",
        ),
        setting("hover-card", "hover_card.arrow", "bool", "true", "false"),
        setting(
            "hover-card",
            "hover_card.focus",
            "Focus",
            "keep-open",
            "close-on-blur",
        ),
        setting(
            "hover-card",
            "hover_card.slot",
            "Slot",
            "heading/body/action",
            "footer/action",
        ),
    ]
}

const fn setting(
    page: &'static str,
    option: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
) -> OverlaySetting {
    OverlaySetting {
        page,
        option,
        value_type,
        before,
        after,
    }
}

fn report(setting: OverlaySetting, example: &StoryExample) -> SettingsMutationReport {
    let marker = format!("catalog-{}", setting.page);
    SettingsMutationReport {
        page: setting.page.to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", setting.option),
        event: format!("{}_settings_changed", setting.page.replace('-', "_")),
        target_state_id: example.tree.root().props().state_id.as_str().to_string(),
        option: TypedOptionMutationReport {
            name: setting.option.to_string(),
            value_type: setting.value_type.to_string(),
            before_value: setting.before.to_string(),
            after_value: setting.after.to_string(),
        },
        state: before_after(setting, "state"),
        preview: before_after(setting, &format!("{marker}:preview")),
    }
}

fn before_after(setting: OverlaySetting, prefix: &str) -> BeforeAfterReport {
    BeforeAfterReport {
        before: format!("{prefix} option:{}={}", setting.option, setting.before),
        after: format!("{prefix} option:{}={}", setting.option, setting.after),
    }
}
