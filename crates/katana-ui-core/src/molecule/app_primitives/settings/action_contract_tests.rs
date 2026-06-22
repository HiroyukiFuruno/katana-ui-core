use super::{
    SettingsControl, SettingsField, SettingsList, SettingsListAction, SettingsListHitTestInput,
    SettingsListLayoutMetrics, SettingsSection, SettingsValue,
};

#[test]
fn action_for_hit_returns_field_activation_action() {
    let action = sample_settings().action_for_hit(SettingsListHitTestInput {
        pointer_x: toggle_control_center_x(),
        pointer_y: dark_field_center_y(),
        scroll_offset_y: 0,
    });

    assert_eq!(
        Some(SettingsListAction::UpdateField {
            field_id: "dark".to_string(),
            value: SettingsValue::Bool(false),
        }),
        action
    );
}

#[test]
fn action_for_hit_returns_field_activation_from_label_inside_row() {
    let action = sample_settings().action_for_hit(SettingsListHitTestInput {
        pointer_x: metrics().field_control_x() - 1,
        pointer_y: dark_field_center_y(),
        scroll_offset_y: 0,
    });

    assert_eq!(
        Some(SettingsListAction::UpdateField {
            field_id: "dark".to_string(),
            value: SettingsValue::Bool(false),
        }),
        action
    );
}

#[test]
fn action_for_hit_returns_none_for_readonly_field_row() {
    let action = sample_settings().action_for_hit(SettingsListHitTestInput {
        pointer_x: text_entry_control_center_x(),
        pointer_y: theme_field_center_y(),
        scroll_offset_y: 0,
    });

    assert_eq!(None, action);
}

#[test]
fn action_for_hit_returns_section_toggle_action() {
    let action = sample_settings().action_for_hit(SettingsListHitTestInput {
        pointer_x: 0,
        pointer_y: display_section_center_y(),
        scroll_offset_y: 0,
    });

    assert_eq!(
        Some(SettingsListAction::ToggleSection {
            section_id: "display".to_string(),
        }),
        action
    );
}

#[test]
fn content_height_uses_settings_row_layout_contract() {
    let expected_height = metrics().title_height()
        + metrics().search_box_height()
        + metrics().section_height()
        + metrics().field_height()
        + metrics().field_height();

    assert_eq!(expected_height, sample_settings().content_height());
}

fn display_section_center_y() -> u32 {
    metrics().title_height() + metrics().search_box_height() + metrics().section_height() / 2
}

fn dark_field_center_y() -> u32 {
    metrics().title_height()
        + metrics().search_box_height()
        + metrics().section_height()
        + metrics().field_height() / 2
}

fn theme_field_center_y() -> u32 {
    dark_field_center_y() + metrics().field_height()
}

const fn toggle_control_center_x() -> u32 {
    SettingsListLayoutMetrics::DEFAULT.field_control_x()
        + SettingsListLayoutMetrics::DEFAULT.toggle_width() / 2
}

const fn text_entry_control_center_x() -> u32 {
    SettingsListLayoutMetrics::DEFAULT.field_control_x()
        + SettingsListLayoutMetrics::DEFAULT.text_entry_width() / 2
}

const fn metrics() -> SettingsListLayoutMetrics {
    SettingsListLayoutMetrics::DEFAULT
}

fn sample_settings() -> SettingsList {
    SettingsList::new("settings").section(
        SettingsSection::new("display", "Display")
            .field(SettingsField::new(
                "dark",
                "Dark",
                SettingsControl::Toggle { checked: true },
            ))
            .field(SettingsField::new(
                "theme",
                "Theme",
                SettingsControl::Input {
                    value: "dark".to_string(),
                },
            )),
    )
}
