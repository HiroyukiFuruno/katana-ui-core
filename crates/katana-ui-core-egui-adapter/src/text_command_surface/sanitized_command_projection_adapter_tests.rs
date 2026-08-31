use super::super::sanitized_command_projection::{
    SanitizedCommandDropdownItem, SanitizedCommandGroup, SanitizedCommandItem,
    SanitizedCommandProjection, SanitizedCommandTarget,
};
use super::command_chrome_toolbar_presentation;
use katana_ui_core::render_model::UiIconProps;

#[test]
fn presentation_inherits_group_metadata_and_preserves_dropdown_metadata() {
    let projection = SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "group")
        .tooltip_text("group tooltip")
        .accessibility_label_text("group access")
        .with_icon(UiIconProps::new("group-icon"))
        .item(
            SanitizedCommandItem::new(
                SanitizedCommandTarget::from_opaque_bytes(b"action"),
                1,
                "action",
            )
            .dropdown_item(
                SanitizedCommandDropdownItem::new(
                    SanitizedCommandTarget::from_opaque_bytes(b"dropdown"),
                    1,
                    "dropdown",
                )
                .tooltip_text("dropdown tooltip")
                .accessibility_label_text("dropdown access")
                .with_icon(UiIconProps::new("dropdown-icon")),
            ),
        )]);

    let toolbar = command_chrome_toolbar_presentation(&projection);
    let action = &toolbar.actions[0];
    let dropdown = action.dropdown_model().expect("dropdown is projected");
    let item = &dropdown.items()[0];

    assert_eq!(
        action.tooltip_model().map(String::as_str),
        Some("group tooltip")
    );
    assert_eq!(
        action.accessibility_label_model().map(String::as_str),
        Some("group access")
    );
    assert_eq!(
        action.icon_model().map(|icon| icon.svg_source.as_str()),
        Some("group-icon")
    );
    assert_eq!(
        item.tooltip_model().map(String::as_str),
        Some("dropdown tooltip")
    );
    assert_eq!(
        item.accessibility_label_model().map(String::as_str),
        Some("dropdown access")
    );
    assert_eq!(
        item.icon_model().map(|icon| icon.svg_source.as_str()),
        Some("dropdown-icon")
    );
}
