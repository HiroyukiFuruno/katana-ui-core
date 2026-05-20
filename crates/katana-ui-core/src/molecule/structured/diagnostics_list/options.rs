use super::DiagnosticSeverity;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticsGroupBy {
    Severity,
    Source,
    Location,
    #[default]
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticsSortBy {
    Severity,
    Location,
    Source,
    #[default]
    Order,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsListOptions {
    pub group_by: DiagnosticsGroupBy,
    pub sort_by: DiagnosticsSortBy,
    pub severity_filter: BTreeSet<DiagnosticSeverity>,
    pub wrap_error_navigation: bool,
}

impl Default for DiagnosticsListOptions {
    fn default() -> Self {
        Self {
            group_by: DiagnosticsGroupBy::None,
            sort_by: DiagnosticsSortBy::Order,
            severity_filter: DiagnosticSeverity::all().into_iter().collect(),
            wrap_error_navigation: true,
        }
    }
}
