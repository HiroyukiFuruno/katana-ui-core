use super::{BeforeAfterReport, SettingsMutationReport, StoryExample, TypedOptionMutationReport};

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "motion") else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    motion_options()
        .into_iter()
        .map(|option| report(option, &state_id))
        .collect()
}

fn report(option: MotionSettingOption, state_id: &str) -> SettingsMutationReport {
    let marker = "catalog-motion".to_string();
    SettingsMutationReport {
        page: "motion".to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", option.name),
        event: "motion_settings_changed".to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("motion option:{}={}", option.name, option.before),
            after: format!("motion option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn motion_options() -> Vec<MotionSettingOption> {
    vec![
        MotionSettingOption {
            name: "motion.primitive",
            value_type: "MotionPrimitive",
            before: "Slide",
            after: "Scale",
        },
        MotionSettingOption {
            name: "motion.duration",
            value_type: "MotionDurationToken",
            before: "Default",
            after: "Slow",
        },
        MotionSettingOption {
            name: "motion.easing",
            value_type: "MotionEasingToken",
            before: "Emphasized",
            after: "Decelerate",
        },
        MotionSettingOption {
            name: "motion.distance",
            value_type: "MotionDistanceToken",
            before: "Default",
            after: "Spacious",
        },
        MotionSettingOption {
            name: "motion.reduced_policy",
            value_type: "ReducedMotionPolicy",
            before: "Respect",
            after: "Ignore",
        },
        MotionSettingOption {
            name: "motion.disable_context",
            value_type: "MotionDisableContext",
            before: "Test",
            after: "Storybook",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct MotionSettingOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}
