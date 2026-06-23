use super::DiagnosticId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticsListEvent {
    DiagnosticSelected {
        id: DiagnosticId,
    },
    DiagnosticFixPreviewToggled {
        id: DiagnosticId,
        expanded: bool,
    },
    DiagnosticFixApplied {
        id: DiagnosticId,
    },
    NavigateRequested {
        id: DiagnosticId,
    },
    BulkFixPreviewOpened,
    BulkFixApplied {
        applied_ids: Vec<DiagnosticId>,
        skipped_ids: Vec<(DiagnosticId, BulkFixSkipReason)>,
    },
    FilterChanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BulkFixSkipReason {
    FilteredOut,
    NoQuickfix,
}
