#[path = "host_root_process/synchronization.rs"]
mod synchronization;

use super::super::root::KucRootEffectRouter;
use super::super::root::{EguiTextCommandSurfaceRoot, EguiTextCommandSurfaceRootOutput};
use super::super::types::TextCommandSurfaceStyle;
use super::EguiTextCommandSurfaceCommandFamilyProjection;
use super::host_root_surface::surface_from_presentation;
use super::host_root_token_codec::DecodedRootPresentation;
use super::host_root_types::EguiTextCommandSurfaceRootFactoryError;
use super::host_root_types::HostProjectionAttachments;

pub(crate) struct HostRootProcess {
    pub(super) root: EguiTextCommandSurfaceRoot,
    pub(super) selection_identity: std::sync::Arc<()>,
    pub(super) selection_visible: bool,
    pub(super) selection_logical_frame: u64,
    identity: String,
    style: TextCommandSurfaceStyle,
    presentation: super::super::types::EguiTextCommandSurfacePresentation,
    command_families: Option<EguiTextCommandSurfaceCommandFamilyProjection>,
    presentation_revision: u64,
    effect_router: Option<Box<dyn KucRootEffectRouter>>,
}

impl HostRootProcess {
    pub(super) const fn selection_frame_serial(&self) -> u64 {
        self.root.selection_frame_serial()
    }
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
        let mut root =
            EguiTextCommandSurfaceRoot::with_identity(decoded.identity.clone(), surface)?;
        let _ = root.synchronize_presentation(decoded.presentation.clone());
        Ok(Self {
            root,
            selection_identity: std::sync::Arc::new(()),
            selection_visible: true,
            selection_logical_frame: 0,
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
        attachments: HostProjectionAttachments,
    ) -> Result<Self, EguiTextCommandSurfaceRootFactoryError> {
        let mut process = Self::retain(decoded, presentation_revision)?;
        process.effect_router = Some(attachments.router);
        process.root.global_actions = attachments.global_actions;
        if let Some(source_address) = attachments.source_address {
            process.root.attach_source_address(source_address);
        }
        if let Some(tab_strip) = attachments.tab_strip {
            let _ = process.root.attach_tab_strip(tab_strip)?;
        }
        if let Some(status_diagnostics) = attachments.status_diagnostics {
            let (status_bar, diagnostics_list) = status_diagnostics.into_parts();
            if let Some(status_bar) = status_bar {
                process.root.attach_status_bar(status_bar);
            }
            if let Some(diagnostics_list) = diagnostics_list {
                process.root.attach_diagnostics_list(diagnostics_list);
            }
        }
        if let Some(editor_viewport) = attachments.editor_viewport {
            process.root.attach_editor_viewport(editor_viewport);
        }
        process
            .root
            .set_context_menu_enabled_resolver(attachments.context_menu_enabled_resolver);
        Ok(process)
    }

    pub(super) fn show(
        &mut self,
        ui: &mut egui::Ui,
    ) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootFactoryError> {
        let output = self.root.show(ui, &self.style)?;
        self.selection_logical_frame = ui.ctx().cumulative_frame_nr().saturating_add(1);
        let viewport = output.evidence_text.record.frame.viewport_bounds;
        self.selection_visible = ui.is_visible()
            && viewport.width > 0
            && viewport.height > 0
            && ui.clip_rect().intersects(egui::Rect::from_min_size(
                egui::pos2(viewport.x as f32, viewport.y as f32),
                egui::vec2(viewport.width as f32, viewport.height as f32),
            ))
            && output
                .artifact_order()
                .contains(&super::super::types::EguiTextCommandSurfaceChild::Text);
        if let Some(router) = self.effect_router.as_mut() {
            let effect = router
                .route(output.events().current_context())
                .map_err(|_| EguiTextCommandSurfaceRootFactoryError::OpaqueHostEffect)?;
            if let Some(effect) = effect {
                output.events().attach_opaque_host_effect_batch(effect)?;
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
