use super::{
    SettingsControl, SettingsField, SettingsList, SettingsListAction, SettingsListEvent,
    SettingsSection, SettingsValue,
};
use crate::render_model::UiCursor;

const TEST_VIEWPORT_WIDTH: u32 = 320;

#[test]
fn hover_action_records_row_level_hover_event_without_value_change()
-> Result<(), Box<dyn std::error::Error>> {
    let mut list = sample_settings();
    let target = list
        .hit_target_for_field("dark", TEST_VIEWPORT_WIDTH)
        .ok_or_else(|| std::io::Error::other("dark toggle target missing"))?;
    let hover_action = target
        .hover_action
        .ok_or_else(|| std::io::Error::other("dark hover action missing"))?;

    let events = list.apply_settings_action(hover_action);

    assert_eq!(
        vec![SettingsListEvent::FieldHovered {
            field_id: "dark".to_string(),
            hovered: true,
        }],
        events
    );
    assert_eq!(
        SettingsValue::Bool(true),
        list.sections()[0].fields[0].control.value()
    );
    Ok(())
}

#[test]
fn hit_target_for_section_returns_named_section_hover_contract()
-> Result<(), Box<dyn std::error::Error>> {
    let list = sample_settings();
    let target = list
        .hit_target_for_section("display", TEST_VIEWPORT_WIDTH)
        .ok_or_else(|| std::io::Error::other("display section target missing"))?;

    assert_eq!(0, target.rect.x);
    assert_eq!(TEST_VIEWPORT_WIDTH, target.rect.width);
    assert_eq!(UiCursor::Pointer, target.cursor);
    assert_eq!(
        Some(SettingsList::section_node_id("display")),
        target.hover_node_id
    );
    assert_eq!(
        Some(SettingsListAction::HoverSection {
            section_id: "display".to_string(),
            hovered: true,
        }),
        target.hover_action
    );
    assert_eq!(
        Some(SettingsListAction::ToggleSection {
            section_id: "display".to_string(),
        }),
        target.action
    );
    Ok(())
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
