use super::ContextMenuAdapterError;
use super::types::{ContextMenuArtifactFrame, ContextMenuPaintPlan, EguiContextMenuFrameRecord};
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
    let bytes = serde_json::to_vec(value)
        .map_err(|error| ContextMenuAdapterError::ArtifactSerialization(error.to_string()))?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame_record() -> EguiContextMenuFrameRecord {
        EguiContextMenuFrameRecord {
            bounds: katana_ui_core::render_model::UiRect::new(0, 0, 10, 10),
            viewport_bounds: katana_ui_core::render_model::UiRect::new(0, 0, 10, 20),
            highlighted_path: vec![0],
            focused: false,
            items: Vec::new(),
        }
    }

    #[test]
    fn artifact_frame_wraps_record_and_events_with_deterministic_hashes()
    -> Result<(), ContextMenuAdapterError> {
        let record = frame_record();
        let plan = ContextMenuPaintPlan {
            surface_bounds: katana_ui_core::render_model::UiRect::new(0, 0, 10, 10),
            operations: Vec::new(),
        };
        let events = Vec::new();
        let artifact = artifact_frame(record.clone(), plan.clone(), events.clone())?;
        assert_eq!(artifact.record, record);
        assert_eq!(artifact.paint_plan, plan);
        assert_eq!(artifact.events, events);
        assert!(!artifact.frame_record_hash.is_empty());
        assert!(!artifact.paint_plan_hash.is_empty());
        assert_ne!(artifact.frame_record_hash, artifact.paint_plan_hash);
        Ok(())
    }

    struct SerializeFail;

    impl Serialize for SerializeFail {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(serde::ser::Error::custom("closed serialization failure"))
        }
    }

    #[test]
    fn hash_maps_serialization_failures_to_the_typed_adapter_error() {
        assert!(matches!(
            hash(&SerializeFail),
            Err(ContextMenuAdapterError::ArtifactSerialization(_))
        ));
    }
}
