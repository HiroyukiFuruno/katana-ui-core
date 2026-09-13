use super::{UiTreeDocumentTypography, UiTreeTextRoleBaselineTypography, UiTreeTextRoleTypography};

impl UiTreeDocumentTypography {
    /// Creates an override set that preserves all theme-derived metrics.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            body: None,
            body_baseline: None,
            heading_1: None,
            heading_1_baseline: None,
            heading_2: None,
            heading_2_baseline: None,
            heading_3: None,
            heading_3_baseline: None,
            heading_4: None,
            heading_4_baseline: None,
            heading_5: None,
            heading_5_baseline: None,
            heading_6: None,
            heading_6_baseline: None,
        }
    }

    #[must_use]
    pub const fn with_body(mut self, typography: UiTreeTextRoleTypography) -> Self {
        self.body = Some(typography);
        self.body_baseline = None;
        self
    }

    #[must_use]
    pub const fn with_body_baseline(
        mut self,
        typography: UiTreeTextRoleBaselineTypography,
    ) -> Self {
        self.body_baseline = Some(typography);
        self.body = None;
        self
    }

    /// Overrides first-level document heading metrics.
    #[must_use]
    pub const fn with_heading_1(mut self, typography: UiTreeTextRoleTypography) -> Self {
        self.heading_1 = Some(typography);
        self.heading_1_baseline = None;
        self
    }

    #[must_use]
    pub const fn with_heading_1_baseline(
        mut self,
        typography: UiTreeTextRoleBaselineTypography,
    ) -> Self {
        self.heading_1_baseline = Some(typography);
        self.heading_1 = None;
        self
    }

    /// Overrides second-level document heading metrics.
    #[must_use]
    pub const fn with_heading_2(mut self, typography: UiTreeTextRoleTypography) -> Self {
        self.heading_2 = Some(typography);
        self.heading_2_baseline = None;
        self
    }

    #[must_use]
    pub const fn with_heading_2_baseline(
        mut self,
        typography: UiTreeTextRoleBaselineTypography,
    ) -> Self {
        self.heading_2_baseline = Some(typography);
        self.heading_2 = None;
        self
    }

    /// Overrides third-level document heading metrics.
    #[must_use]
    pub const fn with_heading_3(mut self, typography: UiTreeTextRoleTypography) -> Self {
        self.heading_3 = Some(typography);
        self.heading_3_baseline = None;
        self
    }

    #[must_use]
    pub const fn with_heading_3_baseline(
        mut self,
        typography: UiTreeTextRoleBaselineTypography,
    ) -> Self {
        self.heading_3_baseline = Some(typography);
        self.heading_3 = None;
        self
    }

    /// Overrides fourth-level document heading metrics.
    #[must_use]
    pub const fn with_heading_4(mut self, typography: UiTreeTextRoleTypography) -> Self {
        self.heading_4 = Some(typography);
        self.heading_4_baseline = None;
        self
    }

    #[must_use]
    pub const fn with_heading_4_baseline(
        mut self,
        typography: UiTreeTextRoleBaselineTypography,
    ) -> Self {
        self.heading_4_baseline = Some(typography);
        self.heading_4 = None;
        self
    }

    /// Overrides fifth-level document heading metrics.
    #[must_use]
    pub const fn with_heading_5(mut self, typography: UiTreeTextRoleTypography) -> Self {
        self.heading_5 = Some(typography);
        self.heading_5_baseline = None;
        self
    }

    #[must_use]
    pub const fn with_heading_5_baseline(
        mut self,
        typography: UiTreeTextRoleBaselineTypography,
    ) -> Self {
        self.heading_5_baseline = Some(typography);
        self.heading_5 = None;
        self
    }

    /// Overrides sixth-level document heading metrics.
    #[must_use]
    pub const fn with_heading_6(mut self, typography: UiTreeTextRoleTypography) -> Self {
        self.heading_6 = Some(typography);
        self.heading_6_baseline = None;
        self
    }

    #[must_use]
    pub const fn with_heading_6_baseline(
        mut self,
        typography: UiTreeTextRoleBaselineTypography,
    ) -> Self {
        self.heading_6_baseline = Some(typography);
        self.heading_6 = None;
        self
    }
}
