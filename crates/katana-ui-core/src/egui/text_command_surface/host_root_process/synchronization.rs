use super::super::super::EditorViewportProjectionLease;
use super::super::super::source_address_projection_lease::SourceAddressProjectionLease;
use super::super::super::status_diagnostics_projection_lease::StatusDiagnosticsProjectionLease;
use super::HostRootProcess;
use super::{
    DecodedRootPresentation, EguiTextCommandSurfaceRootFactoryError, HostProjectionAttachments,
};

impl HostRootProcess {
    pub(in super::super) fn synchronize(
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
        Ok(self.apply_presentation(revision, decoded, false))
    }

    fn apply_presentation(
        &mut self,
        revision: u64,
        decoded: DecodedRootPresentation,
        preserve_tabs: bool,
    ) -> bool {
        let family_changed = decoded.command_families != self.command_families;
        let mut changed = self
            .root
            .synchronize_presentation(decoded.presentation.clone());
        if family_changed && let Some(command_families) = decoded.command_families.as_ref() {
            changed |= self.root.synchronize_command_families(
                command_families.primary().cloned(),
                command_families.floating().cloned(),
            );
        }
        /* WHY: 通常tokenにはtab leaseがないため、以前のleaseが所有していたtabを保持しない。 */
        if !preserve_tabs {
            changed |= self.root.clear_tab_strip();
        }
        changed |= self.root.clear_source_address();
        changed |= self.root.clear_status_diagnostics();
        changed |= self.root.clear_editor_viewport();
        changed |= self.root.global_actions.take().is_some();
        self.root.global_renderer = None;
        self.root
            .global_generation
            .set(self.root.global_generation.get().saturating_add(1));
        self.style = decoded.style;
        self.presentation = decoded.presentation;
        self.command_families = decoded.command_families;
        self.presentation_revision = revision;
        changed
    }

    pub(in super::super) fn synchronize_with_router(
        &mut self,
        revision: u64,
        decoded: DecodedRootPresentation,
        attachments: HostProjectionAttachments,
    ) -> Result<bool, EguiTextCommandSurfaceRootFactoryError> {
        if revision <= self.presentation_revision {
            return Err(EguiTextCommandSurfaceRootFactoryError::DuplicateLease { revision });
        }
        if decoded.identity != self.identity {
            return Err(EguiTextCommandSurfaceRootFactoryError::IdentityChanged);
        }
        let preserve_tabs = attachments.tab_strip.is_some();
        let HostProjectionAttachments {
            source_address: source,
            tab_strip: tabs,
            status_diagnostics: status,
            editor_viewport: viewport,
            context_menu_enabled_resolver,
            global_actions,
            ..
        } = attachments;
        /* WHY: hostが取消を拒否した場合、外側の状態を再試行可能なまま保持する。 */
        let mut changed = if let Some(tab_strip) = tabs {
            self.root.attach_tab_strip(tab_strip)?
        } else {
            false
        };
        changed |= self.apply_presentation(revision, decoded, preserve_tabs);
        changed |= self.synchronize_lease_projections(source, status, viewport);
        self.effect_router = Some(attachments.router);
        self.root.global_actions = global_actions;
        self.root
            .set_context_menu_enabled_resolver(context_menu_enabled_resolver);
        Ok(changed)
    }

    /* WHY: 先に発行したlease系列は初期presentationを保持しているため、入力適用済みの本文を
    巻き戻さず、host配線と任意の子projectionだけを次のleaseへ進める。 */
    #[cfg(any(test, feature = "storybook-artifacts"))]
    pub(in super::super) fn synchronize_router_preserving_state(
        &mut self,
        revision: u64,
        decoded: DecodedRootPresentation,
        attachments: HostProjectionAttachments,
    ) -> Result<bool, EguiTextCommandSurfaceRootFactoryError> {
        if decoded.identity != self.identity {
            return Err(EguiTextCommandSurfaceRootFactoryError::IdentityChanged);
        }
        if revision <= self.presentation_revision {
            return Err(EguiTextCommandSurfaceRootFactoryError::DuplicateLease { revision });
        }

        let mut changed = match attachments.tab_strip {
            Some(tab_strip) => self.root.attach_tab_strip(tab_strip)?,
            None => self.root.clear_tab_strip(),
        };
        changed |= self.root.clear_source_address();
        changed |= self.root.clear_status_diagnostics();
        changed |= self.root.clear_editor_viewport();
        changed |= self.synchronize_lease_projections(
            attachments.source_address,
            attachments.status_diagnostics,
            attachments.editor_viewport,
        );
        self.effect_router = Some(attachments.router);
        self.root.global_actions = attachments.global_actions;
        self.root.global_renderer = None;
        self.root
            .global_generation
            .set(self.root.global_generation.get().saturating_add(1));
        self.presentation_revision = revision;
        self.root
            .set_context_menu_enabled_resolver(attachments.context_menu_enabled_resolver);
        Ok(changed)
    }

    fn synchronize_lease_projections(
        &mut self,
        source_address: Option<SourceAddressProjectionLease>,
        status_diagnostics: Option<StatusDiagnosticsProjectionLease>,
        editor_viewport: Option<EditorViewportProjectionLease>,
    ) -> bool {
        let mut changed = false;
        if let Some(source_address) = source_address {
            self.root.attach_source_address(source_address);
            changed = true;
        }
        if let Some(status_diagnostics) = status_diagnostics {
            let (status_bar, diagnostics_list) = status_diagnostics.into_parts();
            if let Some(status_bar) = status_bar {
                self.root.attach_status_bar(status_bar);
                changed = true;
            }
            if let Some(diagnostics_list) = diagnostics_list {
                self.root.attach_diagnostics_list(diagnostics_list);
                changed = true;
            }
        }
        if let Some(editor_viewport) = editor_viewport {
            self.root.attach_editor_viewport(editor_viewport);
            changed = true;
        }
        changed
    }
}
