use katana_ui_core::molecule::{
    CollapsiblePanel, CollapsiblePanelAction, CollapsiblePanelEvent, CollapsiblePanelWidth,
    PanelMode,
};

const WIDTH_MIN: u16 = 180;
const WIDTH_MAX: u16 = 420;
const WIDTH_DEFAULT: u16 = 240;
const WIDTH_CURRENT: u16 = 260;
const WIDTH_RESIZED: u16 = 320;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum CollapsiblePanelStoryAction {
    Resize,
    Focus,
    Hover,
    KeyboardToggle,
    ContextPinToggle,
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::visual) struct CollapsiblePanelScreenState {
    panel: CollapsiblePanel,
    pub(in crate::visual) focused: bool,
    pub(in crate::visual) hovered: bool,
    pub(in crate::visual) context_open: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct CollapsiblePanelUpdate {
    pub(in crate::visual) action: &'static str,
    pub(in crate::visual) event: &'static str,
    pub(in crate::visual) state: &'static str,
}

impl Default for CollapsiblePanelScreenState {
    fn default() -> Self {
        Self {
            panel: panel(),
            focused: false,
            hovered: false,
            context_open: false,
        }
    }
}

impl CollapsiblePanelScreenState {
    pub(in crate::visual) fn apply(
        &mut self,
        action: CollapsiblePanelStoryAction,
    ) -> CollapsiblePanelUpdate {
        match action {
            CollapsiblePanelStoryAction::Resize => self.resize(),
            CollapsiblePanelStoryAction::Focus => self.focus(),
            CollapsiblePanelStoryAction::Hover => self.hover(),
            CollapsiblePanelStoryAction::KeyboardToggle => self.keyboard_toggle(),
            CollapsiblePanelStoryAction::ContextPinToggle => self.context_pin_toggle(),
        }
    }

    fn resize(&mut self) -> CollapsiblePanelUpdate {
        self.panel
            .apply_action(CollapsiblePanelAction::Resize(WIDTH_RESIZED));
        CollapsiblePanelUpdate::new(
            "collapsible_panel_resize",
            "collapsible_panel_width_changed",
            "width=320",
        )
    }

    fn focus(&mut self) -> CollapsiblePanelUpdate {
        self.focused = true;
        self.panel
            .apply_action(CollapsiblePanelAction::SetMode(PanelMode::Expanded));
        CollapsiblePanelUpdate::new(
            "collapsible_panel_focus",
            "collapsible_panel_mode_changed",
            "focus=true",
        )
    }

    fn hover(&mut self) -> CollapsiblePanelUpdate {
        self.hovered = true;
        let mut hover_panel = self
            .panel
            .clone()
            .mode(PanelMode::IconOnly)
            .pinned(false)
            .expand_on_hover(true);
        hover_panel.apply_action(CollapsiblePanelAction::HoverTrigger);
        self.panel = hover_panel;
        CollapsiblePanelUpdate::new(
            "collapsible_panel_hover",
            "collapsible_panel_hover_expanded",
            "hover=expanded",
        )
    }

    fn keyboard_toggle(&mut self) -> CollapsiblePanelUpdate {
        self.panel
            .apply_action(CollapsiblePanelAction::ToggleExpand);
        CollapsiblePanelUpdate::new(
            "collapsible_panel_keyboard_toggle",
            "collapsible_panel_mode_changed",
            "mode=toggled",
        )
    }

    fn context_pin_toggle(&mut self) -> CollapsiblePanelUpdate {
        self.context_open = true;
        let events = self.panel.apply_action(CollapsiblePanelAction::Unpin);
        let event = if events
            .iter()
            .any(|it| matches!(it, CollapsiblePanelEvent::PinChanged { pinned: false }))
        {
            "collapsible_panel_pin_changed"
        } else {
            "collapsible_panel_context_opened"
        };
        CollapsiblePanelUpdate::new("collapsible_panel_context_pin", event, "pinned=false")
    }
}

impl CollapsiblePanelUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}

fn panel() -> CollapsiblePanel {
    CollapsiblePanel::new(
        "Collapsible panel",
        CollapsiblePanelWidth::new(
            WIDTH_MIN,
            WIDTH_MAX,
            WIDTH_DEFAULT,
            WIDTH_CURRENT,
            None::<String>,
        ),
    )
    .resize_handle(true)
}
