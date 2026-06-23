use super::{
    WindowControlButtonGroupAction, WindowControlButtonGroupEvent, WindowControlButtonGroupOptions,
    WindowControlKind,
};
use crate::render_model::{UiInteractionState, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowControlButtonGroupState {
    state_id: UiStateId,
    hovered: bool,
    fullscreen: bool,
    visible: bool,
    last_pressed: Option<WindowControlKind>,
    events: Vec<WindowControlButtonGroupEvent>,
}

impl WindowControlButtonGroupState {
    #[must_use]
    pub fn new(options: &WindowControlButtonGroupOptions) -> Self {
        Self {
            state_id: UiStateId::next_for(UiNodeKind::WindowControlButtonGroup),
            hovered: false,
            fullscreen: false,
            visible: options.visible(false, false),
            last_pressed: None,
            events: Vec::new(),
        }
    }

    #[must_use]
    pub fn visible(&self) -> bool {
        self.visible
    }

    #[must_use]
    pub fn events(&self) -> &[WindowControlButtonGroupEvent] {
        &self.events
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    pub(crate) fn sync_options(&mut self, options: &WindowControlButtonGroupOptions) {
        self.visible = options.visible(self.hovered, self.fullscreen);
    }

    pub(crate) fn apply(
        &mut self,
        action: WindowControlButtonGroupAction,
        options: &WindowControlButtonGroupOptions,
    ) -> Vec<WindowControlButtonGroupEvent> {
        let events = match action {
            WindowControlButtonGroupAction::Press(which) => self.press(which),
            WindowControlButtonGroupAction::SetHover(hovered) => self.set_hover(hovered, options),
            WindowControlButtonGroupAction::SetFullscreen(fullscreen) => {
                self.set_fullscreen(fullscreen, options)
            }
        };
        self.events.extend(events.iter().copied());
        events
    }

    pub(crate) fn interaction(
        &self,
        options: &WindowControlButtonGroupOptions,
    ) -> UiInteractionState {
        UiInteractionState {
            open: self.visible,
            hovered: self.hovered,
            active: self.fullscreen,
            has_selection: self.last_pressed.is_some(),
            selected_index: options.position.index(),
            item_count: options.controls.len(),
            ..UiInteractionState::default()
        }
    }

    pub(crate) fn into_state_id(self) -> UiStateId {
        self.state_id
    }

    fn press(&mut self, which: WindowControlKind) -> Vec<WindowControlButtonGroupEvent> {
        self.last_pressed = Some(which);
        vec![WindowControlButtonGroupEvent::ControlPressed { which }]
    }

    fn set_hover(
        &mut self,
        hovered: bool,
        options: &WindowControlButtonGroupOptions,
    ) -> Vec<WindowControlButtonGroupEvent> {
        self.hovered = hovered;
        self.sync_visibility(options)
    }

    fn set_fullscreen(
        &mut self,
        fullscreen: bool,
        options: &WindowControlButtonGroupOptions,
    ) -> Vec<WindowControlButtonGroupEvent> {
        self.fullscreen = fullscreen;
        let mut events = vec![WindowControlButtonGroupEvent::FullscreenChanged { fullscreen }];
        events.extend(self.sync_visibility(options));
        events
    }

    fn sync_visibility(
        &mut self,
        options: &WindowControlButtonGroupOptions,
    ) -> Vec<WindowControlButtonGroupEvent> {
        let next = options.visible(self.hovered, self.fullscreen);
        if self.visible == next {
            return Vec::new();
        }
        self.visible = next;
        vec![WindowControlButtonGroupEvent::VisibilityChanged { visible: next }]
    }
}
