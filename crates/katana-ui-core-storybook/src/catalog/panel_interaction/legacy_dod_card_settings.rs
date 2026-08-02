use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

const CARD_SETTING_OPTION_COUNT: usize = 4;

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    card_options()
        .into_iter()
        .filter_map(|option| card_report(examples, option))
        .collect()
}

fn card_report(
    examples: &[StoryExample],
    option: CardSettingOption,
) -> Option<SettingsMutationReport> {
    let example = examples.iter().find(|it| it.page == "card")?;
    let marker = "catalog-card";
    Some(SettingsMutationReport {
        page: "card".to_string(),
        ui_marker: marker.to_string(),
        action: format!("set_{}", option.name),
        event: "card_settings_changed".to_string(),
        target_state_id: example.tree.root().props().state_id.as_str().to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: option.state(option.before),
            after: option.state(option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    })
}

fn card_options() -> [CardSettingOption; CARD_SETTING_OPTION_COUNT] {
    [
        CardSettingOption {
            name: "card.label",
            value_type: "String",
            before: "Card",
            after: "Project summary",
        },
        CardSettingOption {
            name: "card.header",
            value_type: "Option<UiNode>",
            before: "visible",
            after: "custom",
        },
        CardSettingOption {
            name: "card.footer",
            value_type: "Option<UiNode>",
            before: "hidden",
            after: "visible",
        },
        CardSettingOption {
            name: "card.padding",
            value_type: "UiSize",
            before: "Medium",
            after: "Large",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct CardSettingOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}

impl CardSettingOption {
    fn state(self, value: &str) -> String {
        format!("card option:{}={}", self.name, value)
    }
}
