use super::{
    SanitizedTab, SanitizedTabCapabilities, SanitizedTabClosePresentation, SanitizedTabGroup,
    SanitizedTabGroupCapabilities, SanitizedTabGroupTarget, SanitizedTabProjection,
    SanitizedTabTarget,
};
use katana_ui_core::render_model::UiIconProps;

#[test]
fn construction_preserves_localized_nested_order_and_capabilities() {
    let tab = SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1, 2]), 7, "編集")
        .with_icon(UiIconProps::new("<svg/>"))
        .with_capabilities(
            SanitizedTabCapabilities::new()
                .active_state(true)
                .dirty_state(true)
                .pinned_state(true)
                .close_state(true),
        );
    let group_capabilities = SanitizedTabGroupCapabilities::new()
        .collapse_state(true)
        .menu_state(true)
        .rename_state(true)
        .recolor_state(true)
        .close_state(true)
        .ungroup_state(true)
        .drag_state(true);
    assert_eq!(format!("{:?}", tab.target), "SanitizedTabTarget(..)");
    let child = SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([3, 4, 5]),
        3,
        "固定",
    )
    .with_capabilities(group_capabilities)
    .tab(tab);
    let projection = SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([9, 9, 1]),
        9,
        "作業",
    )
    .group(child)]);

    let group = &projection.groups[0];
    assert_eq!(group.order, 9);
    assert_eq!(group.label, "作業");
    assert!(group.icon.is_none());
    assert_eq!(format!("{:?}", group.target), "SanitizedTabGroupTarget(..)");
    assert_eq!(group.groups.len(), 1);
    let nested = &group.groups[0];
    assert_eq!(nested.order, 3);
    assert_eq!(nested.label, "固定");
    assert_eq!(
        format!("{:?}", nested.target),
        "SanitizedTabGroupTarget(..)"
    );
    assert_eq!(nested.tabs.len(), 1);
    assert!(nested.capabilities.collapse);
    assert!(nested.capabilities.menu);
    assert!(nested.capabilities.rename);
    assert!(nested.capabilities.recolor);
    assert!(nested.capabilities.close);
    assert!(nested.capabilities.ungroup);
    assert!(nested.capabilities.drag);
    assert_eq!(nested.tabs[0].order, 7);
    assert_eq!(nested.tabs[0].label, "編集");
    assert!(nested.tabs[0].icon.is_some());
    assert!(nested.tabs[0].capabilities.active);
    assert!(nested.tabs[0].capabilities.dirty);
    assert!(nested.tabs[0].capabilities.pinned);
    assert!(nested.tabs[0].capabilities.close);
}

#[test]
fn close_presentation_requires_caller_supplied_localized_labels() {
    let presentation =
        SanitizedTabClosePresentation::new("閉じる", "タブを閉じる", "編集タブを閉じる");
    let tab = SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 0, "編集")
        .with_close_presentation(presentation);

    let close_presentation = tab
        .close_presentation
        .as_ref()
        .expect("close presentation must be caller supplied");
    assert_eq!(close_presentation.visible_label.as_str(), "閉じる");
    assert_eq!(close_presentation.tooltip.as_str(), "タブを閉じる");
    assert_eq!(
        close_presentation.accessibility_label.as_str(),
        "編集タブを閉じる"
    );
}

#[test]
fn construction_can_represent_sibling_groups_and_tabs() {
    let projection = SanitizedTabProjection::new([
        SanitizedTabGroup::new(SanitizedTabGroupTarget::from_opaque_bytes([2]), 2, "右")
            .tab(SanitizedTab::new(
                SanitizedTabTarget::from_opaque_bytes([2]),
                2,
                "B",
            ))
            .tab(SanitizedTab::new(
                SanitizedTabTarget::from_opaque_bytes([3]),
                3,
                "C",
            )),
        SanitizedTabGroup::new(SanitizedTabGroupTarget::from_opaque_bytes([1]), 1, "左").tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 1, "A"),
        ),
    ]);

    assert_eq!(projection.groups.len(), 2);
    assert_eq!(projection.groups[0].order, 2);
    assert_eq!(projection.groups[0].tabs.len(), 2);
    assert_eq!(projection.groups[1].order, 1);
    assert_eq!(projection.groups[1].tabs[0].label, "A");
}

