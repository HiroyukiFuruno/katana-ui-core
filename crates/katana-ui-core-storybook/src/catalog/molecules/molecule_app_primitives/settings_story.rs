use katana_ui_core::molecule::{
    SettingsControl, SettingsControlOption, SettingsDirtyVisualization, SettingsField,
    SettingsList, SettingsSection, SettingsValue,
};
use katana_ui_core::{atom, molecule};

const DEFAULT_FONT_SIZE: i64 = 14;
const MIN_FONT_SIZE: i64 = 10;
const MAX_FONT_SIZE: i64 = 24;
const DEFAULT_TOKEN_BUDGET: i64 = 24_000;

pub(super) fn settings_list() -> SettingsList {
    SettingsList::new("Settings list")
        .dirty_visualization(SettingsDirtyVisualization::Marker)
        .section(app_section())
        .section(chat_section())
        .section(lint_section())
}

fn app_section() -> SettingsSection {
    SettingsSection::new("app", "App settings")
        .description("Window and editor preferences")
        .field(
            SettingsField::new(
                "app.format-on-save",
                "Format on save",
                SettingsControl::Toggle { checked: true },
            )
            .description("Run formatter before writing the file")
            .reset_to_default(SettingsValue::Bool(false)),
        )
        .field(
            SettingsField::new(
                "app.font-size",
                "Font size",
                SettingsControl::Number {
                    value: DEFAULT_FONT_SIZE,
                    min: MIN_FONT_SIZE,
                    max: MAX_FONT_SIZE,
                },
            )
            .reset_to_default(SettingsValue::Number(DEFAULT_FONT_SIZE)),
        )
        .field(SettingsField::new(
            "app.theme",
            "Theme",
            SettingsControl::Select {
                options: vec![
                    SettingsControlOption::new("system", "System"),
                    SettingsControlOption::new("dark", "Dark"),
                    SettingsControlOption::new("katana", "Katana"),
                ],
                selected: "system".to_string(),
            },
        ))
        .field(SettingsField::new(
            "app.workspace",
            "Workspace",
            SettingsControl::Input {
                value: "katana-ui-core".to_string(),
            },
        ))
}

fn chat_section() -> SettingsSection {
    SettingsSection::new("chat", "Chat settings")
        .description("Assistant composer preferences")
        .collapsible(true)
        .field(SettingsField::new(
            "chat.model",
            "Model",
            SettingsControl::Combo {
                options: vec![
                    SettingsControlOption::new("gpt-5", "GPT-5"),
                    SettingsControlOption::new("gpt-5-codex", "GPT-5 Codex"),
                ],
                query: "gpt".to_string(),
                selected: Some("gpt-5-codex".to_string()),
            },
        ))
        .field(SettingsField::new(
            "chat.system-prompt",
            "System prompt",
            SettingsControl::TextArea {
                value: "結論を先に述べる".to_string(),
            },
        ))
        .field(SettingsField::new(
            "chat.token-budget",
            "Token budget",
            SettingsControl::Number {
                value: DEFAULT_TOKEN_BUDGET,
                min: 4_000,
                max: 64_000,
            },
        ))
}

fn lint_section() -> SettingsSection {
    SettingsSection::new("lint", "Lint settings")
        .description("Validation and autofix preferences")
        .collapsible(true)
        .field(SettingsField::new(
            "lint.severity",
            "Severity",
            SettingsControl::Radio {
                options: vec![
                    SettingsControlOption::new("warning", "Warning"),
                    SettingsControlOption::new("error", "Error"),
                ],
                selected: "warning".to_string(),
            },
        ))
        .field(SettingsField::new(
            "lint.tags",
            "Tags",
            SettingsControl::Chips {
                values: vec!["style".to_string(), "contract".to_string()],
            },
        ))
        .field(SettingsField::new(
            "lint.accent",
            "Accent",
            SettingsControl::ColorPicker {
                color: SettingsValue::Color {
                    red: 96,
                    green: 165,
                    blue: 250,
                    alpha: 255,
                },
            },
        ))
        .field(SettingsField::new(
            "lint.custom-action",
            "Custom action",
            SettingsControl::custom(
                molecule::FormField::new("Run lint")
                    .child(atom::Button::new("Run now"))
                    .child(atom::Text::new("control kind: Custom")),
            ),
        ))
}
