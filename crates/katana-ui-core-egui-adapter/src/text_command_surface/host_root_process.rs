use super::super::root::KucRootEffectRouter;
use super::super::root::{EguiTextCommandSurfaceRoot, EguiTextCommandSurfaceRootOutput};
use super::host_root_surface::surface_from_presentation;
use super::{EguiTextCommandSurfaceRootFactoryError, TextCommandSurfaceStyle};
use crate::text_command_surface::host_root::host_root_token_codec::DecodedRootPresentation;
use crate::text_command_surface::root::KucOpaqueHostEffectAttachError;

fn map_opaque_effect_attach_error(
    _error: KucOpaqueHostEffectAttachError,
) -> EguiTextCommandSurfaceRootFactoryError {
    EguiTextCommandSurfaceRootFactoryError::OpaqueHostEffectRejected
}

pub(crate) struct HostRootProcess {
    root: EguiTextCommandSurfaceRoot,
    identity: String,
    style: TextCommandSurfaceStyle,
    presentation: super::super::types::EguiTextCommandSurfacePresentation,
    command_families: Option<super::EguiTextCommandSurfaceCommandFamilyProjection>,
    presentation_revision: u64,
    effect_router: Option<Box<dyn KucRootEffectRouter>>,
}

impl HostRootProcess {
    pub(super) fn retain(
        decoded: DecodedRootPresentation,
        presentation_revision: u64,
    ) -> Result<Self, EguiTextCommandSurfaceRootFactoryError> {
        if decoded.identity.trim().is_empty() {
            return Err(EguiTextCommandSurfaceRootFactoryError::InvalidToken(
                "host target identity is empty",
            ));
        }
        let surface = surface_from_presentation(
            &decoded.identity,
            &decoded.presentation,
            decoded.command_families.as_ref(),
        );
        let mut root = EguiTextCommandSurfaceRoot::with_identity(decoded.identity.clone(), surface);
        let _ = root.synchronize_presentation(decoded.presentation.clone());
        Ok(Self {
            root,
            identity: decoded.identity,
            style: decoded.style,
            presentation: decoded.presentation,
            command_families: decoded.command_families,
            presentation_revision,
            effect_router: None,
        })
    }

    pub(super) fn retain_with_router(
        decoded: DecodedRootPresentation,
        presentation_revision: u64,
        router: Box<dyn KucRootEffectRouter>,
    ) -> Result<Self, EguiTextCommandSurfaceRootFactoryError> {
        let mut process = Self::retain(decoded, presentation_revision)?;
        process.effect_router = Some(router);
        Ok(process)
    }

    pub(super) fn synchronize(
        &mut self,
        revision: u64,
        decoded: DecodedRootPresentation,
    ) -> Result<bool, EguiTextCommandSurfaceRootFactoryError> {
        if decoded.identity != self.identity {
            return Err(EguiTextCommandSurfaceRootFactoryError::IdentityChanged);
        }
        if revision < self.presentation_revision {
            return Err(EguiTextCommandSurfaceRootFactoryError::StaleRevision {
                current: self.presentation_revision,
                received: revision,
            });
        }
        if revision == self.presentation_revision {
            if decoded.style != self.style
                || decoded.presentation != self.presentation
                || decoded.command_families != self.command_families
            {
                return Err(EguiTextCommandSurfaceRootFactoryError::RevisionConflict { revision });
            }
            return Ok(false);
        }
        let changed = self
            .root
            .synchronize_presentation(decoded.presentation.clone());
        if let Some(command_families) = decoded.command_families.as_ref() {
            self.root.apply_command_family_projection(command_families);
        }
        self.style = decoded.style;
        self.presentation = decoded.presentation;
        self.command_families = decoded.command_families;
        self.presentation_revision = revision;
        Ok(changed)
    }

    pub(super) fn synchronize_with_router(
        &mut self,
        revision: u64,
        decoded: DecodedRootPresentation,
        router: Box<dyn KucRootEffectRouter>,
    ) -> Result<bool, EguiTextCommandSurfaceRootFactoryError> {
        if revision <= self.presentation_revision {
            return Err(EguiTextCommandSurfaceRootFactoryError::DuplicateLease { revision });
        }
        let changed = self.synchronize(revision, decoded)?;
        self.effect_router = Some(router);
        Ok(changed)
    }

