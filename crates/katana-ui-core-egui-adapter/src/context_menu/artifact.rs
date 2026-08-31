use super::types::{
    ContextMenuAdapterError, ContextMenuArtifactFrame, ContextMenuPaintPlan,
    EguiContextMenuFrameRecord,
};
use katana_ui_core::molecule::selection::ContextMenuEvent;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub(super) fn artifact_frame(
    record: EguiContextMenuFrameRecord,
    paint_plan: ContextMenuPaintPlan,
    events: Vec<ContextMenuEvent>,
) -> Result<ContextMenuArtifactFrame, ContextMenuAdapterError> {
    Ok(ContextMenuArtifactFrame {
        frame_record_hash: artifact_hash(&record)?,
        paint_plan_hash: artifact_hash(&paint_plan)?,
        record,
        paint_plan,
        events,
    })
}

pub(super) fn artifact_hash(value: &impl Serialize) -> Result<String, ContextMenuAdapterError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| ContextMenuAdapterError::ArtifactSerialization(error.to_string()))?;
    Ok(hex::encode(Sha256::digest(bytes)))
}
