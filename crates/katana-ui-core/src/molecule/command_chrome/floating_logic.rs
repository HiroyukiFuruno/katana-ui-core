use super::{
    CommandChromeDropdownCloseReason, CommandChromeToolbarAction, FloatingCommandToolbar,
    FloatingCommandToolbarAction, FloatingCommandToolbarEvent, FloatingCommandToolbarPresentation,
    FloatingCommandToolbarVisibility,
};
use crate::interaction::placement::{
    AnchorKind, Placement, PlacementEngine, PlacementRequest, PlacementResult, Rect,
};

impl FloatingCommandToolbar {
    /// Synchronizes opaque toolbar presentation without exposing the retained child model.
    pub fn synchronize_toolbar_presentation(
        &mut self,
        value: super::CommandChromeToolbarPresentation,
    ) -> bool {
        self.toolbar.synchronize_presentation(value)
    }

    /// Synchronizes consumer frame facts without fabricating interaction events.
    /// Focus, dropdown and tooltip state remain KUC-owned and are not reset.
    pub fn synchronize_presentation(&mut self, value: FloatingCommandToolbarPresentation) -> bool {
        let changed = self.layout.anchor != value.anchor
            || self.layout.viewport != value.viewport
            || self.visibility_model() != value.visibility;
        if !changed {
            return false;
        }
        self.layout.anchor = value.anchor;
        self.layout.viewport = value.viewport;
        match value.visibility {
            FloatingCommandToolbarVisibility::Visible => {
                self.open = true;
                self.resolve_placement();
            }
            FloatingCommandToolbarVisibility::Closed => {
                self.open = false;
                self.placement = None;
                self.bounds = None;
            }
        }
        true
    }

    /// Records an adapter raster measurement and recomputes KUC-owned placement without events.
    pub fn synchronize_measured_panel(
        &mut self,
        panel_size: crate::interaction::placement::Size,
    ) -> bool {
        if self.layout.panel_size == panel_size {
            return false;
        }
        self.layout.panel_size = panel_size;
        if self.open {
            self.resolve_placement();
        }
        true
    }

    pub(super) fn initialize_visible(&mut self) {
        self.open = true;
        self.resolve_placement();
    }

    #[must_use]
    pub fn apply_action(
        &mut self,
        action: FloatingCommandToolbarAction,
    ) -> Vec<FloatingCommandToolbarEvent> {
        match action {
            FloatingCommandToolbarAction::Open => self.open_or_reposition(),
            FloatingCommandToolbarAction::UpdateLayout { layout } => {
                self.layout = layout;
                self.open.then(|| self.reposition()).into_iter().collect()
            }
            FloatingCommandToolbarAction::Dismiss { reason } => self.dismiss(reason),
            FloatingCommandToolbarAction::Toolbar { action } => self.apply_toolbar_action(action),
        }
    }

    fn open_or_reposition(&mut self) -> Vec<FloatingCommandToolbarEvent> {
        let placement = self.resolve_placement();
        let event = if self.open {
            FloatingCommandToolbarEvent::Repositioned { placement }
        } else {
            self.open = true;
            FloatingCommandToolbarEvent::Opened { placement }
        };
        vec![event]
    }

    fn reposition(&mut self) -> FloatingCommandToolbarEvent {
        FloatingCommandToolbarEvent::Repositioned {
            placement: self.resolve_placement(),
        }
    }

    fn resolve_placement(&mut self) -> PlacementResult {
        let priority = placement_priority(&self.placement_priority);
        let request = PlacementRequest::new(
            AnchorKind::virtual_rect(self.layout.anchor),
            priority[0],
            self.layout.panel_size,
            self.layout.viewport,
        )
        .priority(priority)
        .clamp_margin(self.clamp_margin);
        let placement = PlacementEngine::resolve(&request);
        self.bounds = Some(Rect::new(
            placement.position.x,
            placement.position.y,
            self.layout.panel_size.width,
            self.layout.panel_size.height,
        ));
        self.placement = Some(placement);
        placement
    }

    fn dismiss(
        &mut self,
        reason: super::FloatingCommandToolbarCloseReason,
    ) -> Vec<FloatingCommandToolbarEvent> {
        if !self.open {
            return Vec::new();
        }
        if reason == super::FloatingCommandToolbarCloseReason::Escape
            && self.toolbar.open_dropdown_model().is_some()
        {
            return self.apply_toolbar_action(CommandChromeToolbarAction::DismissDropdown {
                reason: CommandChromeDropdownCloseReason::Escape,
            });
        }
        let mut events = if self.toolbar.open_dropdown_model().is_some() {
            self.apply_toolbar_action(CommandChromeToolbarAction::DismissDropdown {
                reason: dropdown_close_reason(reason),
            })
        } else {
            Vec::new()
        };
        self.open = false;
        self.placement = None;
        self.bounds = None;
        events.push(FloatingCommandToolbarEvent::Closed { reason });
        if let Some(target) = &self.focus_return_target {
            events.push(FloatingCommandToolbarEvent::FocusReturnRequested {
                target: target.clone(),
            });
        }
        events
    }

    fn apply_toolbar_action(
        &mut self,
        action: CommandChromeToolbarAction,
    ) -> Vec<FloatingCommandToolbarEvent> {
        if !self.open {
            return Vec::new();
        }
        let toolbar_events = self.toolbar.apply_action(action);
        if toolbar_events.is_empty() {
            return Vec::new();
        }
        let mut events = vec![FloatingCommandToolbarEvent::FocusRetained];
        events.extend(
            toolbar_events
                .into_iter()
                .map(|event| FloatingCommandToolbarEvent::Toolbar { event }),
        );
        events
    }
}

fn dropdown_close_reason(
    value: super::FloatingCommandToolbarCloseReason,
) -> CommandChromeDropdownCloseReason {
    match value {
        super::FloatingCommandToolbarCloseReason::OutsideClick => {
            CommandChromeDropdownCloseReason::OutsideClick
        }
        super::FloatingCommandToolbarCloseReason::Escape => {
            CommandChromeDropdownCloseReason::Escape
        }
        super::FloatingCommandToolbarCloseReason::ConsumerSurfaceClick
        | super::FloatingCommandToolbarCloseReason::Explicit => {
            CommandChromeDropdownCloseReason::Explicit
        }
    }
}

fn placement_priority(priority: &[Placement]) -> Vec<Placement> {
    if priority.is_empty() {
        return vec![Placement::BottomStart, Placement::TopStart];
    }
    priority.to_vec()
}
