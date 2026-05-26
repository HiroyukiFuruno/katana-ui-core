use super::panel_scroll_state;

const NAV_SCROLL_Y: u32 = 48;
const PREVIEW_SCROLL_X: u32 = 96;
const PREVIEW_SCROLL_Y: u32 = 72;
const DETAILS_SCROLL_Y: u32 = 36;
const ACTION_SCROLL: u32 = 260;
const MAX_SCROLL_X: u32 = 480;
const MAX_SCROLL_Y: u32 = 380;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PanelChildKey {
    Navigation,
    Preview,
    Details,
}

impl PanelChildKey {
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Navigation => "nav",
            Self::Preview => "preview",
            Self::Details => "details",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PanelOptionControl {
    ActivePanel(PanelChildKey),
    ScrollbarVisible(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PanelChildState {
    pub(super) scroll_x: u32,
    pub(super) scroll_y: u32,
    pub(super) scrollbar_visible: bool,
}

impl PanelChildState {
    const fn new(scroll_x: u32, scroll_y: u32) -> Self {
        Self {
            scroll_x,
            scroll_y,
            scrollbar_visible: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PanelScreenState {
    pub(super) active_panel: PanelChildKey,
    pub(super) navigation: PanelChildState,
    pub(super) preview: PanelChildState,
    pub(super) details: PanelChildState,
}

impl Default for PanelScreenState {
    fn default() -> Self {
        Self {
            active_panel: PanelChildKey::Preview,
            navigation: PanelChildState::new(0, NAV_SCROLL_Y),
            preview: PanelChildState::new(PREVIEW_SCROLL_X, PREVIEW_SCROLL_Y),
            details: PanelChildState::new(0, DETAILS_SCROLL_Y),
        }
    }
}

impl PanelScreenState {
    pub(super) fn child(self, panel: PanelChildKey) -> PanelChildState {
        match panel {
            PanelChildKey::Navigation => self.navigation,
            PanelChildKey::Preview => self.preview,
            PanelChildKey::Details => self.details,
        }
    }

    pub(super) fn apply_preview_action(&mut self) -> PanelScreenUpdate {
        let panel = self.active_panel;
        let state = self.child_mut(panel);
        state.scroll_x = ACTION_SCROLL.min(MAX_SCROLL_X);
        state.scroll_y = ACTION_SCROLL.min(MAX_SCROLL_Y);
        PanelScreenUpdate::new(
            "panel_scroll_preview",
            "panel_scroll_changed",
            "panel.nested_state",
            "advanced",
            "panel_scroll=advanced",
        )
    }

    pub(super) fn apply_option(&mut self, control: PanelOptionControl) -> PanelScreenUpdate {
        match control {
            PanelOptionControl::ActivePanel(panel) => self.select_panel(panel),
            PanelOptionControl::ScrollbarVisible(visible) => self.set_scrollbar_visible(visible),
        }
    }

    pub(super) fn scroll_vertical(&mut self, panel: PanelChildKey, delta_y: f32) -> bool {
        self.active_panel = panel;
        let state = self.child_mut(panel);
        let before = state.scroll_y;
        state.scroll_y = next_scroll(state.scroll_y, MAX_SCROLL_Y, delta_y);
        before != state.scroll_y
    }

    pub(super) fn scroll_horizontal(&mut self, panel: PanelChildKey, delta_x: f32) -> bool {
        self.active_panel = panel;
        let state = self.child_mut(panel);
        let before = state.scroll_x;
        state.scroll_x = next_scroll(state.scroll_x, MAX_SCROLL_X, delta_x);
        before != state.scroll_x
    }

    fn select_panel(&mut self, panel: PanelChildKey) -> PanelScreenUpdate {
        self.active_panel = panel;
        PanelScreenUpdate::new(
            "panel_active_select",
            "panel_active_changed",
            "panel.active",
            panel.label(),
            "active_panel=changed",
        )
    }

    fn set_scrollbar_visible(&mut self, visible: bool) -> PanelScreenUpdate {
        self.child_mut(self.active_panel).scrollbar_visible = visible;
        PanelScreenUpdate::new(
            if visible {
                "panel_scrollbar_show"
            } else {
                "panel_scrollbar_hide"
            },
            "panel_scrollbar_visibility_changed",
            "panel.scrollbar_visibility",
            if visible { "visible" } else { "hidden" },
            if visible {
                "panel_scrollbar=visible"
            } else {
                "panel_scrollbar=hidden"
            },
        )
    }

    fn child_mut(&mut self, panel: PanelChildKey) -> &mut PanelChildState {
        match panel {
            PanelChildKey::Navigation => &mut self.navigation,
            PanelChildKey::Preview => &mut self.preview,
            PanelChildKey::Details => &mut self.details,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PanelScreenUpdate {
    pub(super) action: &'static str,
    pub(super) event: &'static str,
    pub(super) setting: &'static str,
    pub(super) value: &'static str,
    pub(super) state: &'static str,
}

impl PanelScreenUpdate {
    const fn new(
        action: &'static str,
        event: &'static str,
        setting: &'static str,
        value: &'static str,
        state: &'static str,
    ) -> Self {
        Self {
            action,
            event,
            setting,
            value,
            state,
        }
    }
}

fn next_scroll(current: u32, max_scroll: u32, delta: f32) -> u32 {
    panel_scroll_state::PanelScrollRegionModel::next_offset(
        current as usize,
        max_scroll as usize,
        delta,
    ) as u32
}
