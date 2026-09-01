//! Stable opaque target identities for the generic diagnostics surface.

use crate::molecule::DiagnosticSeverity;
use sha2::{Digest, Sha256};

const TARGET_IDENTITY_DOMAIN: &str = "kuc.diagnostics.target.v1";

/// Opaque identities used by diagnostics interactions.
pub struct DiagnosticsTargetIdentity;

impl DiagnosticsTargetIdentity {
    #[must_use]
    pub fn severity_filter(severity: DiagnosticSeverity) -> String {
        opaque_target_identity("severity-filter", severity_key(severity))
    }

    #[must_use]
    pub fn item(item_id: &str) -> String {
        opaque_target_identity("item", item_id)
    }

    #[must_use]
    pub fn fix(item_id: &str) -> String {
        opaque_target_identity("fix", item_id)
    }

    #[must_use]
    pub fn disclosure(item_id: &str) -> String {
        opaque_target_identity("disclosure", item_id)
    }

    #[must_use]
    pub fn scope(scope_key: &str) -> String {
        opaque_target_identity("scope", scope_key)
    }
}

fn severity_key(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Error => "error",
        DiagnosticSeverity::Warning => "warning",
        DiagnosticSeverity::Info => "info",
        DiagnosticSeverity::Hint => "hint",
    }
}

fn opaque_target_identity(kind: &str, stable_key: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(TARGET_IDENTITY_DOMAIN.as_bytes());
    digest.update([0]);
    digest.update(kind.as_bytes());
    digest.update([0]);
    digest.update(stable_key.as_bytes());
    format!(
        "{TARGET_IDENTITY_DOMAIN}.{kind}.{}",
        hex::encode(digest.finalize())
    )
}
