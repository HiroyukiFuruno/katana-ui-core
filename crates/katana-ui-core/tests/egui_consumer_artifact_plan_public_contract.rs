#![cfg(feature = "egui")]
#![cfg(feature = "storybook-artifacts")]

use katana_ui_core::egui::text_command_surface::{
    ConsumerArtifactLeafId, ConsumerArtifactPlanError, EguiTextCommandSurfaceHostProjectionEncoder,
    EguiTextCommandSurfacePresentation, EguiTextCommandSurfacePresentationToken,
    TextCommandSurfaceStyle,
};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

fn host_projected_token(revision: u64) -> EguiTextCommandSurfacePresentationToken {
    let surface = TextSurface::new(TextSurfaceProps::new(
        katana_ui_core::atom::TextArea::new("consumer-artifact-plan")
            .value("consumer artifact plan"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 320, 180),
    ));
    EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"host-owned-target",
        EguiTextCommandSurfacePresentation {
            text_state_id: None,
            text: TextSurfacePresentation::from_props(surface.props()),
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
        },
        TextCommandSurfaceStyle::standard().expect("standard style"),
    )
    .expect("opaque token")
}

mod foreign_consumer {
    use katana_ui_core::egui::text_command_surface::{
        ConsumerArtifactLeafId, ConsumerArtifactPlanError, ConsumerArtifactPlanIssuer,
        ConsumerArtifactPlanV1, ConsumerArtifactStageBinding,
        EguiTextCommandSurfacePresentationToken, GenericEffectClass, GenericInteractionClass,
    };

    pub(super) fn issue_stage_count(
        token: EguiTextCommandSurfacePresentationToken,
    ) -> Result<usize, ConsumerArtifactPlanError> {
        let plan = ConsumerArtifactPlanV1::new(
            1,
            vec![ConsumerArtifactStageBinding::new(
                ConsumerArtifactLeafId::new("source-derived-leaf")?,
                GenericInteractionClass::ImeCommit,
                GenericEffectClass::OpaqueForwarding,
                token,
            )],
        );
        ConsumerArtifactPlanIssuer::new()
            .issue(plan)
            .map(|issued| issued.remaining_stage_count())
    }
}

#[test]
fn foreign_consumer_is_rejected_before_opaque_forwarding_can_drop_events() {
    assert!(matches!(
        foreign_consumer::issue_stage_count(host_projected_token(1)),
        Err(ConsumerArtifactPlanError::UnsupportedEffectClass(
            katana_ui_core::egui::text_command_surface::GenericEffectClass::OpaqueForwarding
        ))
    ));
}

#[test]
fn consumer_plan_rejects_invalid_leaf_identifier() {
    assert!(matches!(
        ConsumerArtifactLeafId::new("\0"),
        Err(ConsumerArtifactPlanError::InvalidLeafId)
    ));
}
