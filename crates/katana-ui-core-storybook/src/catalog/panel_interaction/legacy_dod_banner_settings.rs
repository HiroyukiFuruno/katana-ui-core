use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "banner") else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    banner_options()
        .into_iter()
        .map(|option| banner_report(option, &state_id))
        .collect()
}

fn banner_report(option: BannerSettingOption, state_id: &str) -> SettingsMutationReport {
    let marker = "catalog-banner".to_string();
    SettingsMutationReport {
        page: "banner".to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", option.name),
        event: "banner_settings_changed".to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("banner option:{}={}", option.name, option.before),
            after: format!("banner option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn banner_options() -> Vec<BannerSettingOption> {
    vec![
        BannerSettingOption {
            name: "banner.severity",
            value_type: "BannerSeverity",
            before: "Warning",
            after: "Danger",
        },
        BannerSettingOption {
            name: "banner.density",
            value_type: "BannerDensity",
            before: "Compact",
            after: "Default",
        },
        BannerSettingOption {
            name: "banner.actions",
            value_type: "BannerActions",
            before: "2",
            after: "1",
        },
        BannerSettingOption {
            name: "banner.details",
            value_type: "BannerDetails",
            before: "Closed",
            after: "Open",
        },
        BannerSettingOption {
            name: "banner.dismissible",
            value_type: "bool",
            before: "true",
            after: "false",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct BannerSettingOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}
