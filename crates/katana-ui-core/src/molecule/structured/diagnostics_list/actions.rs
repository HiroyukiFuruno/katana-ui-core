use super::{DiagnosticId, DiagnosticSeverity, DiagnosticsGroupBy, DiagnosticsSortBy};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticsListAction {
    SetGroupBy(DiagnosticsGroupBy),
    SetSortBy(DiagnosticsSortBy),
    SetSeverityFilter(BTreeSet<DiagnosticSeverity>),
    Select(DiagnosticId),
    ToggleFixPreview(DiagnosticId),
    ApplyFix(DiagnosticId),
    OpenBulkPreview,
    ConfirmBulkApply,
    Keyboard(DiagnosticKeyboardInput),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticKeyboardInput {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Enter,
    Space,
    F8,
    ShiftF8,
}
