use super::{
    EguiTextCommandSurfaceError, EguiTextCommandSurfaceRoot, SourceAddressProjectionLease,
    StatusDiagnosticsProjectionLease, TabStripProjectionLease, TabStripRetainedState,
};

impl EguiTextCommandSurfaceRoot {
    pub(crate) fn set_context_menu_enabled_resolver(
        &mut self,
        resolver: Option<super::super::ContextMenuEnabledResolver>,
    ) {
        self.adapter.context_menu_enabled_resolver = resolver;
    }

    pub fn attach_source_address(&mut self, lease: SourceAddressProjectionLease) {
        let (strip, port) = lease.into_parts();
        self.surface.set_source_address(strip);
        self.source_address_submission_port = port;
    }

    pub(crate) fn clear_source_address(&mut self) -> bool {
        let cleared = self.surface.clear_source_address();
        self.source_address_submission_port = None;
        cleared
    }

    /// Mounts the generic KUC status child into this retained root.
    pub(crate) fn attach_status_bar(&mut self, status_bar: crate::molecule::StatusBar) {
        self.status_bar = Some(status_bar);
    }

    /// Mounts the generic KUC diagnostics child into this retained root.
    pub(crate) fn attach_diagnostics_list(
        &mut self,
        diagnostics_list: crate::molecule::DiagnosticsList,
    ) {
        self.diagnostics_list = Some(diagnostics_list);
    }

    /// Consumes a generic child projection without exposing child models through the root API.
    pub fn attach_status_diagnostics(&mut self, lease: StatusDiagnosticsProjectionLease) {
        let (status_bar, diagnostics_list) = lease.into_parts();
        if let Some(status_bar) = status_bar {
            self.attach_status_bar(status_bar);
        }
        if let Some(diagnostics_list) = diagnostics_list {
            self.attach_diagnostics_list(diagnostics_list);
        }
    }

    pub(crate) fn attach_tab_strip(
        &mut self,
        lease: TabStripProjectionLease,
    ) -> Result<bool, EguiTextCommandSurfaceError> {
        if let Some(retained) = self.tab_strip.as_mut() {
            return retained.synchronize_lease(lease).map_err(Into::into);
        }
        TabStripRetainedState::from_lease(
            lease,
            std::sync::Arc::clone(&self.adapter.catalog),
            self.adapter.text_raster_config.clone(),
        )
        .map(|tab_strip| {
            self.tab_strip = Some(tab_strip);
            true
        })
        .map_err(Into::into)
    }

    pub(crate) fn clear_tab_strip(&mut self) -> bool {
        self.tab_strip.take().is_some()
    }

    pub(crate) fn clear_status_diagnostics(&mut self) -> bool {
        let changed = self.status_bar.is_some() || self.diagnostics_list.is_some();
        self.status_bar = None;
        self.diagnostics_list = None;
        changed
    }

    pub(crate) fn attach_editor_viewport(
        &mut self,
        lease: super::super::EditorViewportProjectionLease,
    ) {
        self.editor_viewport = Some(lease);
    }

    pub(crate) fn clear_editor_viewport(&mut self) -> bool {
        self.editor_viewport.take().is_some()
    }

    pub(crate) fn synchronize_command_families(
        &mut self,
        primary: Option<crate::molecule::command_chrome::CommandChromeFamilyId>,
        floating: Option<crate::molecule::command_chrome::CommandChromeFamilyId>,
    ) -> bool {
        self.surface.synchronize_command_families(primary, floating)
    }
}
