use katana_ui_core::molecule::{
    SettingsControl, SettingsControlOption, SettingsDirtyVisualization, SettingsField,
    SettingsList, SettingsSection, SettingsValue,
};

const DEFAULT_FONT_SIZE: i64 = 14;
const MIN_FONT_SIZE: i64 = 10;
const MAX_FONT_SIZE: i64 = 24;

pub(super) fn settings_list() -> SettingsList {
    SettingsList::new("Settings list")
        .dirty_visualization(SettingsDirtyVisualization::Marker)
        .section(editor_section())
        .section(appearance_section())
}

fn editor_section() -> SettingsSection {
    SettingsSection::new("editor", "Editor")
        .description("Editing preferences")
        .field(
            SettingsField::new(
                "editor.format-on-save",
                "Format on save",
                SettingsControl::Toggle { checked: true },
            )
            .description("Run formatter before writing the file")
            .reset_to_default(SettingsValue::Bool(false)),
        )
        .field(
            SettingsField::new(
                "editor.font-size",
                "Font size",
                SettingsControl::Number {
                    value: DEFAULT_FONT_SIZE,
                    min: MIN_FONT_SIZE,
                    max: MAX_FONT_SIZE,
                },
            )
            .reset_to_default(SettingsValue::Number(DEFAULT_FONT_SIZE)),
        )
}

fn appearance_section() -> SettingsSection {
    SettingsSection::new("appearance", "Appearance")
        .description("Visual preferences")
        .collapsible(true)
        .field(SettingsField::new(
            "appearance.theme",
            "Theme",
            SettingsControl::Select {
                options: vec![
                    SettingsControlOption::new("system", "System"),
                    SettingsControlOption::new("dark", "Dark"),
                ],
                selected: "system".to_string(),
            },
        ))
}
