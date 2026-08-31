use super::types::{
    EguiTextCommandSurface, EguiTextCommandSurfaceFloatingPresentation,
    EguiTextCommandSurfacePresentation, EguiTextCommandSurfaceSearchPresentation,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchStrip, CommandChromeToolbar, FloatingCommandToolbarVisibility,
};
use katana_ui_core::molecule::structured::SearchControlStrip;

impl EguiTextCommandSurface {
    /// Synchronizes generic controlled values without exposing mutable child models.
    pub fn synchronize_presentation(&mut self, value: EguiTextCommandSurfacePresentation) -> bool {
        let mut changed = value.search.is_some() && self.search_closed_by_interaction;
        self.search_closed_by_interaction = false;
        changed |= value
            .text_state_id
            .is_some_and(|state_id| self.text.synchronize_state_id(state_id));
        changed |= self.text.synchronize_presentation(value.text);
        changed |= synchronize_toolbar(&mut self.toolbar, value.toolbar);
        changed |= synchronize_floating(self, value.floating);
        changed |= synchronize_search(&mut self.search, value.search);
        changed |= synchronize_context_menu(&mut self.context_menu, value.context_menu);
        changed
    }
}

fn synchronize_context_menu(
    target: &mut Option<crate::context_menu::ContextMenuPresentation>,
    value: Option<crate::context_menu::ContextMenuPresentation>,
) -> bool {
    if *target == value {
        return false;
    }
    *target = value;
    true
}

fn synchronize_toolbar(
    target: &mut Option<CommandChromeToolbar>,
    value: Option<katana_ui_core::molecule::command_chrome::CommandChromeToolbarPresentation>,
) -> bool {
    match value {
        Some(value) => {
            let target = target.get_or_insert_with(CommandChromeToolbar::new);
            target.synchronize_presentation(value)
        }
        None => target.take().is_some(),
    }
}

fn synchronize_floating(
    surface: &mut EguiTextCommandSurface,
    value: Option<EguiTextCommandSurfaceFloatingPresentation>,
) -> bool {
    let Some(value) = value else {
        let changed =
            surface.floating.take().is_some() || surface.deferred_floating_toolbar.take().is_some();
        surface.floating_visibility = FloatingCommandToolbarVisibility::Closed;
        surface.floating_visibility_controlled = false;
        return changed;
    };
    let mut changed = surface.floating_visibility != value.visibility;
    surface.floating_visibility = value.visibility;
    surface.floating_visibility_controlled = true;
    if let Some(floating) = surface.floating.as_mut() {
        changed |= floating.synchronize_toolbar_presentation(value.toolbar);
    } else {
        let toolbar = surface
            .deferred_floating_toolbar
            .get_or_insert_with(CommandChromeToolbar::new);
        changed |= toolbar.synchronize_presentation(value.toolbar);
    }
    changed
}

fn synchronize_search(
    target: &mut Option<CommandChromeSearchStrip>,
    value: Option<EguiTextCommandSurfaceSearchPresentation>,
) -> bool {
    match value {
        Some(value) => {
            let target = target.get_or_insert_with(|| {
                CommandChromeSearchStrip::new(
                    SearchControlStrip::new(value.label).stable_state_id(value.state_id),
                    value.value.strings.clone(),
                )
            });
            target.synchronize_presentation(value.value)
        }
        None => target.take().is_some(),
    }
}
