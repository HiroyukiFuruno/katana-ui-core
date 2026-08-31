use super::sanitized_context_projection::{
    SanitizedContextMenuItem, SanitizedContextMenuProjection,
};
use crate::context_menu::{ContextMenuPresentation, ContextMenuPresentationItem};
use katana_ui_core::molecule::selection::ContextMenuItemKind;
use sha2::{Digest, Sha256};

pub(super) fn context_menu_presentation(
    projection: &SanitizedContextMenuProjection,
) -> ContextMenuPresentation {
    ContextMenuPresentation {
        visible: !projection.items().is_empty(),
        items: ordered_items(projection.items())
            .into_iter()
            .map(presentation_item)
            .collect(),
    }
}

fn ordered_items(items: &[SanitizedContextMenuItem]) -> Vec<&SanitizedContextMenuItem> {
    let mut ordered = items.iter().enumerate().collect::<Vec<_>>();
    ordered.sort_by_key(|(index, item)| (item.order(), *index));
    ordered.into_iter().map(|(_, item)| item).collect()
}

fn presentation_item(item: &SanitizedContextMenuItem) -> ContextMenuPresentationItem {
    let children = ordered_items(item.submenu())
        .into_iter()
        .map(presentation_item)
        .collect::<Vec<_>>();
    let kind = if children.is_empty() {
        ContextMenuItemKind::Action
    } else {
        ContextMenuItemKind::Submenu
    };

    ContextMenuPresentationItem {
        id: target_id(item),
        label: item.label().to_owned(),
        accessibility_label: item.accessibility_label().unwrap_or_default().to_owned(),
        icon: item.icon().cloned(),
        enabled: item.enabled(),
        checked: item.checked(),
        kind,
        children,
    }
}

fn target_id(item: &SanitizedContextMenuItem) -> String {
    let mut digest = Sha256::new();
    digest.update((item.target().opaque().len() as u64).to_le_bytes());
    digest.update(item.target().opaque());
    format!(
        concat!("kuc-context-menu-", "{}"),
        hex::encode(digest.finalize())
    )
}

#[cfg(test)]
mod tests {
    use super::context_menu_presentation;
    use crate::text_command_surface::{
        SanitizedContextMenuItem, SanitizedContextMenuProjection, SanitizedContextMenuTarget,
    };
    use katana_ui_core::molecule::selection::ContextMenuItemKind;
    use katana_ui_core::render_model::UiIconProps;

    #[test]
    fn maps_opaque_identity_attributes_order_and_nested_items() {
        let projection = SanitizedContextMenuProjection::new([
            SanitizedContextMenuItem::new(
                SanitizedContextMenuTarget::from_opaque_bytes([2]),
                20,
                "遅い項目",
            ),
            SanitizedContextMenuItem::new(
                SanitizedContextMenuTarget::from_opaque_bytes([1]),
                10,
                "表示 ⭐️",
            )
            .accessibility_label_text("表示設定")
            .with_icon(UiIconProps::new("<svg/>"))
            .enabled_state(false)
            .checked_state(true)
            .submenu_item(SanitizedContextMenuItem::new(
                SanitizedContextMenuTarget::from_opaque_bytes([3]),
                1,
                "子項目",
            )),
        ]);

        let presentation = context_menu_presentation(&projection);
        assert!(presentation.visible);
        assert_eq!(presentation.items.len(), 2);
        assert_eq!(presentation.items[0].label, "表示 ⭐️");
        assert_eq!(presentation.items[0].kind, ContextMenuItemKind::Submenu);
        assert_eq!(presentation.items[0].accessibility_label, "表示設定");
        assert!(presentation.items[0].icon.is_some());
        assert!(!presentation.items[0].enabled);
        assert!(presentation.items[0].checked);
        assert_eq!(presentation.items[0].children.len(), 1);
        assert_eq!(
            presentation.items[0].children[0].kind,
            ContextMenuItemKind::Action
        );
        assert_ne!(presentation.items[0].id, presentation.items[1].id);
        assert!(presentation.items[0].id.starts_with("kuc-context-menu-"));
    }

    #[test]
    fn empty_projection_is_present_but_not_visible() {
        let presentation = context_menu_presentation(&SanitizedContextMenuProjection::default());

        assert!(!presentation.visible);
        assert!(presentation.items.is_empty());
    }
}
