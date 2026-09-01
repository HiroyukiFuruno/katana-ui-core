//! Opaque KUC-owned projection lease for generic status and diagnostics children.

/// A one-shot, non-wire projection consumed by the retained KUC root.
///
/// The child models stay private to the lease consumer. Host facades expose
/// only the lease and the closed root frame, never these models or their
/// typed events.
pub struct StatusDiagnosticsProjectionLease {
    pub(super) status_bar: Option<crate::molecule::StatusBar>,
    pub(super) diagnostics_list: Option<crate::molecule::DiagnosticsList>,
}

impl StatusDiagnosticsProjectionLease {
    #[must_use]
    pub fn new() -> Self {
        Self {
            status_bar: None,
            diagnostics_list: None,
        }
    }

    #[must_use]
    pub fn with_status_bar(mut self, status_bar: crate::molecule::StatusBar) -> Self {
        self.status_bar = Some(status_bar);
        self
    }

    #[must_use]
    pub fn with_diagnostics_list(
        mut self,
        diagnostics_list: crate::molecule::DiagnosticsList,
    ) -> Self {
        self.diagnostics_list = Some(diagnostics_list);
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        Option<crate::molecule::StatusBar>,
        Option<crate::molecule::DiagnosticsList>,
    ) {
        (self.status_bar, self.diagnostics_list)
    }
}

impl Default for StatusDiagnosticsProjectionLease {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for StatusDiagnosticsProjectionLease {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("StatusDiagnosticsProjectionLease(..)")
    }
}
