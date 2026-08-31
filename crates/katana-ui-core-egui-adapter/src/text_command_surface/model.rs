use super::types::EguiTextCommandSurface;
use crate::context_menu::ContextMenuPresentation;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeFamilyId, CommandChromeSearchStrip, CommandChromeToolbar, FloatingCommandToolbar,
    FloatingCommandToolbarVisibility,
};
use katana_ui_core::molecule::structured::source_address_strip::SourceAddressStrip;
use katana_ui_core::text_surface::TextSurface;

impl EguiTextCommandSurface {
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
            search_closed_by_interaction: false,
            context_menu: None,
            primary_command_family: None,
            floating_command_family: None,
            source_address: None,
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

    pub(crate) fn set_source_address(&mut self, strip: SourceAddressStrip) {
        self.source_address = Some(strip);
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

    pub(crate) fn synchronize_command_families(
        &mut self,
        primary: Option<CommandChromeFamilyId>,
        floating: Option<CommandChromeFamilyId>,
    ) -> bool {
        let changed =
            self.primary_command_family != primary || self.floating_command_family != floating;
        self.primary_command_family = primary;
        self.floating_command_family = floating;
        changed
    }

    pub(crate) fn primary_command_family(&self) -> Option<&CommandChromeFamilyId> {
        self.toolbar.as_ref().and_then(|toolbar| {
            self.primary_command_family
                .as_ref()
                .or_else(|| Some(toolbar.command_family_id()))
        })
    }

    pub(crate) fn floating_command_family(&self) -> Option<&CommandChromeFamilyId> {
        self.deferred_floating_toolbar
            .as_ref()
            .or_else(|| self.floating.as_ref().map(|value| value.toolbar_model()))
            .and_then(|toolbar| {
                self.floating_command_family
                    .as_ref()
                    .or_else(|| Some(toolbar.command_family_id()))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::EguiTextCommandSurface;
    use katana_ui_core::atom::TextArea;
    use katana_ui_core::interaction::placement::{Rect, Size};
    use katana_ui_core::molecule::command_chrome::{
        CommandChromeToolbar, FloatingCommandToolbar, FloatingCommandToolbarLayout,
        FloatingCommandToolbarVisibility,
    };
    use katana_ui_core::text_surface::{TextSurface, TextSurfaceProps, TextSurfaceViewport};

    fn text_surface() -> TextSurface {
        let value = "fixture";
        let props = TextSurfaceProps::new(
            TextArea::new("surface-text").value(value),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, 16, 16),
        );
        TextSurface::new(props)
    }

    fn floating_toolbar() -> FloatingCommandToolbar {
        let toolbar = CommandChromeToolbar::new().action(
            katana_ui_core::molecule::command_chrome::CommandChromeAction::new(
                "fixture", "fixture",
            ),
        );
        FloatingCommandToolbar::new(
            toolbar,
            FloatingCommandToolbarLayout::new(
                Rect::new(0, 0, 10, 10),
                Size::new(1, 1),
                Rect::new(0, 0, 20, 20),
            ),
        )
        .focus_return_target("focus-return".into())
        .initial_visibility(FloatingCommandToolbarVisibility::Visible)
    }

    #[test]
    fn floating_toolbar_returns_none_before_assignment() {
        let surface = EguiTextCommandSurface::new(text_surface());
        assert!(surface.floating_toolbar().is_none());
    }

    #[test]
    fn floating_toolbar_returns_active_toolbar_when_present() {
        let mut surface = EguiTextCommandSurface::new(text_surface());
        surface.floating = Some(floating_toolbar());
        assert!(surface.floating_toolbar().is_some());
    }
}
