use super::{BeforeAfterReport, SettingsMutationReport, StoryExample, TypedOptionMutationReport};

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "command-palette") else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    options()
        .into_iter()
        .map(|option| report(option, &state_id))
        .collect()
}

fn report(option: CommandPaletteOption, state_id: &str) -> SettingsMutationReport {
    let marker = "catalog-command-palette".to_string();
    SettingsMutationReport {
        page: "command-palette".to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", option.name),
        event: "command_palette_settings_changed".to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("command palette option:{}={}", option.name, option.before),
            after: format!("command palette option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn options() -> Vec<CommandPaletteOption> {
    vec![
        CommandPaletteOption {
            name: "command_palette.query",
            value_type: "String",
            before: "open",
            after: "theme",
        },
        CommandPaletteOption {
            name: "command_palette.highlight",
            value_type: "usize",
            before: "0",
            after: "2",
        },
        CommandPaletteOption {
            name: "command_palette.row_count",
            value_type: "usize",
            before: "5",
            after: "50",
        },
        CommandPaletteOption {
            name: "command_palette.provider_group",
            value_type: "ProviderGroupSet",
            before: "workspace",
            after: "workspace/editor/app",
        },
        CommandPaletteOption {
            name: "command_palette.shortcut_display",
            value_type: "bool",
            before: "true",
            after: "false",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct CommandPaletteOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}
