//! Closed frame projection for the retained root.

use super::super::types::EguiTextCommandSurfaceOutput;
use crate::artifact_compositor::ArtifactCompositeFrame;
use serde::Serialize;
use sha2::{Digest, Sha256};

const ROOT_CHILD_RECORD_SLOT_COUNT: usize = 7;

/// Root dimensions without child coordinates or geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EguiTextCommandSurfaceRootDimensions {
    width: u32,
    height: u32,
}

impl EguiTextCommandSurfaceRootDimensions {
    #[must_use]
    pub const fn width(self) -> u32 {
        self.width
    }

    #[must_use]
    pub const fn height(self) -> u32 {
        self.height
    }
}

/// Opaque reference to the KUC-produced AccessKit projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EguiTextCommandSurfaceRootAccessKitReference {
    snapshot_hash: String,
}

impl EguiTextCommandSurfaceRootAccessKitReference {
    #[must_use]
    pub fn snapshot_hash(&self) -> &str {
        &self.snapshot_hash
    }
}

/// Closed retained root frame. Child artifacts and paint plans remain KUC-private.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EguiTextCommandSurfaceRootFrame {
    identity: String,
    state_revision: u64,
    dimensions: EguiTextCommandSurfaceRootDimensions,
    rgba_hash: String,
    paint_plan_hash: String,
    record_hash: String,
    accessibility: EguiTextCommandSurfaceRootAccessKitReference,
}

impl EguiTextCommandSurfaceRootFrame {
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    #[must_use]
    pub const fn state_revision(&self) -> u64 {
        self.state_revision
    }

    #[must_use]
    pub const fn dimensions(&self) -> EguiTextCommandSurfaceRootDimensions {
        self.dimensions
    }

    #[must_use]
    pub fn rgba_hash(&self) -> &str {
        &self.rgba_hash
    }

    #[must_use]
    pub fn paint_plan_hash(&self) -> &str {
        &self.paint_plan_hash
    }

    #[must_use]
    pub fn record_hash(&self) -> &str {
        &self.record_hash
    }

    #[must_use]
    pub const fn accessibility(&self) -> &EguiTextCommandSurfaceRootAccessKitReference {
        &self.accessibility
    }
}

#[derive(Serialize)]
struct RootRecordMaterial<'a> {
    identity: &'a str,
    state_revision: u64,
    dimensions: EguiTextCommandSurfaceRootDimensionsWire,
    rgba_hash: &'a str,
    paint_plan_hash: &'a str,
    child_record_hashes: [Option<&'a str>; ROOT_CHILD_RECORD_SLOT_COUNT],
    accessibility_hash: &'a str,
}

#[derive(Serialize)]
struct EguiTextCommandSurfaceRootDimensionsWire {
    width: u32,
    height: u32,
}

pub(super) fn build_frame(
    identity: &str,
    state_revision: u64,
    output: &EguiTextCommandSurfaceOutput,
    composite: &ArtifactCompositeFrame,
) -> Result<EguiTextCommandSurfaceRootFrame, String> {
    let accessibility_hash =
        super::super::accesskit_evidence::snapshot_hash(&output.accesskit_evidence)?;
    let bounds = composite.canvas.ui_rect();
    let dimensions = EguiTextCommandSurfaceRootDimensions {
        width: bounds.width,
        height: bounds.height,
    };
    let status_bar_hash = output
        .status_bar
        .as_ref()
        .map(|value| hash_serialized(&value.paint_plan))
        .transpose()?;
    let diagnostics_list_hash = output
        .diagnostics_list
        .as_ref()
        .map(|value| hash_serialized(&value.paint_plan))
        .transpose()?;
    let material = RootRecordMaterial {
        identity,
        state_revision,
        dimensions: EguiTextCommandSurfaceRootDimensionsWire {
            width: dimensions.width,
            height: dimensions.height,
        },
        rgba_hash: &composite.pixel_hash,
        paint_plan_hash: &composite.paint_plan_hash,
        child_record_hashes: [
            Some(output.text.artifact.frame_record_hash.as_str()),
            output
                .toolbar
                .as_ref()
                .map(|value| value.artifact.frame_record_hash.as_str()),
            output
                .search
                .as_ref()
                .map(|value| value.artifact.frame_record_hash.as_str()),
            output.floating.as_ref().and_then(|value| {
                value
                    .artifact
                    .as_ref()
                    .map(|artifact| artifact.frame_record_hash.as_str())
            }),
            output.context_menu.as_ref().and_then(|value| {
                value
                    .artifact
                    .as_ref()
                    .map(|artifact| artifact.frame_record_hash.as_str())
            }),
            status_bar_hash.as_deref(),
            diagnostics_list_hash.as_deref(),
        ],
        accessibility_hash: &accessibility_hash,
    };
    let record_hash = hash_serialized(&material)?;
    Ok(EguiTextCommandSurfaceRootFrame {
        identity: identity.to_string(),
        state_revision,
        dimensions,
        rgba_hash: composite.pixel_hash.clone(),
        paint_plan_hash: composite.paint_plan_hash.clone(),
        record_hash,
        accessibility: EguiTextCommandSurfaceRootAccessKitReference {
            snapshot_hash: accessibility_hash,
        },
    })
}

fn hash_serialized(value: &impl Serialize) -> Result<String, String> {
    serde_json::to_vec(value)
        .map(|bytes| format!("{:x}", Sha256::digest(bytes)))
        .map_err(|error| error.to_string())
}