    pub(super) fn show(
        &mut self,
        ui: &mut egui::Ui,
    ) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootFactoryError> {
        let output = self
            .root
            .show(ui, &self.style)
            .map_err(|error| EguiTextCommandSurfaceRootFactoryError::Root(error.to_string()))?;
        if let Some(router) = self.effect_router.as_mut() {
            let effect = router
                .route(output.events().current_context())
                .map_err(|_| EguiTextCommandSurfaceRootFactoryError::OpaqueHostEffect)?;
            if let Some(effect) = effect {
                output
                    .events()
                    .attach_opaque_host_effect_batch(effect)
                    .map_err(map_opaque_effect_attach_error)?;
            }
        }
        Ok(output)
    }

    pub(super) fn identity(&self) -> &str {
        &self.identity
    }

    pub(super) const fn presentation_revision(&self) -> u64 {
        self.presentation_revision
    }
}

#[cfg(test)]
mod tests {
    use super::EguiTextCommandSurfaceRootFactoryError;
    use super::EguiTextCommandSurfaceRootFactoryError::*;
    use super::HostRootProcess;
    use super::map_opaque_effect_attach_error;
    use crate::text_command_surface::host_root::host_root_token_codec::DecodedRootPresentation;
    use crate::text_command_surface::{
        EguiTextCommandSurfaceCommandFamilyProjection, KucOpaqueHostEffectBatch,
        KucOpaqueHostEffectError, KucRootEventBatchContext, TextCommandSurfaceStyle,
    };
    use katana_ui_core::atom::TextArea;
    use katana_ui_core::molecule::command_chrome::CommandChromeAction;
    use katana_ui_core::molecule::command_chrome::CommandChromeToolbarPresentation;
    use katana_ui_core::render_model::UiStateId;
    use katana_ui_core::text_surface::{
        TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
    };
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn opaque_effect_attach_failures_map_to_the_closed_factory_error() {
        use crate::text_command_surface::root::KucOpaqueHostEffectAttachError;

        for error in [
            KucOpaqueHostEffectAttachError::AlreadyConsumed,
            KucOpaqueHostEffectAttachError::AlreadyAttached,
        ] {
            assert!(matches!(
                map_opaque_effect_attach_error(error),
                EguiTextCommandSurfaceRootFactoryError::OpaqueHostEffectRejected
            ));
        }
    }

