#![cfg(feature = "raster-host")]

use katana_ui_core::raster_host::{
    UiTreeDocumentTypography, UiTreeTextRoleBaselineTypography, UiTreeTextRoleTypography,
};

#[test]
fn public_raster_typography_exposes_higher_heading_builders() {
    const H4: UiTreeTextRoleBaselineTypography =
        UiTreeTextRoleBaselineTypography::new(17.507, 26.5, 15.5);
    const H5: UiTreeTextRoleBaselineTypography =
        UiTreeTextRoleBaselineTypography::new(16.338, 24.5, 14.5);
    const H6: UiTreeTextRoleTypography = UiTreeTextRoleTypography::new(15.169, 23, 0);
    const TYPOGRAPHY: UiTreeDocumentTypography = UiTreeDocumentTypography::new()
        .with_heading_4_baseline(H4)
        .with_heading_5_baseline(H5)
        .with_heading_6(H6);

    let copied = TYPOGRAPHY;
    let _: UiTreeDocumentTypography = copied;
}
