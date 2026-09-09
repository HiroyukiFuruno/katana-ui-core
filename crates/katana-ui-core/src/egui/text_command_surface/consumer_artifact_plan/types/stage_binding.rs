use super::{ConsumerArtifactLeafId, GenericEffectClass, GenericInteractionClass};
use crate::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionLease, EguiTextCommandSurfacePresentationToken,
};

/// One plan binding. Its presentation token remains opaque and is consumed by KUC.
pub struct ConsumerArtifactStageBinding {
    pub(super) leaf: ConsumerArtifactLeafId,
    pub(super) interaction: GenericInteractionClass,
    pub(super) effect: GenericEffectClass,
    pub(super) token: Option<EguiTextCommandSurfacePresentationToken>,
    action_target: Option<String>,
}

impl ConsumerArtifactStageBinding {
    /// Consumes one KUC-issued opaque lease without exposing its token or host router.
    #[must_use]
    pub fn from_host_projection_lease(
        leaf: ConsumerArtifactLeafId,
        interaction: GenericInteractionClass,
        effect: GenericEffectClass,
        lease: EguiTextCommandSurfaceHostProjectionLease,
    ) -> Self {
        let token = lease.into_consumer_artifact_token();
        Self::new(leaf, interaction, effect, token)
    }

    #[must_use]
    pub fn new(
        leaf: ConsumerArtifactLeafId,
        interaction: GenericInteractionClass,
        effect: GenericEffectClass,
        token: EguiTextCommandSurfacePresentationToken,
    ) -> Self {
        let action_target = leaf.as_str().to_owned();
        Self::new_with_action_target(leaf, action_target, interaction, effect, token)
    }

    #[must_use]
    pub fn new_with_action_target(
        leaf: ConsumerArtifactLeafId,
        action_target: impl Into<String>,
        interaction: GenericInteractionClass,
        effect: GenericEffectClass,
        token: EguiTextCommandSurfacePresentationToken,
    ) -> Self {
        Self {
            leaf,
            interaction,
            effect,
            token: Some(token),
            action_target: Some(action_target.into()),
        }
    }

    #[must_use]
    pub(super) fn action_target(&self) -> &str {
        self.action_target
            .as_ref()
            .map_or(self.leaf.as_str(), String::as_str)
    }
}

impl std::fmt::Debug for ConsumerArtifactStageBinding {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ConsumerArtifactStageBinding")
            .field("leaf", &self.leaf)
            .field("interaction", &self.interaction)
            .field("effect", &self.effect)
            .finish_non_exhaustive()
    }
}