#[test]
fn opaque_debug_does_not_reveal_target_bytes() {
    let target = SanitizedTabTarget::from_opaque_bytes([0xde, 0xad, 0xbe, 0xef]);

    assert_eq!(format!("{target:?}"), "SanitizedTabTarget(..)");
}

#[test]
fn group_fingerprint_tracks_target_and_capabilities() {
    let base = SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([1]),
        0,
        "root",
    )
    .with_capabilities(SanitizedTabGroupCapabilities::new())
    .tab(SanitizedTab::new(
        SanitizedTabTarget::from_opaque_bytes([11]),
        1,
        "Tab",
    ))]);
    let target_changed = SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([2]),
        0,
        "root",
    )
    .with_capabilities(SanitizedTabGroupCapabilities::new())
    .tab(SanitizedTab::new(
        SanitizedTabTarget::from_opaque_bytes([11]),
        1,
        "Tab",
    ))]);
    let capability_changed = SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([1]),
        0,
        "root",
    )
    .with_capabilities(
        SanitizedTabGroupCapabilities::new()
            .collapse_state(true)
            .menu_state(true),
    )
    .tab(SanitizedTab::new(
        SanitizedTabTarget::from_opaque_bytes([11]),
        1,
        "Tab",
    ))]);

    assert_ne!(
        base.stable_fingerprint(),
        target_changed.stable_fingerprint()
    );
    assert_ne!(
        base.stable_fingerprint(),
        capability_changed.stable_fingerprint()
    );
}

#[test]
fn nested_group_targets_remain_distinct_without_public_readback() {
    let first = SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([0]),
        0,
        "root",
    )
    .group(SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([1]),
        0,
        "nested",
    ))]);
    let second = SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([0]),
        0,
        "root",
    )
    .group(SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([2]),
        0,
        "nested",
    ))]);

    assert_ne!(first.stable_fingerprint(), second.stable_fingerprint());
}

#[test]
fn close_presentation_fingerprint_distinguishes_absent_and_different_values() {
    let base = SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([0]),
        0,
        "タブ",
    )
    .tab(SanitizedTab::new(
        SanitizedTabTarget::from_opaque_bytes([1]),
        0,
        "編集",
    ))]);
    let present = SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([0]),
        0,
        "タブ",
    )
    .tab(
        SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 0, "編集")
            .with_close_presentation(SanitizedTabClosePresentation::new(
                "閉じる",
                "タブを閉じる",
                "編集タブを閉じる",
            )),
    )]);
    let visible_label_changed = SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([0]),
        0,
        "タブ",
    )
    .tab(
        SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 0, "編集")
            .with_close_presentation(SanitizedTabClosePresentation::new(
                "閉じる別表記",
                "タブを閉じる",
                "編集タブを閉じる",
            )),
    )]);
    let tooltip_changed = SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([0]),
        0,
        "タブ",
    )
    .tab(
        SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 0, "編集")
            .with_close_presentation(SanitizedTabClosePresentation::new(
                "閉じる",
                "別のツールチップ",
                "編集タブを閉じる",
            )),
    )]);
    let accessibility_label_changed = SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([0]),
        0,
        "タブ",
    )
    .tab(
        SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 0, "編集")
            .with_close_presentation(SanitizedTabClosePresentation::new(
                "閉じる",
                "タブを閉じる",
                "別のアクセシビリティラベル",
            )),
    )]);

    let base_fingerprint = base.stable_fingerprint();
    assert_ne!(base_fingerprint, present.stable_fingerprint());
    assert_ne!(
        present.stable_fingerprint(),
        visible_label_changed.stable_fingerprint()
    );
    assert_ne!(
        present.stable_fingerprint(),
        tooltip_changed.stable_fingerprint()
    );
    assert_ne!(
        present.stable_fingerprint(),
        accessibility_label_changed.stable_fingerprint()
    );
}

#[test]
fn close_presentation_debug_does_not_reveal_localized_labels() {
    let presentation = SanitizedTabClosePresentation::new(
        "可視ラベルの秘密",
        "ツールチップの秘密",
        "アクセシビリティの秘密",
    );
    let debug = format!("{presentation:?}");

    assert_eq!(debug, "SanitizedTabClosePresentation(..)");
    assert!(!debug.contains("可視ラベルの秘密"));
    assert!(!debug.contains("ツールチップの秘密"));
    assert!(!debug.contains("アクセシビリティの秘密"));
}

include!("coverage.rs");
include!("api.rs");
include!("debug.rs");
