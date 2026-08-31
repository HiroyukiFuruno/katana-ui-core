#[test]
fn projection_identity_is_stable_for_equal_values_and_changes_for_text_icon_and_order() {
    let base = SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([1]),
        0,
        "group",
    )
    .tab(SanitizedTab::new(
        SanitizedTabTarget::from_opaque_bytes([2]),
        0,
        "tab",
    ))]);
    let equal = SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([1]),
        0,
        "group",
    )
    .tab(SanitizedTab::new(
        SanitizedTabTarget::from_opaque_bytes([2]),
        0,
        "tab",
    ))]);
    let text_changed = SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([1]),
        0,
        "changed",
    )]);
    let icon_changed = SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([1]),
        1,
        "group",
    )
    .with_icon(UiIconProps::new("<svg/>"))]);

    assert!(base.same_as(&equal));
    assert!(!base.same_as(&text_changed));
    assert!(!base.same_as(&icon_changed));
    assert_eq!(base.stable_fingerprint().len(), 64);
}
