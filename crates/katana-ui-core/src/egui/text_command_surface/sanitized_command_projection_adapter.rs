use super::sanitized_command_projection::{
    SanitizedCommandDropdownItem, SanitizedCommandGroup, SanitizedCommandItem,
    SanitizedCommandProjection, SanitizedCommandTarget,
};
use crate::molecule::command_chrome::{
    CommandChromeAction, CommandChromeDisplayMode, CommandChromeDropdown,
    CommandChromeDropdownItem, CommandChromeDropdownTrigger, CommandChromeToolbarPresentation,
};
use crate::molecule::toolbar::{
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
#[path = "sanitized_command_projection_adapter_inline_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "sanitized_command_projection_adapter_tests.rs"]
mod coverage_tests;
