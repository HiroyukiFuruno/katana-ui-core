use super::{UiTreeDocumentTypography, UiTreeTextRoleBaselineTypography, UiTreeTextRoleTypography};

#[test]
fn role_overrides_are_optional_and_keep_independent_metrics() {
    let body = UiTreeTextRoleTypography::new(16.5, 23, 0);
    let heading = UiTreeTextRoleTypography::new(24.75, 40, 9);
    let typography = UiTreeDocumentTypography::new()
        .with_body(body)
        .with_heading_1(heading);

    assert_eq!(Some(body), typography.body());
    assert_eq!(Some(heading), typography.heading_1());
    assert_eq!(None, typography.heading_2());
    assert_eq!(None, typography.heading_3());
}

#[test]
fn additive_baseline_api_preserves_role_selection_and_legacy_conversion() {
    let legacy = UiTreeTextRoleTypography::new(16.5, 23, 4);
    let baseline = legacy.with_baseline_from_line_box_top(23.5, 18.5);
    assert_eq!(
        baseline,
        UiTreeTextRoleBaselineTypography::new(16.5, 23.5, 18.5)
    );

    let body = UiTreeTextRoleBaselineTypography::new(16.5, 23.5, 18.5);
    let heading_1 = UiTreeTextRoleBaselineTypography::new(24.75, 40.0, 30.0);
    let heading_2 = UiTreeTextRoleBaselineTypography::new(22.0, 34.0, 25.0);
    let heading_3 = UiTreeTextRoleBaselineTypography::new(20.0, 30.0, 22.0);
    let typography = UiTreeDocumentTypography::new()
        .with_body_baseline(body)
        .with_heading_1_baseline(heading_1)
        .with_heading_2_baseline(heading_2)
        .with_heading_3_baseline(heading_3);

    assert_eq!(Some(body), typography.body_baseline());
    assert_eq!(Some(heading_1), typography.heading_1_baseline());
    assert_eq!(Some(heading_2), typography.heading_2_baseline());
    assert_eq!(Some(heading_3), typography.heading_3_baseline());
    assert_eq!(None, typography.body());
    assert_eq!(None, typography.heading_1());
    assert_eq!(None, typography.heading_2());
    assert_eq!(None, typography.heading_3());

    let legacy_typography = UiTreeDocumentTypography::new()
        .with_body(UiTreeTextRoleTypography::new(16.5, 23, 4))
        .with_heading_1(UiTreeTextRoleTypography::new(24.75, 40, 10))
        .with_heading_2(UiTreeTextRoleTypography::new(22.0, 34, 5))
        .with_heading_3(UiTreeTextRoleTypography::new(20.0, 30, 4));

    assert!(typography.has_fractional_baseline());
    assert!(!legacy_typography.has_fractional_baseline());
}

#[test]
fn heading_4_through_6_builders_are_public_const_and_keep_slots_exclusive() {
    const H4: UiTreeTextRoleBaselineTypography =
        UiTreeTextRoleBaselineTypography::new(17.507, 26.5, 15.5);
    const H5: UiTreeTextRoleBaselineTypography =
        UiTreeTextRoleBaselineTypography::new(16.338, 24.5, 14.5);
    const H6: UiTreeTextRoleBaselineTypography =
        UiTreeTextRoleBaselineTypography::new(15.169, 23.0, 13.5);
    const TYPOGRAPHY: UiTreeDocumentTypography = UiTreeDocumentTypography::new()
        .with_heading_4_baseline(H4)
        .with_heading_5_baseline(H5)
        .with_heading_6_baseline(H6);

    let copied = TYPOGRAPHY;
    assert_eq!(Some(H4), copied.heading_4_baseline());
    assert_eq!(Some(H5), copied.heading_5_baseline());
    assert_eq!(Some(H6), copied.heading_6_baseline());
    assert!(copied.has_fractional_baseline());

    let legacy_h4 = UiTreeTextRoleTypography::new(17.0, 27, 1);
    let exclusive = copied.with_heading_4(legacy_h4);
    assert_eq!(Some(legacy_h4), exclusive.heading_4());
    assert_eq!(None, exclusive.heading_4_baseline());
    assert_eq!(Some(H5), exclusive.heading_5_baseline());
}

#[test]
fn higher_heading_legacy_and_baseline_slots_are_independent() {
    let h4_legacy = UiTreeTextRoleTypography::new(17.0, 27, 1);
    let h5_legacy = UiTreeTextRoleTypography::new(16.0, 25, 1);
    let h6_legacy = UiTreeTextRoleTypography::new(15.0, 23, 1);
    let legacy = UiTreeDocumentTypography::new()
        .with_heading_4(h4_legacy)
        .with_heading_5(h5_legacy)
        .with_heading_6(h6_legacy);

    assert_eq!(Some(h4_legacy), legacy.heading_4());
    assert_eq!(Some(h5_legacy), legacy.heading_5());
    assert_eq!(Some(h6_legacy), legacy.heading_6());
    assert_eq!(None, legacy.heading_4_baseline());
    assert_eq!(None, legacy.heading_5_baseline());
    assert_eq!(None, legacy.heading_6_baseline());
    assert!(!legacy.has_fractional_baseline());

    let h4 = UiTreeTextRoleBaselineTypography::new(17.507, 26.5, 15.5);
    let h5 = UiTreeTextRoleBaselineTypography::new(16.338, 24.5, 14.5);
    let h6 = UiTreeTextRoleBaselineTypography::new(15.169, 23.0, 13.5);
    let baseline = legacy
        .with_heading_4_baseline(h4)
        .with_heading_5_baseline(h5)
        .with_heading_6_baseline(h6);

    assert_eq!(None, baseline.heading_4());
    assert_eq!(None, baseline.heading_5());
    assert_eq!(None, baseline.heading_6());
    assert_eq!(Some(h4), baseline.heading_4_baseline());
    assert_eq!(Some(h5), baseline.heading_5_baseline());
    assert_eq!(Some(h6), baseline.heading_6_baseline());
}

#[test]
fn each_higher_heading_baseline_enables_fractional_layout() {
    let baseline = UiTreeTextRoleBaselineTypography::new(16.0, 24.5, 14.5);

    assert!(
        UiTreeDocumentTypography::new()
            .with_heading_4_baseline(baseline)
            .has_fractional_baseline()
    );
    assert!(
        UiTreeDocumentTypography::new()
            .with_heading_5_baseline(baseline)
            .has_fractional_baseline()
    );
    assert!(
        UiTreeDocumentTypography::new()
            .with_heading_6_baseline(baseline)
            .has_fractional_baseline()
    );
}

#[test]
fn invalid_role_values_are_rejected_by_the_raster_host_boundary() {
    assert!(!UiTreeTextRoleTypography::new(0.0, 23, 0).is_valid());
    assert!(!UiTreeTextRoleTypography::new(f32::NAN, 23, 0).is_valid());
    assert!(!UiTreeTextRoleTypography::new(16.5, 0, 0).is_valid());
    assert!(!UiTreeTextRoleTypography::new(16.5, 23, 23).is_valid());
    assert!(UiTreeTextRoleBaselineTypography::new(16.5, 23.5, 0.5).is_valid());
}
