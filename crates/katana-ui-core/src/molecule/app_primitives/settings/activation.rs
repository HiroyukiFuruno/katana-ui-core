use super::{SettingsControl, SettingsList, SettingsListAction, SettingsValue};

pub(super) fn field_action(list: &SettingsList, field_id: &str) -> Option<SettingsListAction> {
    let field = list
        .sections
        .iter()
        .flat_map(|section| section.fields.iter())
        .find(|field| field.id == field_id)?;
    let value = next_value(&field.control)?;
    Some(SettingsListAction::UpdateField {
        field_id: field.id.clone(),
        value,
    })
}

fn next_value(control: &SettingsControl) -> Option<SettingsValue> {
    match control {
        SettingsControl::Toggle { checked } => Some(SettingsValue::Bool(!checked)),
        SettingsControl::Select { options, selected }
        | SettingsControl::Radio { options, selected } => {
            next_selected_value(options, selected).map(SettingsValue::Text)
        }
        _ => None,
    }
}

fn next_selected_value(options: &[super::SettingsControlOption], selected: &str) -> Option<String> {
    let index = options
        .iter()
        .position(|option| option.value == selected)
        .unwrap_or_default();
    let next = options.get((index + 1) % options.len())?;
    Some(next.value.clone())
}

#[cfg(test)]
mod tests {
    use super::field_action;
    use crate::atom::Text;
    use crate::molecule::{
        SettingsControl, SettingsControlOption, SettingsField, SettingsList, SettingsListAction,
        SettingsSection, SettingsValue,
    };

    #[test]
    fn toggle_field_activation_returns_kuc_update_action() {
        let list = SettingsList::new("Settings").section(
            SettingsSection::new("display", "Display").field(SettingsField::new(
                "dark",
                "Dark",
                SettingsControl::Toggle { checked: true },
            )),
        );

        let action = field_action(&list, "dark");

        assert_eq!(
            Some(SettingsListAction::UpdateField {
                field_id: "dark".to_string(),
                value: SettingsValue::Bool(false),
            }),
            action
        );
    }

    #[test]
    fn select_field_activation_cycles_options() {
        let list = SettingsList::new("Settings").section(
            SettingsSection::new("display", "Display").field(SettingsField::new(
                "theme",
                "Theme",
                SettingsControl::Select {
                    options: vec![
                        SettingsControlOption::new("dark", "Dark"),
                        SettingsControlOption::new("light", "Light"),
                    ],
                    selected: "dark".to_string(),
                },
            )),
        );

        let action = field_action(&list, "theme");

        assert_eq!(
            Some(SettingsListAction::UpdateField {
                field_id: "theme".to_string(),
                value: SettingsValue::Text("light".to_string()),
            }),
            action
        );
    }

    #[test]
    fn readonly_field_activation_returns_none() {
        let list = SettingsList::new("Settings").section(
            SettingsSection::new("state", "State").field(SettingsField::new(
                "viewport",
                "Viewport",
                SettingsControl::Input {
                    value: "320x240".to_string(),
                },
            )),
        );

        assert_eq!(None, field_action(&list, "viewport"));
    }

    #[test]
    fn non_activatable_field_controls_return_none() {
        for control in non_activatable_controls() {
            let list = SettingsList::new("Settings").section(
                SettingsSection::new("state", "State")
                    .field(SettingsField::new("field", "Field", control)),
            );

            assert_eq!(None, field_action(&list, "field"));
        }
    }

    fn non_activatable_controls() -> Vec<SettingsControl> {
        vec![
            SettingsControl::Combo {
                options: vec![SettingsControlOption::new("dark", "Dark")],
                query: "da".to_string(),
                selected: None,
            },
            SettingsControl::Input {
                value: "dark".to_string(),
            },
            SettingsControl::TextArea {
                value: "multi\nline".to_string(),
            },
            SettingsControl::Number {
                value: 1,
                min: 0,
                max: 10,
            },
            SettingsControl::Chips {
                values: vec!["a".to_string(), "b".to_string()],
            },
            SettingsControl::ColorPicker {
                color: SettingsValue::Color {
                    red: 1,
                    green: 2,
                    blue: 3,
                    alpha: 255,
                },
            },
            SettingsControl::custom(Text::new("custom")),
        ]
    }
}
