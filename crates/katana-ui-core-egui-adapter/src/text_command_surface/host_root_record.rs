use super::super::root::EguiTextCommandSurfaceRootOutput;

/// Closed root record returned by the consumer-safe facade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EguiTextCommandSurfaceHostRootRecord {
    identity: String,
    presentation_revision: u64,
    state_revision: u64,
    dimensions: EguiTextCommandSurfaceHostRootRecordDimensions,
    rgba_hash: String,
    paint_plan_hash: String,
    record_hash: String,
    accessibility_snapshot_hash: String,
}

impl EguiTextCommandSurfaceHostRootRecord {
    pub(super) fn from_output(
        identity: &str,
        presentation_revision: u64,
        output: &EguiTextCommandSurfaceRootOutput,
    ) -> Self {
        let frame = output.frame();
        let dimensions = frame.dimensions();
        Self {
            identity: identity.to_owned(),
            presentation_revision,
            state_revision: frame.state_revision(),
            dimensions: EguiTextCommandSurfaceHostRootRecordDimensions {
                width: dimensions.width(),
                height: dimensions.height(),
            },
            rgba_hash: frame.rgba_hash().to_owned(),
            paint_plan_hash: frame.paint_plan_hash().to_owned(),
            record_hash: frame.record_hash().to_owned(),
            accessibility_snapshot_hash: frame.accessibility().snapshot_hash().to_owned(),
        }
    }

    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    #[must_use]
    pub const fn presentation_revision(&self) -> u64 {
        self.presentation_revision
    }

    #[must_use]
    pub const fn state_revision(&self) -> u64 {
        self.state_revision
    }

    #[must_use]
    pub const fn dimensions(&self) -> EguiTextCommandSurfaceHostRootRecordDimensions {
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
    pub fn accessibility_snapshot_hash(&self) -> &str {
        &self.accessibility_snapshot_hash
    }
}

/// Root dimensions without child geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EguiTextCommandSurfaceHostRootRecordDimensions {
    width: u32,
    height: u32,
}

impl EguiTextCommandSurfaceHostRootRecordDimensions {
    #[must_use]
    pub const fn width(self) -> u32 {
        self.width
    }

    #[must_use]
    pub const fn height(self) -> u32 {
        self.height
    }
}
