use super::{ArtifactCompositeError, ArtifactPaintPlanRef};
use serde::Serialize;
use sha2::{Digest, Sha256};

pub(super) fn paint_plan_hash(
    plans: &[ArtifactPaintPlanRef<'_>],
) -> Result<String, ArtifactCompositeError> {
    if let [plan] = plans {
        return serialized_plan(plan).map(|bytes| hash_bytes(&bytes));
    }
    let mut hasher = Sha256::new();
    for plan in plans {
        hasher.update(plan_kind(plan));
        hasher.update(serialized_plan(plan)?);
    }
    Ok(hex::encode(hasher.finalize()))
}

pub(super) fn hash_bytes(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn serialized_plan(plan: &ArtifactPaintPlanRef<'_>) -> Result<Vec<u8>, ArtifactCompositeError> {
    match plan {
        ArtifactPaintPlanRef::TextSurface(value) => serialize_value(value),
        ArtifactPaintPlanRef::SourceAddress(value) => serialize_value(value),
        ArtifactPaintPlanRef::StatusBar(value) => serialize_value(value),
        ArtifactPaintPlanRef::DiagnosticsList(value) => serialize_value(value),
        ArtifactPaintPlanRef::TabStrip(value) => serialize_value(value),
        ArtifactPaintPlanRef::CommandChrome(value) => serialize_value(value),
        ArtifactPaintPlanRef::ContextMenu(value) => serialize_value(value),
    }
}

pub(super) fn serialize_value(value: &impl Serialize) -> Result<Vec<u8>, ArtifactCompositeError> {
    serde_json::to_vec(value)
        .map_err(|error| ArtifactCompositeError::Serialization(error.to_string()))
}

const fn plan_kind(plan: &ArtifactPaintPlanRef<'_>) -> &'static [u8] {
    match plan {
        ArtifactPaintPlanRef::TextSurface(_) => b"text",
        ArtifactPaintPlanRef::SourceAddress(_) => b"source-address",
        ArtifactPaintPlanRef::StatusBar(_) => b"status-bar",
        ArtifactPaintPlanRef::DiagnosticsList(_) => b"diagnostics-list",
        ArtifactPaintPlanRef::TabStrip(_) => b"tab-strip",
        ArtifactPaintPlanRef::CommandChrome(_) => b"chrome",
        ArtifactPaintPlanRef::ContextMenu(_) => b"context-menu",
    }
}
