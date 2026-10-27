use super::super::EditorViewportProjectionLease;
use super::super::root::KucRootEffectRouter;
use super::super::source_address_projection_lease::SourceAddressProjectionLease;
use super::super::status_diagnostics_projection_lease::StatusDiagnosticsProjectionLease;
use super::super::tab_strip_projection_lease::TabStripProjectionLease;
use super::host_root_types::{
    EguiTextCommandSurfaceHostProjectionLease, EguiTextCommandSurfacePresentationToken,
    HostProjectionAttachments, HostProjectionAttachmentsParts,
};

impl EguiTextCommandSurfaceHostProjectionLease {
    #[cfg(any(test, feature = "storybook-artifacts"))]
    pub(crate) fn token(&self) -> &EguiTextCommandSurfacePresentationToken {
        &self.token
    }

    #[must_use]
    pub fn new<R>(token: EguiTextCommandSurfacePresentationToken, router: R) -> Self
    where
        R: KucRootEffectRouter + 'static,
    {
        Self::from_router(token, Box::new(router))
    }

    #[must_use]
    pub fn from_router(
        token: EguiTextCommandSurfacePresentationToken,
        router: Box<dyn KucRootEffectRouter>,
    ) -> Self {
        Self {
            token,
            router,
            source_address: None,
            tab_strip: None,
            status_diagnostics: None,
            editor_viewport: None,
            context_menu_enabled_resolver: None,
            global_actions: None,
        }
    }

    #[must_use]
    pub fn with_source_address(mut self, lease: SourceAddressProjectionLease) -> Self {
        self.source_address = Some(lease);
        self
    }
    #[must_use]
    pub fn with_tab_strip(mut self, lease: TabStripProjectionLease) -> Self {
        self.tab_strip = Some(lease);
        self
    }
    #[must_use]
    pub fn with_status_diagnostics(mut self, lease: StatusDiagnosticsProjectionLease) -> Self {
        self.status_diagnostics = Some(lease);
        self
    }
    #[must_use]
    pub fn with_editor_viewport(mut self, lease: EditorViewportProjectionLease) -> Self {
        self.editor_viewport = Some(lease);
        self
    }

    /// Attaches host-defined actions whose scope is this retained root.
    #[must_use]
    pub fn with_global_actions(
        mut self,
        value: super::super::root::KucGlobalActionsProjection,
    ) -> Self {
        self.global_actions = Some(value);
        self
    }

    #[must_use]
    pub fn with_context_menu_enabled_resolver<F>(mut self, resolver: F) -> Self
    where
        F: FnMut(
                &str,
                bool,
            ) -> Result<
                bool,
                crate::egui::text_command_surface::KucContextMenuEnabledResolverError,
            > + 'static,
    {
        self.context_menu_enabled_resolver = Some(Box::new(resolver));
        self
    }

    pub(super) fn into_parts_with_attachments(self) -> HostProjectionAttachmentsParts {
        (
            self.token,
            HostProjectionAttachments {
                router: self.router,
                source_address: self.source_address,
                tab_strip: self.tab_strip,
                status_diagnostics: self.status_diagnostics,
                editor_viewport: self.editor_viewport,
                context_menu_enabled_resolver: self.context_menu_enabled_resolver,
                global_actions: self.global_actions,
            },
        )
    }
}

impl std::fmt::Debug for EguiTextCommandSurfaceHostProjectionLease {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("EguiTextCommandSurfaceHostProjectionLease(..)")
    }
}
