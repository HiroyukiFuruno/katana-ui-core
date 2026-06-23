use super::{
    SettingsControl, SettingsField, SettingsList, SettingsListAction, SettingsListHitTestResult,
    SettingsSection, SettingsValue,
};
use crate::render_model::{UiBorder, UiCursor, UiHostActionPlan, UiNode, UiNodeKind};

const TEST_VIEWPORT_WIDTH: u32 = 320;

#[test]
fn settings_field_id_reaches_form_field_state_id() -> Result<(), Box<dyn std::error::Error>> {
    let node: UiNode = SettingsList::new("settings")
        .section(
            SettingsSection::new("display", "Display").field(SettingsField::new(
                "dark",
                "Dark",
                SettingsControl::Toggle { checked: true },
            )),
        )
        .into();
    let field = find_kind(&node, UiNodeKind::FormField)
        .ok_or_else(|| std::io::Error::other("FormField missing"))?;

    assert_eq!("settings-field:dark", field.id().as_str());
    assert_eq!("settings-field:dark", field.props().state_id.as_str());
    assert_eq!(
        "settings-field:dark",
        SettingsList::field_node_id("dark").as_str()
    );
    Ok(())
}

#[test]
fn settings_toggle_field_keeps_interaction_on_rendered_control()
-> Result<(), Box<dyn std::error::Error>> {
    let node: UiNode = SettingsList::new("settings")
        .section(
            SettingsSection::new("display", "Display").field(SettingsField::new(
                "dark",
                "Dark",
                SettingsControl::Toggle { checked: true },
            )),
        )
        .into();
    let field = find_kind(&node, UiNodeKind::FormField)
        .ok_or_else(|| std::io::Error::other("FormField missing"))?;
    let toggle = find_kind(field, UiNodeKind::Toggle)
        .ok_or_else(|| std::io::Error::other("Toggle missing"))?;

    assert_eq!(UiCursor::Pointer, field.props().common.cursor);
    assert_eq!(
        UiBorder::solid(1, 4, "control.hover.border"),
        field.props().common.hover_border
    );
    assert_eq!(UiCursor::Pointer, toggle.props().common.cursor);
    assert!(toggle.props().common.hover_border.visible);
    assert_eq!(SettingsList::control_node_id("dark"), *toggle.id());
    Ok(())
}

#[test]
fn settings_field_row_dispatches_rendered_host_action() -> Result<(), Box<dyn std::error::Error>> {
    let list =
        SettingsList::new("settings").section(SettingsSection::new("display", "Display").field(
            SettingsField::new("dark", "Dark", SettingsControl::Toggle { checked: true }),
        ));
    let node: UiNode = list.clone().into();
    let plan = UiHostActionPlan::collect_from_root(&node)
        .into_iter()
        .find(|plan| plan.target == SettingsList::field_node_id("dark"))
        .ok_or_else(|| std::io::Error::other("field row host action missing"))?;

    assert_eq!(
        Some(SettingsListAction::UpdateField {
            field_id: "dark".to_string(),
            value: SettingsValue::Bool(false),
        }),
        list.action_from_host_plan(&plan)
    );
    Ok(())
}

#[test]
fn settings_section_header_exposes_hover_node_id() -> Result<(), Box<dyn std::error::Error>> {
    let node: UiNode = sample_settings().into();
    let header = find_panel(&node, "Display")
        .ok_or_else(|| std::io::Error::other("Display section header missing"))?;
    let target = sample_settings()
        .hit_targets(TEST_VIEWPORT_WIDTH)
        .into_iter()
        .find(|target| {
            matches!(
                &target.result,
                SettingsListHitTestResult::ToggleSection { section_id } if section_id == "display"
            )
        })
        .ok_or_else(|| std::io::Error::other("Display section hit target missing"))?;

    assert_eq!(SettingsList::section_node_id("display"), *header.id());
    assert_eq!(
        Some(SettingsList::section_node_id("display")),
        target.hover_node_id
    );
    Ok(())
}

fn find_kind(node: &UiNode, kind: UiNodeKind) -> Option<&UiNode> {
    if node.kind() == kind {
        return Some(node);
    }
    node.children()
        .iter()
        .find_map(|child| find_kind(child, kind))
}

fn find_panel<'a>(node: &'a UiNode, label: &str) -> Option<&'a UiNode> {
    if node.kind() == UiNodeKind::Panel && node.props().label == label {
        return Some(node);
    }
    node.children()
        .iter()
        .find_map(|child| find_panel(child, label))
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
