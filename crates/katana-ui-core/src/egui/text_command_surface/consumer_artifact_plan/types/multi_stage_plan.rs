use super::{ConsumerArtifactStageBinding, MULTI_STAGE_SCHEMA_VERSION};

/// Versioned input for a generic variable-length consumer artifact plan.
///
/// Unlike v1, this plan does not prescribe a fixed interaction sequence. Every
/// binding still uses only KUC-defined interactions and shares one retained root.
pub struct ConsumerArtifactPlanV2 {
    pub(super) schema_version: u16,
    pub(super) initial_revision: u64,
    pub(super) bindings: Vec<ConsumerArtifactStageBinding>,
}

impl ConsumerArtifactPlanV2 {
    #[must_use]
    pub fn new(initial_revision: u64, bindings: Vec<ConsumerArtifactStageBinding>) -> Self {
        Self {
            schema_version: MULTI_STAGE_SCHEMA_VERSION,
            initial_revision,
            bindings,
        }
    }

    /// Allows a consumer to declare its wire schema explicitly; unsupported values fail at issue.
    #[must_use]
    pub fn with_schema_version(
        schema_version: u16,
        initial_revision: u64,
        bindings: Vec<ConsumerArtifactStageBinding>,
    ) -> Self {
        Self {
            schema_version,
            initial_revision,
            bindings,
        }
    }
}
