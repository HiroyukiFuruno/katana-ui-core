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

#[test]
fn tab_and_group_capability_debug_formats_are_stable() {
    let tab_capabilities = SanitizedTabCapabilities::new()
        .active_state(true)
        .dirty_state(false)
        .pinned_state(true)
        .close_state(false);
    let group_capabilities = SanitizedTabGroupCapabilities::new()
        .collapse_state(true)
        .menu_state(false)
        .rename_state(true)
        .recolor_state(false)
        .close_state(true)
        .ungroup_state(false)
        .drag_state(true);

    assert_eq!(
        format!("{:?}", tab_capabilities),
        "SanitizedTabCapabilities { active: true, dirty: false, pinned: true, close: false }"
    );
    assert_eq!(
        format!("{:?}", group_capabilities),
        "SanitizedTabGroupCapabilities { collapse: true, menu: false, rename: true, recolor: false, close: true, ungroup: false, drag: true }"
    );
}

#[test]
fn tab_and_group_debug_formats_expose_localized_label_and_flags() {
    let tab = SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1, 2]), 7, "編集")
        .with_capabilities(
            SanitizedTabCapabilities::new()
                .active_state(true)
                .dirty_state(false)
                .pinned_state(true)
                .close_state(true),
        );
    let tab_debug = format!("{:?}", tab);
    let group = SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([3, 4]),
        3,
        "固定",
    )
    .with_capabilities(
        SanitizedTabGroupCapabilities::new()
            .collapse_state(false)
            .menu_state(true)
            .rename_state(false)
            .recolor_state(true)
            .close_state(false)
            .ungroup_state(true)
            .drag_state(false),
    )
    .tab(tab);

    assert!(tab_debug.starts_with("SanitizedTab {"));
    assert!(format!("{:?}", group).starts_with("SanitizedTabGroup {"));
    let group_debug = format!("{:?}", group);

    assert!(tab_debug.contains("order: 7"));
    assert!(tab_debug.contains("label: \"編集\""));
    assert!(tab_debug.contains("icon: false"));
    assert!(group_debug.contains("order: 3"));
    assert!(group_debug.contains("label: \"固定\""));
}
