//! Document-role typography overrides for framework-neutral raster hosts.

/// Typography values for one document text role.
///
/// Values are expressed in the consumer's logical line-box coordinate system.
/// The raster host converts the target baseline into its own raster draw
/// origin, so consumers never need to guess a font-engine-specific offset.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiTreeTextRoleTypography {
    /// Raster font size in logical pixels.
    pub font_size: f32,
    /// Total logical line-box height in pixels.
    pub line_box_height: f32,
    /// Vertical offset from the line-box origin to the text baseline.
    pub baseline_from_line_box_top: f32,
}

impl UiTreeTextRoleTypography {
    /// Creates one role's independent raster typography values.
    #[must_use]
    pub const fn new(
        font_size: f32,
        line_box_height: f32,
        baseline_from_line_box_top: f32,
    ) -> Self {
        Self {
            font_size,
            line_box_height,
            baseline_from_line_box_top,
        }
    }

    pub(in crate::raster_host) fn is_valid(self) -> bool {
        self.font_size.is_finite()
            && self.font_size > 0.0
            && self.line_box_height.is_finite()
            && self.line_box_height > 0.0
            && self.baseline_from_line_box_top.is_finite()
            && self.baseline_from_line_box_top >= 0.0
            && self.baseline_from_line_box_top < self.line_box_height
    }
}

/// Optional document-role typography overrides for a raster host.
///
/// Roles that are not configured retain the metrics derived from the supplied
/// [`ThemeSnapshot`](crate::theme::ThemeSnapshot). Invalid role values are
/// ignored so the existing theme-derived metrics remain active.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct UiTreeDocumentTypography {
    body: Option<UiTreeTextRoleTypography>,
    heading_1: Option<UiTreeTextRoleTypography>,
    heading_2: Option<UiTreeTextRoleTypography>,
    heading_3: Option<UiTreeTextRoleTypography>,
}

impl UiTreeDocumentTypography {
    /// Creates an override set that preserves all theme-derived metrics.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            body: None,
            heading_1: None,
            heading_2: None,
            heading_3: None,
        }
    }

    /// Overrides document body text metrics.
    #[must_use]
    pub const fn with_body(mut self, typography: UiTreeTextRoleTypography) -> Self {
        self.body = Some(typography);
        self
    }

    /// Overrides first-level document heading metrics.
    #[must_use]
    pub const fn with_heading_1(mut self, typography: UiTreeTextRoleTypography) -> Self {
        self.heading_1 = Some(typography);
        self
    }

    /// Overrides second-level document heading metrics.
    #[must_use]
    pub const fn with_heading_2(mut self, typography: UiTreeTextRoleTypography) -> Self {
        self.heading_2 = Some(typography);
        self
    }

    /// Overrides third-level document heading metrics.
    #[must_use]
    pub const fn with_heading_3(mut self, typography: UiTreeTextRoleTypography) -> Self {
        self.heading_3 = Some(typography);
        self
    }

    pub(in crate::raster_host) const fn body(self) -> Option<UiTreeTextRoleTypography> {
        self.body
    }

    pub(in crate::raster_host) const fn heading_1(self) -> Option<UiTreeTextRoleTypography> {
        self.heading_1
    }

    pub(in crate::raster_host) const fn heading_2(self) -> Option<UiTreeTextRoleTypography> {
        self.heading_2
    }

    pub(in crate::raster_host) const fn heading_3(self) -> Option<UiTreeTextRoleTypography> {
        self.heading_3
    }
}

#[cfg(test)]
mod tests {
    use super::{UiTreeDocumentTypography, UiTreeTextRoleTypography};

    #[test]
    fn role_overrides_are_optional_and_keep_independent_metrics() {
        let body = UiTreeTextRoleTypography::new(16.5, 23.0, 0.0);
        let heading = UiTreeTextRoleTypography::new(24.75, 40.0, 9.0);
        let typography = UiTreeDocumentTypography::new()
            .with_body(body)
            .with_heading_1(heading);

        assert_eq!(Some(body), typography.body());
        assert_eq!(Some(heading), typography.heading_1());
        assert_eq!(None, typography.heading_2());
        assert_eq!(None, typography.heading_3());
    }

    #[test]
    fn invalid_role_values_are_rejected_by_the_raster_host_boundary() {
        assert!(!UiTreeTextRoleTypography::new(0.0, 23.0, 0.0).is_valid());
        assert!(!UiTreeTextRoleTypography::new(f32::NAN, 23.0, 0.0).is_valid());
        assert!(!UiTreeTextRoleTypography::new(16.5, 0.0, 0.0).is_valid());
        assert!(!UiTreeTextRoleTypography::new(16.5, 23.0, 23.0).is_valid());
        assert!(UiTreeTextRoleTypography::new(16.5, 23.5, 0.5).is_valid());
    }
}
