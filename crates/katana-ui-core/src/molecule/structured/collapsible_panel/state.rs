use super::ResizableWidth;
use super::{CollapsiblePanelAction, CollapsiblePanelEvent, CollapsiblePanelOptions, PanelMode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollapsiblePanelState {
    pub mode: PanelMode,
    pub width: ResizableWidth,
    pub pinned: bool,
    pub hover_open: bool,
    hover_return_mode: Option<PanelMode>,
}

impl CollapsiblePanelState {
    #[must_use]
    pub fn new(mode: PanelMode, width: ResizableWidth, pinned: bool) -> Self {
        Self {
            mode,
            width,
            pinned,
            hover_open: false,
            hover_return_mode: None,
        }
    }

    pub fn apply_action(
        &mut self,
        action: CollapsiblePanelAction,
        options: &CollapsiblePanelOptions,
    ) -> Vec<CollapsiblePanelEvent> {
        match action {
            CollapsiblePanelAction::ToggleExpand => self.toggle_expand(),
            CollapsiblePanelAction::SetMode(mode) => self.set_mode(mode),
            CollapsiblePanelAction::Resize(width) if options.resize_handle => {
                self.set_width(self.width.clamped(width))
            }
            CollapsiblePanelAction::ResetWidth if options.resize_handle => {
                self.set_width(self.width.default)
            }
            CollapsiblePanelAction::HoverTrigger => self.hover_open(options),
            CollapsiblePanelAction::LeaveTrigger => self.hover_close(),
            CollapsiblePanelAction::Pin => self.set_pinned(true),
            CollapsiblePanelAction::Unpin => self.set_pinned(false),
            _ => Vec::new(),
        }
    }

    #[must_use]
    pub fn rendered_mode(&self) -> PanelMode {
        if self.hover_open {
            PanelMode::Expanded
        } else {
            self.mode
        }
    }

    fn toggle_expand(&mut self) -> Vec<CollapsiblePanelEvent> {
        let next = if self.mode == PanelMode::Expanded {
            PanelMode::Collapsed
        } else {
            PanelMode::Expanded
        };
        self.set_mode(next)
    }

    fn set_mode(&mut self, mode: PanelMode) -> Vec<CollapsiblePanelEvent> {
        let from = self.mode;
        if from == mode {
            return Vec::new();
        }
        self.mode = mode;
        self.hover_open = false;
        self.hover_return_mode = None;
        mode_events(from, mode)
    }

    fn set_width(&mut self, width: u16) -> Vec<CollapsiblePanelEvent> {
        if self.width.current == width {
            return Vec::new();
        }
        self.width.current = width;
        vec![CollapsiblePanelEvent::WidthChanged {
            width,
            persist_id: self.width.persist_id.clone(),
        }]
    }

    fn hover_open(&mut self, options: &CollapsiblePanelOptions) -> Vec<CollapsiblePanelEvent> {
        if self.pinned || !options.expand_on_hover || self.hover_open {
            return Vec::new();
        }
        self.hover_return_mode = Some(self.mode);
        self.hover_open = true;
        vec![CollapsiblePanelEvent::HoverTemporaryExpanded {
            from: self.mode,
            to: PanelMode::Expanded,
        }]
    }

    fn hover_close(&mut self) -> Vec<CollapsiblePanelEvent> {
        if !self.hover_open {
            return Vec::new();
        }
        self.hover_open = false;
        let restored = self.hover_return_mode.take().unwrap_or(self.mode);
        vec![CollapsiblePanelEvent::HoverTemporaryClosed { restored }]
    }

    fn set_pinned(&mut self, pinned: bool) -> Vec<CollapsiblePanelEvent> {
        if self.pinned == pinned {
            return Vec::new();
        }
        self.pinned = pinned;
        vec![CollapsiblePanelEvent::PinChanged { pinned }]
    }
}

fn mode_events(from: PanelMode, to: PanelMode) -> Vec<CollapsiblePanelEvent> {
    let mut events = vec![CollapsiblePanelEvent::ModeChanged { from, to }];
    if to == PanelMode::FloatingOverlay {
        events.push(CollapsiblePanelEvent::FloatingShown);
    }
    if from == PanelMode::FloatingOverlay {
        events.push(CollapsiblePanelEvent::FloatingHidden);
    }
    events
}
