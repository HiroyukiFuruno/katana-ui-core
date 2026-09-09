//! Consumer-defined full-editor artifacts without consumer-owned input or rendering.

mod types;

pub use types::{
    ConsumerArtifactEvidence, ConsumerArtifactForwardingReceipt, ConsumerArtifactLeafId,
    ConsumerArtifactPlanError, ConsumerArtifactPlanIssuer, ConsumerArtifactPlanV1,
    ConsumerArtifactStageBinding, GenericEffectClass, GenericInteractionClass,
    IssuedConsumerArtifactPlan,
};
