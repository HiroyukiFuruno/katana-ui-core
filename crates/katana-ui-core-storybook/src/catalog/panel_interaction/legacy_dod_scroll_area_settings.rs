use super::{BeforeAfterReport, SettingsMutationReport, StoryExample, TypedOptionMutationReport};

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "scroll-area") else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    scroll_area_options()
        .into_iter()
        .map(|option| report(option, &state_id))
        .collect()
}

fn report(option: ScrollAreaOption, state_id: &str) -> SettingsMutationReport {
    let marker = "catalog-scroll-area".to_string();
    SettingsMutationReport {
        page: "scroll-area".to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", option.name),
        event: "scroll_area_settings_changed".to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("scroll area option:{}={}", option.name, option.before),
            after: format!("scroll area option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn scroll_area_options() -> Vec<ScrollAreaOption> {
    vec![
        ScrollAreaOption {
            name: "scroll_area.axis",
            value_type: "ScrollAxis",
            before: "Both",
            after: "Vertical",
        },
        ScrollAreaOption {
            name: "scroll_area.offset",
            value_type: "Offset",
            before: "40,180",
            after: "0,720",
        },
        ScrollAreaOption {
            name: "scroll_area.viewport",
            value_type: "Extent",
            before: "320x220",
            after: "480x320",
        },
        ScrollAreaOption {
            name: "scroll_area.content",
            value_type: "Extent",
            before: "860x1400",
            after: "860x1800",
        },
        ScrollAreaOption {
            name: "scroll_area.scrollbar_visibility",
            value_type: "ScrollbarVisibility",
            before: "Always",
            after: "Auto",
        },
        ScrollAreaOption {
            name: "scroll_area.scrollbar_placement",
            value_type: "ScrollbarPlacement",
            before: "Reserved",
            after: "Overlay",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct ScrollAreaOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}
