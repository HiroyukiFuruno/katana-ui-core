use super::sanitized_command_projection::{
    SanitizedCommandDropdownItem, SanitizedCommandGroup, SanitizedCommandItem,
    SanitizedCommandProjection, SanitizedCommandTarget,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeDisplayMode, CommandChromeDropdown,
    CommandChromeDropdownItem, CommandChromeDropdownTrigger, CommandChromeToolbarPresentation,
};
use katana_ui_core::molecule::toolbar::{
    SplitAction, SplitActionPart, ToolbarDensity, ToolbarGroup, ToolbarPriority, ToolbarStrategy,
};

const FNV1A_OFFSET_BASIS_BYTES: [u8; 8] = [0xcb, 0xf2, 0x9c, 0xe4, 0x84, 0x22, 0x23, 0x25];
const FNV1A_PRIME: u64 = 0x0000_0100_0000_01b3;

pub(super) fn command_chrome_toolbar_presentation(
    projection: &SanitizedCommandProjection,
) -> CommandChromeToolbarPresentation {
    let ordered_groups = ordered_visible_groups(projection);
    let mut toolbar_groups = Vec::new();
    let mut actions = Vec::new();

    for group in ordered_groups {
        let visible_items = ordered_visible_items(group);
        if visible_items.is_empty() {
            continue;
        }

        let group_id = group_id(group);
        toolbar_groups.push(ToolbarGroup::new(group_id.clone()).label(group.label()));
        for item in visible_items {
            actions.push(action_for_item(item, group, &group_id, actions.len()));
        }
    }

    CommandChromeToolbarPresentation {
        actions,
        groups: toolbar_groups,
        display_mode: CommandChromeDisplayMode::IconLeading,
        density: ToolbarDensity::Default,
        overflow_strategy: ToolbarStrategy::Menu,
    }
}

impl From<&SanitizedCommandProjection> for CommandChromeToolbarPresentation {
    fn from(projection: &SanitizedCommandProjection) -> Self {
        command_chrome_toolbar_presentation(projection)
    }
}

fn ordered_visible_groups(projection: &SanitizedCommandProjection) -> Vec<&SanitizedCommandGroup> {
    let mut groups = projection
        .groups()
        .iter()
        .enumerate()
        .filter(|(_, group)| group.visible())
        .collect::<Vec<_>>();
    groups.sort_by_key(|(index, group)| (group.order(), *index));
    groups.into_iter().map(|(_, group)| group).collect()
}

fn ordered_visible_items(group: &SanitizedCommandGroup) -> Vec<&SanitizedCommandItem> {
    let mut items = group
        .items()
        .iter()
        .enumerate()
        .filter(|(_, item)| item.visible())
        .collect::<Vec<_>>();
    items.sort_by_key(|(index, item)| (item.order(), *index));
    items.into_iter().map(|(_, item)| item).collect()
}

fn ordered_visible_dropdown_items(
    item: &SanitizedCommandItem,
) -> Vec<&SanitizedCommandDropdownItem> {
    let mut items = item
        .dropdown_items()
        .iter()
        .enumerate()
        .filter(|(_, item)| item.visible())
        .collect::<Vec<_>>();
    items.sort_by_key(|(index, item)| (item.order(), *index));
    items.into_iter().map(|(_, item)| item).collect()
}

fn action_for_item(
    item: &SanitizedCommandItem,
    group: &SanitizedCommandGroup,
    group_id: &str,
    sequence: usize,
) -> CommandChromeAction {
    let disabled = !group.enabled() || !item.enabled();
    let mut action =
        CommandChromeAction::new(target_id("kuc-command", item.target()), item.label())
            .group_id(group_id.to_owned())
            .priority(priority_for_sequence(sequence))
            .disabled(disabled);

    if let Some(value) = item.tooltip() {
        action = action.tooltip(value);
    }
    if let Some(value) = item.accessibility_label() {
        action = action.accessibility_label(value);
    }
    if let Some(value) = item.icon() {
        action = action.icon(value.clone());
    }
    if item.tooltip().is_none()
        && let Some(value) = group.tooltip()
    {
        action = action.tooltip(value);
    }
    if item.accessibility_label().is_none()
        && let Some(value) = group.accessibility_label()
    {
        action = action.accessibility_label(value);
    }
    if item.icon().is_none()
        && let Some(value) = group.icon()
    {
        action = action.icon(value.clone());
    }

    let dropdown_items = ordered_visible_dropdown_items(item);
    if dropdown_items.is_empty() {
        return action;
    }

    let split_part = SplitActionPart::new().disabled(disabled);
    let mut dropdown = CommandChromeDropdown::new(CommandChromeDropdownTrigger::SplitSecondary);
    for dropdown_item in dropdown_items {
        dropdown = dropdown.item(command_dropdown_item(dropdown_item));
    }

    action
        .split(SplitAction::new(split_part.clone(), split_part))
        .dropdown(dropdown)
}

fn command_dropdown_item(item: &SanitizedCommandDropdownItem) -> CommandChromeDropdownItem {
    let mut value =
        CommandChromeDropdownItem::new(target_id("kuc-dropdown", item.target()), item.label())
            .disabled(!item.enabled());
    if let Some(tooltip) = item.tooltip() {
        value = value.tooltip(tooltip);
    }
    if let Some(label) = item.accessibility_label() {
        value = value.accessibility_label(label);
    }
    if let Some(icon) = item.icon() {
        value = value.icon(icon.clone());
    }
    value
}

fn group_id(group: &SanitizedCommandGroup) -> String {
    format!(
        "kuc-group-{:08x}-{:016x}",
        group.order(),
        stable_hash_text(group.label())
    )
}

fn target_id(namespace: &str, target: &SanitizedCommandTarget) -> String {
    format!("{namespace}-{}", target.stable_fingerprint())
}

fn stable_hash_text(value: &str) -> u64 {
    let mut hash = u64::from_be_bytes(FNV1A_OFFSET_BASIS_BYTES);
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV1A_PRIME);
    }
    hash
}

fn priority_for_sequence(sequence: usize) -> ToolbarPriority {
    let sequence = i32::try_from(sequence).unwrap_or(i32::MAX);
    ToolbarPriority::new(i32::MAX.saturating_sub(sequence))
}

#[cfg(test)]
mod tests {
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
                            SanitizedCommandDropdownItem::new(
                                target("drop later"),
                                30,
                                "later option",
                            )
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
        let projection =
            SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "host").item(
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
}
