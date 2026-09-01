//! Generic KUC diagnostics-list surface.

mod accessibility;
mod adapter;
mod identity;
mod paint;
mod types;

pub use adapter::EguiDiagnosticsListAdapter;
pub use identity::DiagnosticsTargetIdentity;
pub use types::{
    DiagnosticsListPaintOperation, DiagnosticsListPaintOperationKind, DiagnosticsListPaintPlan,
    DiagnosticsListPaintTexture, DiagnosticsListRasterEvidence, EguiDiagnosticsListError,
    EguiDiagnosticsListOutput,
};
