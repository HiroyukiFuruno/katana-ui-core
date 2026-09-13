//! Public document-role typography contracts for raster hosts.

/// Typography values for one document text role.
///
/// `baseline_offset` is the vertical offset from the role line box origin to
/// the raster draw origin. This field and the constructor signature are kept
/// stable for consumers of the 0.3.x API.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiTreeTextRoleTypography {
    /// Raster font size in logical pixels.
    pub font_size: f32,
    /// Total logical line-box height in pixels.
    pub line_height: usize,
    /// Vertical offset from the line-box origin before raster drawing.
    pub baseline_offset: usize,
}

impl UiTreeTextRoleTypography {
    /// Creates one role's independent raster typography values.
    #[must_use]
    pub const fn new(font_size: f32, line_height: usize, baseline_offset: usize) -> Self {
        Self {
            font_size,
            line_height,
            baseline_offset,
        }
    }

    /// Creates an additive fractional baseline contract while preserving the
    /// legacy [`Self::new`] fields and meaning.
    #[must_use]
    pub const fn with_baseline_from_line_box_top(
        self,
        line_box_height: f32,
        baseline_from_line_box_top: f32,
    ) -> UiTreeTextRoleBaselineTypography {
        UiTreeTextRoleBaselineTypography::new(
            self.font_size,
            line_box_height,
            baseline_from_line_box_top,
        )
    }

    pub(in crate::raster_host) fn is_valid(self) -> bool {
        self.font_size.is_finite()
            && self.font_size > 0.0
            && self.line_height > 0
            && self.baseline_offset < self.line_height
    }
}

/// Additive fractional baseline contract for a document text role.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiTreeTextRoleBaselineTypography {
    pub font_size: f32,
    pub line_box_height: f32,
    pub baseline_from_line_box_top: f32,
}

impl UiTreeTextRoleBaselineTypography {
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
    body_baseline: Option<UiTreeTextRoleBaselineTypography>,
    heading_1: Option<UiTreeTextRoleTypography>,
    heading_1_baseline: Option<UiTreeTextRoleBaselineTypography>,
    heading_2: Option<UiTreeTextRoleTypography>,
    heading_2_baseline: Option<UiTreeTextRoleBaselineTypography>,
    heading_3: Option<UiTreeTextRoleTypography>,
    heading_3_baseline: Option<UiTreeTextRoleBaselineTypography>,
    heading_4: Option<UiTreeTextRoleTypography>,
    heading_4_baseline: Option<UiTreeTextRoleBaselineTypography>,
    heading_5: Option<UiTreeTextRoleTypography>,
    heading_5_baseline: Option<UiTreeTextRoleBaselineTypography>,
    heading_6: Option<UiTreeTextRoleTypography>,
    heading_6_baseline: Option<UiTreeTextRoleBaselineTypography>,
}

impl UiTreeDocumentTypography {
    pub(in crate::raster_host) const fn has_fractional_baseline(self) -> bool {
        self.body_baseline.is_some()
            || self.heading_1_baseline.is_some()
            || self.heading_2_baseline.is_some()
            || self.heading_3_baseline.is_some()
            || self.heading_4_baseline.is_some()
            || self.heading_5_baseline.is_some()
            || self.heading_6_baseline.is_some()
    }

    pub(in crate::raster_host) const fn body(self) -> Option<UiTreeTextRoleTypography> {
        self.body
    }

    pub(in crate::raster_host) const fn body_baseline(
        self,
    ) -> Option<UiTreeTextRoleBaselineTypography> {
        self.body_baseline
    }

    pub(in crate::raster_host) const fn heading_1(self) -> Option<UiTreeTextRoleTypography> {
        self.heading_1
    }

    pub(in crate::raster_host) const fn heading_1_baseline(
        self,
    ) -> Option<UiTreeTextRoleBaselineTypography> {
        self.heading_1_baseline
    }

    pub(in crate::raster_host) const fn heading_2(self) -> Option<UiTreeTextRoleTypography> {
        self.heading_2
    }

    pub(in crate::raster_host) const fn heading_2_baseline(
        self,
    ) -> Option<UiTreeTextRoleBaselineTypography> {
        self.heading_2_baseline
    }

    pub(in crate::raster_host) const fn heading_3(self) -> Option<UiTreeTextRoleTypography> {
        self.heading_3
    }

    pub(in crate::raster_host) const fn heading_3_baseline(
        self,
    ) -> Option<UiTreeTextRoleBaselineTypography> {
        self.heading_3_baseline
    }

    pub(in crate::raster_host) const fn heading_4(self) -> Option<UiTreeTextRoleTypography> {
        self.heading_4
    }

    pub(in crate::raster_host) const fn heading_4_baseline(
        self,
    ) -> Option<UiTreeTextRoleBaselineTypography> {
        self.heading_4_baseline
    }

    pub(in crate::raster_host) const fn heading_5(self) -> Option<UiTreeTextRoleTypography> {
        self.heading_5
    }

    pub(in crate::raster_host) const fn heading_5_baseline(
        self,
    ) -> Option<UiTreeTextRoleBaselineTypography> {
        self.heading_5_baseline
    }

    pub(in crate::raster_host) const fn heading_6(self) -> Option<UiTreeTextRoleTypography> {
        self.heading_6
    }

    pub(in crate::raster_host) const fn heading_6_baseline(
        self,
    ) -> Option<UiTreeTextRoleBaselineTypography> {
        self.heading_6_baseline
    }
}

#[cfg(test)]
#[path = "types_tests.rs"]
mod tests;

mod builders;
