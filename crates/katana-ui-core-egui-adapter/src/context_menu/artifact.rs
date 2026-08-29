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
        frame_record_hash: hash(&record)?,
        paint_plan_hash: hash(&paint_plan)?,
        record,
        paint_plan,
        events,
    })
}

fn hash(value: &impl Serialize) -> Result<String, ContextMenuAdapterError> {
    serde_json::to_vec(value)
        .map(|bytes| hex::encode(Sha256::digest(bytes)))
        .map_err(|error| ContextMenuAdapterError::ArtifactSerialization(error.to_string()))
}