    fn minimal_presentation() -> super::super::super::types::EguiTextCommandSurfacePresentation {
        let text_surface = TextSurface::new(
            TextSurfaceProps::new(
                TextArea::new("host-root-process").value("hello"),
                Vec::new(),
                TextSurfaceViewport::new(0, 0, 1, 1),
            )
            .adapter_measured_viewport(),
        );
        super::super::super::types::EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("host-root-process")),
            text: TextSurfacePresentation::from_props(text_surface.props()),
            toolbar: Some(CommandChromeToolbarPresentation {
                actions: vec![CommandChromeAction::new("action", "Action")],
                groups: Vec::new(),
                display_mode: Default::default(),
                density: Default::default(),
                overflow_strategy: Default::default(),
            }),
            floating: None,
            search: None,
            context_menu: None,
        }
    }

    fn decoded_presentation(identity: &str) -> DecodedRootPresentation {
        DecodedRootPresentation {
            identity: identity.to_owned(),
            presentation: minimal_presentation(),
            style: TextCommandSurfaceStyle::standard(),
            command_families: Some(EguiTextCommandSurfaceCommandFamilyProjection::new(
                Some(
                    katana_ui_core::molecule::command_chrome::CommandChromeFamilyId::new("primary"),
                ),
                Some(
                    katana_ui_core::molecule::command_chrome::CommandChromeFamilyId::new(
                        "floating",
                    ),
                ),
            )),
        }
    }

    fn no_effect(
        _context: KucRootEventBatchContext,
    ) -> Result<Option<KucOpaqueHostEffectBatch>, KucOpaqueHostEffectError> {
        Ok(None)
    }

    fn successful_effect() -> Result<(), KucOpaqueHostEffectError> {
        Ok(())
    }

    #[test]
    fn retain_rejects_empty_identity() {
        let mut decoded = decoded_presentation("   ");
        decoded.identity = String::new();
        assert!(matches!(
            HostRootProcess::retain(decoded, 1),
            Err(InvalidToken(reason)) if reason == "host target identity is empty"
        ));
    }

    #[test]
    fn retain_accepts_and_tracks_identity_and_revision() {
        let process =
            HostRootProcess::retain(decoded_presentation("host-root-process"), 3).expect("retain");
        assert_eq!(process.identity(), "host-root-process");
        assert_eq!(process.presentation_revision(), 3);
    }

    #[test]
    fn synchronize_detects_identity_and_revision_conflicts() {
        let mut process =
            HostRootProcess::retain(decoded_presentation("host-root-process"), 3).expect("retain");
        assert!(matches!(
            process.synchronize(2, decoded_presentation("host-root-process")),
            Err(StaleRevision {
                current: 3,
                received: 2
            })
        ));
        assert!(matches!(
            process.synchronize(3, decoded_presentation("another-root")),
            Err(IdentityChanged)
        ));
        let revision_conflicting = decoded_presentation("host-root-process");
        let revision_conflicting_without_family = DecodedRootPresentation {
            command_families: None,
            ..revision_conflicting
        };
        assert!(matches!(
            process.synchronize(3, revision_conflicting_without_family),
            Err(RevisionConflict { revision: 3 })
        ));
        let mut revision_conflicting = decoded_presentation("host-root-process");
        revision_conflicting.command_families = None;
        assert!(process.synchronize(4, revision_conflicting).is_ok());
        let mut same_revision = decoded_presentation("host-root-process");
        same_revision.command_families = None;
        assert!(
            !process
                .synchronize(4, same_revision)
                .expect("same revision returns cached state")
        );
        assert!(
            process
                .synchronize(5, decoded_presentation("host-root-process"))
                .is_ok()
        );
    }

    #[test]
    fn synchronize_with_router_rejects_duplicate_lease_and_routes_output() {
        assert!(successful_effect().is_ok());
        let mut process =
            HostRootProcess::retain(decoded_presentation("host-root-process"), 1).expect("retain");
        assert!(matches!(
            process.synchronize_with_router(
                1,
                decoded_presentation("host-root-process"),
                Box::new(no_effect)
            ),
            Err(DuplicateLease { revision: 1 })
        ));

        let mut no_effect_process = HostRootProcess::retain_with_router(
            decoded_presentation("host-root-no-effect"),
            1,
            Box::new(no_effect),
        )
        .expect("router retain");
        let context = egui::Context::default();
        crate::run_ui_discard(&context, egui::RawInput::default(), |ui| {
            let _ = no_effect_process.show(ui).expect("no-effect show");
        });

        let router_calls = Rc::new(Cell::new(0usize));
        let closure_calls = Rc::clone(&router_calls);
        let next = DecodedRootPresentation {
            identity: String::from("host-root-process"),
            presentation: minimal_presentation(),
            style: TextCommandSurfaceStyle::standard(),
            command_families: None,
        };
        assert!(
            !process
                .synchronize_with_router(
                    2,
                    next,
                    Box::new(move |_context| {
                        closure_calls.set(closure_calls.get() + 1);
                        Ok(Some(KucOpaqueHostEffectBatch::from_handler(
                            successful_effect,
                        )))
                    })
                )
                .expect("lease update")
        );

        let context = egui::Context::default();
        crate::run_ui_discard(&context, egui::RawInput::default(), |ui| {
            let _ = process.show(ui).expect("show");
        });
        assert_eq!(router_calls.get(), 1);
    }

    #[test]
    fn show_propagates_router_error() {
        let mut process =
            HostRootProcess::retain(decoded_presentation("host-root-process"), 1).expect("retain");
        process.effect_router = Some(Box::new(|_context| Err(KucOpaqueHostEffectError)));

        let context = egui::Context::default();
        crate::run_ui_discard(&context, egui::RawInput::default(), |_ui| {
            assert!(matches!(
                process.show(_ui),
                Err(EguiTextCommandSurfaceRootFactoryError::OpaqueHostEffect)
            ));
        });
    }
}
