use crate::render_model::{UiContextMenuItem, UiContextMenuItemKind, UiContextMenuProps};

pub(super) fn command_for_path(props: &UiContextMenuProps, path: &[usize]) -> String {
    item_for_path(&props.items, path).map_or_else(String::new, |item| {
        if selectable(item) {
            item.id.clone()
        } else {
            String::new()
        }
    })
}

pub(super) fn path_for_command(props: &UiContextMenuProps, command: &str) -> Option<Vec<usize>> {
    path_for_command_in_items(&props.items, command, &[])
}

pub(super) fn first_enabled_child_path(props: &UiContextMenuProps, path: &[usize]) -> Vec<usize> {
    let Some(item) = item_for_path(&props.items, path) else {
        return path.to_vec();
    };
    item.children
        .iter()
        .enumerate()
        .find_map(|(index, child)| selectable(child).then_some(child_path(path, index)))
        .unwrap_or_else(|| path.to_vec())
}

pub(super) fn apply_checked_state(props: &mut UiContextMenuProps, path: &[usize]) {
    let Some(target) = item_for_path(&props.items, path) else {
        return;
    };
    let kind = target.kind;
    let radio_group = target.radio_group.clone();
    if kind == UiContextMenuItemKind::Toggle
        && let Some(item) = item_for_path_mut(&mut props.items, path)
    {
        item.checked = !item.checked;
    }
    if kind == UiContextMenuItemKind::Radio {
        set_radio_group(&mut props.items, path, &radio_group);
    }
}

fn child_path(path: &[usize], child_index: usize) -> Vec<usize> {
    let mut result = path.to_vec();
    result.push(child_index);
    result
}

fn path_for_command_in_items(
    items: &[UiContextMenuItem],
    command: &str,
    parent_path: &[usize],
) -> Option<Vec<usize>> {
    for (index, item) in items.iter().enumerate() {
        let path = child_path(parent_path, index);
        if selectable(item) && item.id == command {
            return Some(path);
        }
        if selectable(item)
            && let Some(child_path) = path_for_command_in_items(&item.children, command, &path)
        {
            return Some(child_path);
        }
    }
    None
}

fn set_radio_group(items: &mut [UiContextMenuItem], path: &[usize], radio_group: &str) {
    for item in &mut *items {
        if item.kind == UiContextMenuItemKind::Radio && item.radio_group == radio_group {
            item.checked = false;
        }
        set_radio_group(&mut item.children, path, radio_group);
    }
    if let Some(item) = item_for_path_mut(items, path) {
        item.checked = true;
    }
}

fn item_for_path<'a>(
    items: &'a [UiContextMenuItem],
    path: &[usize],
) -> Option<&'a UiContextMenuItem> {
    let (first, rest) = path.split_first()?;
    let item = items.get(*first)?;
    if rest.is_empty() {
        Some(item)
    } else {
        item_for_path(&item.children, rest)
    }
}

fn item_for_path_mut<'a>(
    items: &'a mut [UiContextMenuItem],
    path: &[usize],
) -> Option<&'a mut UiContextMenuItem> {
    let (first, rest) = path.split_first()?;
    let item = items.get_mut(*first)?;
    if rest.is_empty() {
        Some(item)
    } else {
        item_for_path_mut(&mut item.children, rest)
    }
}

fn selectable(item: &UiContextMenuItem) -> bool {
    !item.disabled
        && !matches!(
            item.kind,
            UiContextMenuItemKind::Divider | UiContextMenuItemKind::Section
        )
}
