use super::{
    SettingsControl, SettingsField, SettingsList, SettingsListAction, SettingsListHitTestInput,
    SettingsListEvent, SettingsListHitTestResult, SettingsListLayoutMetrics, SettingsSection,
    SettingsValue,
};
use crate::render_model::UiCursor;

const TEST_VIEWPORT_WIDTH: u32 = 320;

#[test]
fn hit_targets_expose_setting_field_row_as_action_contract()
-> Result<(), Box<dyn std::error::Error>> {
    let list = sample_settings();
    let target = list
        .hit_targets(TEST_VIEWPORT_WIDTH)
        .into_iter()
        .find(|target| {
            matches!(
                &target.action,
                Some(SettingsListAction::UpdateField { field_id, .. }) if field_id == "dark"
            )
        })
        .ok_or_else(|| std::io::Error::other("dark toggle target missing"))?;
    let center_x = target.rect.x + target.rect.width / 2;
    let center_y = target.rect.y + target.rect.height / 2;

    assert_eq!(0, target.rect.x);
    assert_eq!(TEST_VIEWPORT_WIDTH, target.rect.width);
    assert_eq!(UiCursor::Pointer, target.cursor);
    assert_eq!(
        Some(SettingsList::field_node_id("dark")),
        target.hover_node_id
    );
    assert_eq!(
        Some(SettingsListAction::HoverField {
            field_id: "dark".to_string(),
            hovered: true,
        }),
        target.hover_action
    );
    assert_eq!(
        SettingsListHitTestResult::Field {
            field_id: "dark".to_string()
        },
        list.hit_test(SettingsListHitTestInput {
            pointer_x: center_x,
            pointer_y: center_y,
            scroll_offset_y: 0,
        })
    );
    assert_eq!(
        target.action,
        list.action_for_hit(SettingsListHitTestInput {
            pointer_x: center_x,
            pointer_y: center_y,
            scroll_offset_y: 0,
        })
    );
    Ok(())
}

#[test]
fn hit_target_for_field_returns_named_row_contract() -> Result<(), Box<dyn std::error::Error>> {
    let list = sample_settings();
    let target = list
        .hit_target_for_field("dark", TEST_VIEWPORT_WIDTH)
        .ok_or_else(|| std::io::Error::other("dark toggle target missing"))?;
    let center_x = target.rect.x + target.rect.width / 2;
    let center_y = target.rect.y + target.rect.height / 2;

    assert_eq!(0, target.rect.x);
    assert_eq!(TEST_VIEWPORT_WIDTH, target.rect.width);
    assert_eq!(UiCursor::Pointer, target.cursor);
    assert_eq!(
        Some(SettingsList::field_node_id("dark")),
        target.hover_node_id
    );
    assert_eq!(
        target.action,
        list.action_for_hit(SettingsListHitTestInput {
            pointer_x: center_x,
            pointer_y: center_y,
            scroll_offset_y: 0,
        })
    );
    Ok(())
}

#[test]
fn hit_target_for_hit_returns_row_target_from_same_contract()
-> Result<(), Box<dyn std::error::Error>> {
    let list = sample_settings();
    let target = list
        .hit_target_for_hit(
            SettingsListHitTestInput {
                pointer_x: toggle_control_center_x(),
                pointer_y: dark_field_center_y(),
                scroll_offset_y: 0,
            },
            TEST_VIEWPORT_WIDTH,
        )
        .ok_or_else(|| std::io::Error::other("dark toggle target missing"))?;

    assert_eq!(0, target.rect.x);
    assert_eq!(TEST_VIEWPORT_WIDTH, target.rect.width);
    assert_eq!(UiCursor::Pointer, target.cursor);
    assert_eq!(
        Some(SettingsList::field_node_id("dark")),
        target.hover_node_id
    );
    assert_eq!(
        Some(SettingsListAction::UpdateField {
            field_id: "dark".to_string(),
            value: SettingsValue::Bool(false),
        }),
        target.action
    );
    Ok(())
}

