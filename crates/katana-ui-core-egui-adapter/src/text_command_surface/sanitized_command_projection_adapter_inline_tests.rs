use crate::text_command_surface::sanitized_document_root::{
    SanitizedCommandDropdownItem, SanitizedCommandGroup, SanitizedCommandItem,
    SanitizedCommandProjection, SanitizedCommandTarget,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeDropdownTrigger, CommandChromeToolbarPresentation,
};
use katana_ui_core::render_model::UiIconProps;

#[test]
fn maps_ordering_capability_and_dropdown_without_host_semantics() {
    let projection = SanitizedCommandProjection::new([
        SanitizedCommandGroup::new(20, "second")
            .enabled_state(false)
            .item(SanitizedCommandItem::new(
                target("second"),
                1,
                "disabled by group",
            )),
        SanitizedCommandGroup::new(10, "first")
            .item(SanitizedCommandItem::new(target("later"), 20, "later").visible_state(false))
            .item(
                SanitizedCommandItem::new(target("main"), 10, "main")
                    .tooltip_text("main tooltip")
                    .accessibility_label_text("main access")
                    .with_icon(UiIconProps::new("<svg/>"))
                    .dropdown_item(
                        SanitizedCommandDropdownItem::new(target("drop hidden"), 5, "hidden")
                            .visible_state(false),
                    )
                    .dropdown_item(
                        SanitizedCommandDropdownItem::new(target("drop later"), 30, "later option")
                            .enabled_state(false),
                    )
                    .dropdown_item(SanitizedCommandDropdownItem::new(
                        target("drop first"),
                        10,
                        "first option",
                    )),
            ),
    ]);

    let toolbar = CommandChromeToolbarPresentation::from(&projection);

    assert_eq!(toolbar.groups.len(), 2);
    assert_eq!(
        toolbar
            .groups
            .iter()
            .map(|group| group.label_model().map(String::as_str))
            .collect::<Vec<_>>(),
        [Some("first"), Some("second")]
    );
    assert_eq!(
        toolbar
            .actions
            .iter()
            .map(|action| action.label_model())
            .collect::<Vec<_>>(),
        ["main", "disabled by group"]
    );
    let main = &toolbar.actions[0];
    assert_eq!(
        main.tooltip_model().map(String::as_str),
        Some("main tooltip")
    );
    assert_eq!(
        main.accessibility_label_model().map(String::as_str),
        Some("main access")
    );
    assert!(main.icon_model().is_some());
    assert!(!main.disabled_model());
    let dropdown = main.dropdown_model().expect("visible dropdown items map");
    assert_eq!(
        dropdown.trigger_model(),
        CommandChromeDropdownTrigger::SplitSecondary
    );
    assert_eq!(
        dropdown
            .items()
            .iter()
            .map(|item| (item.label_model(), item.disabled_model()))
            .collect::<Vec<_>>(),
        [("first option", false), ("later option", true)]
    );
    assert!(toolbar.actions[1].disabled_model());
}

#[test]
fn visible_items_are_stably_ordered_within_one_group() {
    let projection = SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "group")
        .item(SanitizedCommandItem::new(target("later"), 20, "later"))
        .item(SanitizedCommandItem::new(target("first"), 10, "first"))
        .item(SanitizedCommandItem::new(
            target("same-order"),
            10,
            "same order",
        ))]);

    let toolbar = CommandChromeToolbarPresentation::from(&projection);

    assert_eq!(
        toolbar
            .actions
            .iter()
            .map(|action| action.label_model())
            .collect::<Vec<_>>(),
        ["first", "same order", "later"]
    );
}

#[test]
fn opaque_target_mapping_is_private_and_does_not_reveal_payload() {
    let secret = "secret-host-payload";
    let projection = SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "group")
        .item(SanitizedCommandItem::new(target(secret), 1, "unknown"))]);

    let toolbar = CommandChromeToolbarPresentation::from(&projection);
    let action_id = toolbar.actions[0].id().as_str();

    assert!(action_id.starts_with("kuc-command-"));
    assert!(!action_id.contains(secret));

    let source = include_str!("sanitized_command_projection_adapter.rs");
    let api_source = source.split("#[cfg(test)]").next().unwrap_or(source);
    assert!(!api_source.contains("pub fn "));
    assert!(!api_source.contains("pub struct "));
    assert!(!api_source.contains("pub enum "));
    assert!(!api_source.contains("pub use "));
    assert!(!api_source.contains("target.opaque"));
    assert!(!api_source.contains("serialize"));
}

#[test]
fn unknown_host_command_remains_generic_without_semantic_switch() {
    let projection = SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "host").item(
        SanitizedCommandItem::new(target("opaque unknown"), 1, "host-defined"),
    )]);

    let toolbar = CommandChromeToolbarPresentation::from(&projection);

    assert_eq!(toolbar.actions.len(), 1);
    assert_eq!(toolbar.actions[0].label_model(), "host-defined");
    assert!(toolbar.actions[0].dropdown_model().is_none());

    let source = include_str!("sanitized_command_projection_adapter.rs");
    let source = source
        .split("#[cfg(test)]")
        .next()
        .unwrap_or(source)
        .to_ascii_lowercase();
    for term in ["katana_language", "katana::", "kle", "markdown"] {
        assert!(
            !source.contains(term),
            "adapter must not contain host semantic switch term: {term}"
        );
    }
}

fn target(value: &str) -> SanitizedCommandTarget {
    SanitizedCommandTarget::from_opaque_bytes(value.as_bytes())
}
