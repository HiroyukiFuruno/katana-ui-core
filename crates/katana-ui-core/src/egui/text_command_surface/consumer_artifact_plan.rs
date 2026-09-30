//! Consumer-defined full-editor artifacts without consumer-owned input or rendering.

mod types;

pub use types::{
    ConsumerArtifactEvidence, ConsumerArtifactForwardingReceipt, ConsumerArtifactLeafId,
    ConsumerArtifactPlanError, ConsumerArtifactPlanExecutionError, ConsumerArtifactPlanIssuer,
    ConsumerArtifactPlanV1, ConsumerArtifactPlanV2, ConsumerArtifactStageBinding,
    GenericEffectClass, GenericInteractionClass, IssuedConsumerArtifactPlan,
};