#[test]
fn interaction_for_hit_returns_single_settings_contract() -> Result<(), Box<dyn std::error::Error>>
{
    let list = sample_settings();
    let interaction = list.interaction_for_hit(
        SettingsListHitTestInput {
            pointer_x: toggle_control_center_x(),
            pointer_y: dark_field_center_y(),
            scroll_offset_y: 0,
        },
        TEST_VIEWPORT_WIDTH,
    );
    let target = interaction
        .target
        .as_ref()
        .ok_or_else(|| std::io::Error::other("dark toggle target missing"))?;

    assert_eq!(
        SettingsListHitTestResult::Field {
            field_id: "dark".to_string()
        },
        interaction.result
    );
    assert_eq!(UiCursor::Pointer, interaction.cursor);
    assert_eq!(
        Some(SettingsList::field_node_id("dark")),
        interaction.hover_node_id
    );
    assert_eq!(
        Some(SettingsListAction::HoverField {
            field_id: "dark".to_string(),
            hovered: true,
        }),
        interaction.hover_action
    );
    assert_eq!(
        Some(SettingsListAction::UpdateField {
            field_id: "dark".to_string(),
            value: SettingsValue::Bool(false),
        }),
        interaction.action
    );
    assert_eq!(0, target.rect.x);
    assert_eq!(TEST_VIEWPORT_WIDTH, target.rect.width);
    Ok(())
}

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
fn interaction_for_hit_preserves_readonly_field_without_action() {
    let interaction = sample_settings().interaction_for_hit(
        SettingsListHitTestInput {
            pointer_x: text_entry_control_center_x(),
            pointer_y: theme_field_center_y(),
            scroll_offset_y: 0,
        },
        TEST_VIEWPORT_WIDTH,
    );

    assert_eq!(
        SettingsListHitTestResult::Field {
            field_id: "theme".to_string()
        },
        interaction.result
    );
    assert_eq!(UiCursor::Default, interaction.cursor);
    assert_eq!(None, interaction.hover_node_id);
    assert_eq!(None, interaction.hover_action);
    assert_eq!(None, interaction.action);
    assert_eq!(None, interaction.target);
}

#[test]
fn hit_target_for_hit_respects_scroll_offset_and_field_row() {
    let list = sample_settings();
    let target = list.hit_target_for_hit(
        SettingsListHitTestInput {
            pointer_x: toggle_control_center_x(),
            pointer_y: 4,
            scroll_offset_y: dark_field_center_y().saturating_sub(4),
        },
        TEST_VIEWPORT_WIDTH,
    );
    let label_target = list.hit_target_for_hit(
        SettingsListHitTestInput {
            pointer_x: metrics().field_control_x() - 1,
            pointer_y: dark_field_center_y(),
            scroll_offset_y: 0,
        },
        TEST_VIEWPORT_WIDTH,
    );

    assert!(target.is_some());
    assert!(label_target.is_some());
}

#[test]
fn hit_targets_expose_section_rect_inside_viewport_contract()
-> Result<(), Box<dyn std::error::Error>> {
    let list = sample_settings();
    let target = list
        .hit_targets(TEST_VIEWPORT_WIDTH)
        .into_iter()
        .find(|target| {
            matches!(
                &target.action,
                Some(SettingsListAction::ToggleSection { section_id }) if section_id == "display"
            )
        })
        .ok_or_else(|| std::io::Error::other("display section target missing"))?;

    assert_eq!(0, target.rect.x);
    assert_eq!(TEST_VIEWPORT_WIDTH, target.rect.width);
    assert_eq!(UiCursor::Pointer, target.cursor);
    assert_eq!(
        Some(SettingsListAction::ToggleSection {
            section_id: "display".to_string(),
        }),
        target.action
    );
    Ok(())
}

#[test]
fn hit_target_for_section_returns_named_section_contract() -> Result<(), Box<dyn std::error::Error>>
{
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
