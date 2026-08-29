use super::host_root_types::EguiTextCommandSurfaceCommandFamilyProjection;
use katana_ui_core::molecule::command_chrome::CommandChromeFamilyId;

impl EguiTextCommandSurfaceCommandFamilyProjection {
    pub(crate) fn primary(&self) -> Option<&CommandChromeFamilyId> {
        self.primary.as_ref()
    }

    pub(crate) fn floating(&self) -> Option<&CommandChromeFamilyId> {
        self.floating.as_ref()
    }

    pub(crate) fn legacy_compatibility() -> Self {
        Self {
            primary: Some(CommandChromeFamilyId::default()),
            floating: Some(CommandChromeFamilyId::default()),
        }
    }
}

impl Default for EguiTextCommandSurfaceCommandFamilyProjection {
    fn default() -> Self {
        Self::new(None, None)
    }
}

impl EguiTextCommandSurfaceCommandFamilyProjection {
    #[must_use]
    pub const fn new(
        primary: Option<CommandChromeFamilyId>,
        floating: Option<CommandChromeFamilyId>,
    ) -> Self {
        Self { primary, floating }
    }
}
