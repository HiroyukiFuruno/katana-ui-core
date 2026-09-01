use super::types::ContextMenuPresentationItem;
use crate::molecule::selection::ContextMenuItem;

pub(super) fn core_items(items: &[ContextMenuPresentationItem]) -> Vec<ContextMenuItem> {
    items.iter().map(core_item).collect()
}

pub(super) fn visible_items<'a>(
    items: &'a [ContextMenuPresentationItem],
    submenu_path: &[usize],
) -> &'a [ContextMenuPresentationItem] {
    submenu_path
        .iter()
        .try_fold(items, |current, index| {
            current.get(*index).map(|item| item.children.as_slice())
        })
        .unwrap_or(items)
}

pub(super) fn full_path(parent: &[usize], index: usize) -> Vec<usize> {
    let mut path = parent.to_vec();
    path.push(index);
    path
}

fn core_item(source: &ContextMenuPresentationItem) -> ContextMenuItem {
    let mut item = ContextMenuItem::new(source.id.clone(), source.label.clone(), source.kind)
        .disabled(!source.enabled)
        .checked(source.checked)
        .accessibility_label(source.accessibility_label.clone());
    for child in &source.children {
        item = item.child(core_item(child));
    }
    item
}
