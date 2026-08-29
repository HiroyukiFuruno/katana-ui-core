use super::types::EguiTextCommandSurface;
use crate::context_menu::ContextMenuPresentation;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchStrip, CommandChromeToolbar, FloatingCommandToolbar,
    FloatingCommandToolbarVisibility,
};
use katana_ui_core::text_surface::TextSurface;

impl EguiTextCommandSurface {
    pub(super) fn apply_command_family_projection(
        &mut self,
        projection: &super::host_root::EguiTextCommandSurfaceCommandFamilyProjection,
    ) {
        if let Some(family) = projection.primary.clone()
            && let Some(toolbar) = self.toolbar.take()
        {
            self.toolbar = Some(toolbar.command_family(family));
        }
        if let Some(family) = projection.floating.clone() {
            if let Some(toolbar) = self.deferred_floating_toolbar.take() {
                self.deferred_floating_toolbar = Some(toolbar.command_family(family.clone()));
            }
            if let Some(toolbar) = self.floating.take() {
                self.floating = Some(toolbar.command_family(family));
            }
        }
    }

    #[must_use]
    pub fn new(text: TextSurface) -> Self {
        Self {
            text,
            toolbar: None,
            floating: None,
            deferred_floating_toolbar: None,
            floating_visibility: FloatingCommandToolbarVisibility::Closed,
            floating_visibility_controlled: false,
            search: None,
            context_menu: None,
        }
    }

    #[must_use]
    pub fn with_toolbar(mut self, toolbar: CommandChromeToolbar) -> Self {
        self.toolbar = Some(toolbar);
        self
    }

    #[must_use]
    pub fn with_floating_toolbar(
        mut self,
        toolbar: CommandChromeToolbar,
        visibility: FloatingCommandToolbarVisibility,
    ) -> Self {
        self.deferred_floating_toolbar = Some(toolbar);
        self.floating_visibility = visibility;
        self
    }

    #[must_use]
    pub fn with_search_strip(mut self, search: CommandChromeSearchStrip) -> Self {
        self.search = Some(search);
        self
    }

    #[must_use]
    pub fn with_context_menu(mut self, presentation: ContextMenuPresentation) -> Self {
        self.context_menu = Some(presentation);
        self
    }

    #[must_use]
    pub fn text(&self) -> &TextSurface {
        &self.text
    }

    #[must_use]
    pub fn toolbar(&self) -> Option<&CommandChromeToolbar> {
        self.toolbar.as_ref()
    }

    #[must_use]
    pub fn floating_toolbar(&self) -> Option<&FloatingCommandToolbar> {
        self.floating.as_ref()
    }

    #[must_use]
    pub fn search_strip(&self) -> Option<&CommandChromeSearchStrip> {
        self.search.as_ref()
    }

    #[must_use]
    pub fn context_menu_presentation(&self) -> Option<&ContextMenuPresentation> {
        self.context_menu.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::atom::TextArea;
    use katana_ui_core::interaction::placement::{Rect, Size};
    use katana_ui_core::molecule::command_chrome::{
        CommandChromeFamilyId, FloatingCommandToolbarLayout,
    };
    use katana_ui_core::text_surface::{TextSurfaceProps, TextSurfaceViewport};

    #[test]
    fn family_projection_reaches_primary_deferred_and_materialized_floating_slots() {
        let text = TextSurface::new(TextSurfaceProps::new(
            TextArea::new("surface"),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, 320, 120),
        ));
        let toolbar = CommandChromeToolbar::new();
        let mut surface = EguiTextCommandSurface::new(text)
            .with_toolbar(toolbar.clone())
            .with_floating_toolbar(toolbar.clone(), FloatingCommandToolbarVisibility::Visible);
        surface.floating = Some(FloatingCommandToolbar::new(
            toolbar,
            FloatingCommandToolbarLayout::new(
                Rect::new(0, 0, 10, 10),
                Size::new(20, 10),
                Rect::new(0, 0, 100, 100),
            ),
        ));
        surface.apply_command_family_projection(
            &super::super::host_root::EguiTextCommandSurfaceCommandFamilyProjection::new(
                Some(CommandChromeFamilyId::new("primary")),
                Some(CommandChromeFamilyId::new("floating")),
            ),
        );

        assert!(surface.toolbar().is_some());
        assert!(surface.floating_toolbar().is_some());
        assert!(surface.search_strip().is_none());
    }

    #[test]
    fn family_projection_is_additive_when_command_slots_are_absent() {
        let text = TextSurface::new(TextSurfaceProps::new(
            TextArea::new("surface"),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, 320, 120),
        ));
        let mut surface = EguiTextCommandSurface::new(text);
        surface.apply_command_family_projection(
            &super::super::host_root::EguiTextCommandSurfaceCommandFamilyProjection::new(
                Some(CommandChromeFamilyId::new("primary")),
                Some(CommandChromeFamilyId::new("floating")),
            ),
        );
        assert!(surface.toolbar().is_none());
        assert!(surface.floating_toolbar().is_none());
        surface.apply_command_family_projection(
            &super::super::host_root::EguiTextCommandSurfaceCommandFamilyProjection::new(
                None, None,
            ),
        );
    }
}
