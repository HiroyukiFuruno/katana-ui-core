//! Generic opaque command projection.

#[path = "sanitized_command_projection/dropdown.rs"]
mod dropdown;
#[path = "sanitized_command_projection/group.rs"]
mod group;
#[path = "sanitized_command_projection/item.rs"]
mod item;
#[path = "sanitized_command_projection/projection.rs"]
mod projection;
#[path = "sanitized_command_projection/target.rs"]
mod target;

pub use dropdown::SanitizedCommandDropdownItem;
pub use group::SanitizedCommandGroup;
pub use item::SanitizedCommandItem;
pub use projection::SanitizedCommandProjection;
pub(super) use target::CommandCapability;
pub use target::{SanitizedCommandCapabilityRejection, SanitizedCommandTarget};

#[cfg(test)]
mod tests {
    use super::{
        SanitizedCommandDropdownItem, SanitizedCommandGroup, SanitizedCommandItem,
        SanitizedCommandProjection, SanitizedCommandTarget,
    };
    use crate::render_model::UiIconProps;

    #[test]
    fn generic_projection_preserves_contract_fields() {
        let child = SanitizedCommandDropdownItem::new(
            SanitizedCommandTarget::from_opaque_bytes([3, 4]),
            7,
            "子項目",
        )
        .tooltip_text("子項目の説明")
        .accessibility_label_text("子項目")
        .enabled_state(false);
        let item =
            SanitizedCommandItem::new(SanitizedCommandTarget::from_opaque_bytes([1, 2]), 5, "編集")
                .tooltip_text("編集操作")
                .accessibility_label_text("編集")
                .with_icon(UiIconProps::new("<svg/>"))
                .visible_state(false)
                .dropdown_item(child);
        let projection = SanitizedCommandProjection::new([SanitizedCommandGroup::new(2, "操作")
            .tooltip_text("操作一覧")
            .accessibility_label_text("操作")
            .item(item)]);

        let group = &projection.groups()[0];
        let item = &group.items()[0];
        assert_eq!(group.order(), 2);
        assert_eq!(group.label(), "操作");
        assert_eq!(item.order(), 5);
        assert_eq!(item.label(), "編集");
        assert_eq!(item.tooltip(), Some("編集操作"));
        assert_eq!(item.accessibility_label(), Some("編集"));
        assert!(item.icon().is_some());
        assert!(!item.visible());
        assert_eq!(item.dropdown_items().len(), 1);
        assert_eq!(item.dropdown_items()[0].order(), 7);
        assert!(!item.dropdown_items()[0].enabled());
    }

    #[test]
    fn opaque_target_debug_does_not_reveal_payload() {
        let target = SanitizedCommandTarget::from_opaque_bytes([0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(format!("{target:?}"), "SanitizedCommandTarget(..)");
    }

    #[test]
    fn opaque_target_fingerprint_is_stable_for_equal_bytes_and_distinct_for_other_bytes() {
        let first = SanitizedCommandTarget::from_opaque_bytes([1, 2, 3]);
        let same = SanitizedCommandTarget::from_opaque_bytes([1, 2, 3]);
        let different = SanitizedCommandTarget::from_opaque_bytes([1, 2, 4]);

        assert_eq!(first.stable_fingerprint(), same.stable_fingerprint());
        assert_ne!(first.stable_fingerprint(), different.stable_fingerprint());
    }

    #[test]
    fn projection_read_accessors_are_not_public_api() {
        let source = [
            include_str!("sanitized_command_projection/target.rs"),
            include_str!("sanitized_command_projection/projection.rs"),
            include_str!("sanitized_command_projection/group.rs"),
            include_str!("sanitized_command_projection/item.rs"),
            include_str!("sanitized_command_projection/dropdown.rs"),
            include_str!("sanitized_command_projection.rs"),
        ]
        .join("\n");
        let api_source = source.split("#[cfg(test)]").next().unwrap_or(&source);
        let read_accessors = [
            "groups",
            "target",
            "order",
            "label",
            "tooltip",
            "accessibility_label",
            "icon",
            "enabled",
            "visible",
            "items",
            "dropdown_items",
        ];

        for accessor in read_accessors {
            let mut declarations = api_source.lines().filter(|line| {
                let declaration = line.trim_start();
                declaration.contains(&format!("fn {accessor}(&"))
                    || declaration.contains(&format!("fn {accessor}(&self"))
            });
            assert!(
                declarations.clone().count() > 0,
                "missing accessor declaration for {accessor}"
            );
            assert!(
                declarations.all(|line| {
                    let declaration = line.trim_start();
                    declaration.starts_with("pub(super) ") || declaration.starts_with("pub(crate) ")
                }),
                "read accessor {accessor} must remain KUC-internal"
            );
        }

        let public_methods = api_source.lines().filter_map(|line| {
            let declaration = line.trim_start();
            (declaration.starts_with("pub fn ") || declaration.starts_with("pub const fn "))
                .then_some(declaration)
        });
        let allowed_public_methods = [
            "from_opaque_bytes",
            "with_unit_capability",
            "new",
            "tooltip_text",
            "accessibility_label_text",
            "with_icon",
            "enabled_state",
            "visible_state",
            "item",
            "dropdown_item",
        ];
        for declaration in public_methods {
            assert!(
                allowed_public_methods
                    .iter()
                    .any(|name| declaration.contains(&format!("fn {name}"))),
                "unexpected public method: {declaration}"
            );
        }

        assert!(!api_source.contains(&["derive(", "Clone"].concat()));
        assert!(!api_source.contains(&["derive(", "Serial", "ize"].concat()));
        assert!(!api_source.contains(&["pub fn ", "bytes("].concat()));
        assert!(!api_source.contains(&["pub fn ", "opaque_bytes("].concat()));
    }

    #[test]
    fn command_projection_fingerprint_tracks_nested_optional_fields() {
        let target = || SanitizedCommandTarget::from_opaque_bytes([1, 2, 3]);
        let base = SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "グループ")
            .tooltip_text("説明")
            .accessibility_label_text("グループ")
            .with_icon(UiIconProps::new("<svg/>"))
            .enabled_state(false)
            .visible_state(false)
            .item(
                SanitizedCommandItem::new(target(), 2, "項目").dropdown_item(
                    SanitizedCommandDropdownItem::new(target(), 3, "子")
                        .tooltip_text("子の説明")
                        .accessibility_label_text("子")
                        .with_icon(UiIconProps::new("<svg-child/>"))
                        .enabled_state(false)
                        .visible_state(false),
                ),
            )]);
        let changed = SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "グループ")
            .item(SanitizedCommandItem::new(target(), 2, "項目"))]);

        assert_ne!(base.stable_fingerprint(), changed.stable_fingerprint());
        assert!(format!("{base:?}").contains("SanitizedCommandProjection"));
    }
}
