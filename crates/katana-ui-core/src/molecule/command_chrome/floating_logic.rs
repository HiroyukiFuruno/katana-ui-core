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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interaction::placement::Size;
    use crate::molecule::command_chrome::{
        CommandChromeAction, CommandChromeDropdown, CommandChromeDropdownItem,
        CommandChromeDropdownLayout, CommandChromeDropdownTrigger, CommandChromeOpenDropdown,
        CommandChromeToolbarAction, CommandChromeToolbarEvent, FloatingCommandToolbarCloseReason,
        FloatingCommandToolbarLayout,
    };

    fn floating() -> FloatingCommandToolbar {
        FloatingCommandToolbar::new(
            super::super::CommandChromeToolbar::new(),
            FloatingCommandToolbarLayout::new(
                Rect::new(10, 10, 4, 4),
                Size::new(40, 20),
                Rect::new(0, 0, 100, 100),
            ),
        )
    }

    #[test]
    fn close_reason_mapping_and_default_priority_cover_every_variant() {
        assert_eq!(
            dropdown_close_reason(FloatingCommandToolbarCloseReason::OutsideClick),
            CommandChromeDropdownCloseReason::OutsideClick
        );
        assert_eq!(
            dropdown_close_reason(FloatingCommandToolbarCloseReason::Escape),
            CommandChromeDropdownCloseReason::Escape
        );
        assert_eq!(
            dropdown_close_reason(FloatingCommandToolbarCloseReason::ConsumerSurfaceClick),
            CommandChromeDropdownCloseReason::Explicit
        );
        assert_eq!(
            dropdown_close_reason(FloatingCommandToolbarCloseReason::Explicit),
            CommandChromeDropdownCloseReason::Explicit
        );
        assert_eq!(
            placement_priority(&[]),
            vec![Placement::BottomStart, Placement::TopStart]
        );
        assert_eq!(
            placement_priority(&[Placement::Right]),
            vec![Placement::Right]
        );
    }

    #[test]
    fn closed_toolbar_actions_are_ignored_and_dismiss_closes_an_open_dropdown_first() {
        let mut floating = floating();
        assert!(
            floating
                .apply_action(FloatingCommandToolbarAction::Toolbar {
                    action: CommandChromeToolbarAction::DismissDropdown {
                        reason: CommandChromeDropdownCloseReason::Explicit,
                    },
                })
                .is_empty()
        );

        floating.initialize_visible();
        floating.focus_return_target = Some(crate::render_model::UiNodeId::new("surface"));
        floating.toolbar.open_dropdown = Some(CommandChromeOpenDropdown::new(
            "menu".into(),
            CommandChromeDropdownLayout::new(
                Rect::new(0, 0, 10, 10),
                Rect::new(0, 0, 100, 100),
                Size::new(50, 40),
            ),
            None,
        ));
        let events = floating.apply_action(FloatingCommandToolbarAction::Dismiss {
            reason: FloatingCommandToolbarCloseReason::OutsideClick,
        });
        assert!(matches!(
            events.first(),
            Some(FloatingCommandToolbarEvent::FocusRetained)
        ));
        assert!(
            events
                .iter()
                .any(|event| matches!(event, FloatingCommandToolbarEvent::Toolbar { .. }))
        );
        assert!(matches!(
            events.last(),
            Some(FloatingCommandToolbarEvent::FocusReturnRequested { target })
                if target.as_str() == "surface"
        ));
        assert!(events.iter().any(|event| matches!(
            event,
            FloatingCommandToolbarEvent::Closed {
                reason: FloatingCommandToolbarCloseReason::OutsideClick
            }
        )));
    }

    #[test]
    fn synchronize_toolbar_and_visibility_paths_cover_expected_state_transitions() {
        let mut floating = floating();
        let anchor = crate::interaction::placement::Rect::new(0, 0, 12, 6);
        let viewport = crate::interaction::placement::Rect::new(0, 0, 120, 120);
        let toolbar_presentation = super::super::CommandChromeToolbarPresentation {
            actions: vec![CommandChromeAction::new("copy", "Copy")],
            groups: Vec::new(),
            display_mode: crate::molecule::command_chrome::CommandChromeDisplayMode::LabelOnly,
            density: crate::molecule::toolbar::ToolbarDensity::default(),
            overflow_strategy: crate::molecule::toolbar::ToolbarStrategy::default(),
        };
        let presentation = FloatingCommandToolbarPresentation::new(
            anchor,
            viewport,
            FloatingCommandToolbarVisibility::Visible,
        );

        assert!(floating.synchronize_toolbar_presentation(toolbar_presentation.clone()));
        assert!(!floating.synchronize_toolbar_presentation(toolbar_presentation));
        assert!(floating.synchronize_presentation(presentation));
        assert!(floating.is_open());
        assert!(floating.placement_model().is_some());

        assert!(!floating.synchronize_presentation(presentation));
        let closed = FloatingCommandToolbarPresentation::new(
            anchor,
            viewport,
            FloatingCommandToolbarVisibility::Closed,
        );
        assert!(floating.synchronize_presentation(closed));
        assert!(!floating.is_open());
        assert!(floating.placement_model().is_none());
        assert!(floating.bounds_model().is_none());

        let reopen = FloatingCommandToolbarPresentation::new(
            anchor,
            viewport,
            FloatingCommandToolbarVisibility::Visible,
        );
        assert!(floating.synchronize_presentation(reopen));
        let previous_placement = floating.placement_model();
        assert!(floating.synchronize_measured_panel(Size::new(41, 21)));
        let measured_placement = floating.placement_model();
        assert_eq!(previous_placement, measured_placement);

        let unchanged = floating.synchronize_measured_panel(Size::new(41, 21));
        assert!(!unchanged);

        let close_event = floating.apply_action(FloatingCommandToolbarAction::Dismiss {
            reason: FloatingCommandToolbarCloseReason::Escape,
        });
        assert!(matches!(
            close_event.first(),
            Some(FloatingCommandToolbarEvent::Closed {
                reason: FloatingCommandToolbarCloseReason::Escape
            })
        ));

        let closed = floating.apply_action(FloatingCommandToolbarAction::Dismiss {
            reason: FloatingCommandToolbarCloseReason::OutsideClick,
        });
        assert!(closed.is_empty());
    }

    #[test]
    fn apply_action_dispatches_toolbar_events_with_open_reposition_and_toolbar_routes() {
        let mut floating = floating();
        let open_event = floating.apply_action(FloatingCommandToolbarAction::Open);
        assert!(matches!(
            open_event.first(),
            Some(FloatingCommandToolbarEvent::Opened { .. })
        ));
        let reposition_event = floating.apply_action(FloatingCommandToolbarAction::Open);
        assert!(matches!(
            reposition_event.first(),
            Some(FloatingCommandToolbarEvent::Repositioned { .. })
        ));

        let resized = floating.apply_action(FloatingCommandToolbarAction::UpdateLayout {
            layout: FloatingCommandToolbarLayout::new(
                crate::interaction::placement::Rect::new(10, 10, 20, 10),
                Size::new(80, 20),
                crate::interaction::placement::Rect::new(0, 0, 120, 120),
            ),
        });
        assert!(matches!(
            resized.first(),
            Some(FloatingCommandToolbarEvent::Repositioned { .. })
        ));

        let mut toolbar_floating = FloatingCommandToolbar::new(
            super::super::CommandChromeToolbar::new().action(
                CommandChromeAction::new("format", "Format").dropdown(
                    CommandChromeDropdown::new(CommandChromeDropdownTrigger::Primary)
                        .item(CommandChromeDropdownItem::new("item", "Item")),
                ),
            ),
            FloatingCommandToolbarLayout::new(
                crate::interaction::placement::Rect::new(10, 10, 4, 4),
                Size::new(40, 20),
                crate::interaction::placement::Rect::new(0, 0, 120, 120),
            ),
        );
        toolbar_floating.toolbar_model_mut().update_dropdown_layout(
            "format".into(),
            CommandChromeDropdownLayout::new(
                crate::interaction::placement::Rect::new(0, 0, 10, 10),
                crate::interaction::placement::Rect::new(0, 0, 120, 120),
                Size::new(40, 20),
            ),
        );
        toolbar_floating.initialize_visible();
        let toolbar_activation =
            toolbar_floating.apply_action(FloatingCommandToolbarAction::Toolbar {
                action: CommandChromeToolbarAction::Activate {
                    action_id: "format".into(),
                },
            });
        assert!(matches!(
            toolbar_activation.get(1),
            Some(FloatingCommandToolbarEvent::Toolbar {
                event: CommandChromeToolbarEvent::DropdownOpened { .. }
            })
        ));
        let dropdown_open = toolbar_floating.apply_action(FloatingCommandToolbarAction::Toolbar {
            action: CommandChromeToolbarAction::Activate {
                action_id: "format".into(),
            },
        });
        assert!(matches!(
            dropdown_open.first(),
            Some(FloatingCommandToolbarEvent::FocusRetained)
        ));
        let escape = toolbar_floating.apply_action(FloatingCommandToolbarAction::Dismiss {
            reason: FloatingCommandToolbarCloseReason::Escape,
        });
        assert!(matches!(
            escape.first(),
            Some(FloatingCommandToolbarEvent::FocusRetained)
        ));
        assert!(
            !escape
                .iter()
                .any(|event| matches!(event, FloatingCommandToolbarEvent::Closed { .. }))
        );

        let events = floating.apply_action(FloatingCommandToolbarAction::Toolbar {
            action: CommandChromeToolbarAction::Activate {
                action_id: "format".into(),
            },
        });
        assert!(events.is_empty());

        let closed = floating.apply_action(FloatingCommandToolbarAction::Dismiss {
            reason: FloatingCommandToolbarCloseReason::Explicit,
        });
        assert!(matches!(
            closed.first(),
            Some(FloatingCommandToolbarEvent::Closed { .. })
        ));
        assert!(
            floating
                .apply_action(FloatingCommandToolbarAction::Dismiss {
                    reason: FloatingCommandToolbarCloseReason::Explicit,
                })
                .is_empty()
        );
    }

    #[test]
    fn dismiss_with_consumer_surface_click_drops_down_to_toolbar_close_reason() {
        let mut floating = floating();
        floating.initialize_visible();
        floating.toolbar.open_dropdown = Some(CommandChromeOpenDropdown::new(
            "format".into(),
            CommandChromeDropdownLayout::new(
                crate::interaction::placement::Rect::new(0, 0, 10, 10),
                crate::interaction::placement::Rect::new(0, 0, 100, 100),
                crate::interaction::placement::Size::new(50, 40),
            ),
            Some(0),
        ));

        let events = floating.apply_action(FloatingCommandToolbarAction::Dismiss {
            reason: FloatingCommandToolbarCloseReason::ConsumerSurfaceClick,
        });

        assert!(matches!(
            events.first(),
            Some(FloatingCommandToolbarEvent::FocusRetained)
        ));
        assert!(events.iter().any(|event| {
            matches!(
                event,
                FloatingCommandToolbarEvent::Toolbar {
                    event: CommandChromeToolbarEvent::DropdownClosed {
                        reason: CommandChromeDropdownCloseReason::Explicit,
                        ..
                    }
                }
            )
        }));
    }

    #[test]
    fn synchronize_toolbar_rejects_noop_presentation_updates_without_state_change() {
        let mut floating = floating();
        let same = FloatingCommandToolbarPresentation::new(
            crate::interaction::placement::Rect::new(0, 0, 12, 6),
            crate::interaction::placement::Rect::new(0, 0, 120, 120),
            FloatingCommandToolbarVisibility::Closed,
        );
        assert!(floating.synchronize_presentation(same));
        assert!(!floating.synchronize_presentation(same));
    }
}
