use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::layout::{SplitPane, SplitPaneResizeSource};
use katana_ui_core::render_model::{UiNode, UiStateId};

use super::StorybookWindowState;
use crate::visual::{dedicated_dod_molecule_split_pane, preview_detail};

const STATE_ID: &str = "split-pane.storybook";
const DEFAULT_RATIO: u8 = 50;
const MIN_RATIO: u8 = 20;
const MAX_RATIO: u8 = 80;
const RESET_RATIO: u8 = 55;
const POINTER_DELTA: i8 = 14;
const KEYBOARD_DELTA: i8 = 8;
const RESIZE_DELTA: i8 = -10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum SplitPaneStoryAction {
    Drag,
    Focus,
    Hover,
    Keyboard,
    Resize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::visual) struct SplitPaneStoryState {
    ratio_percent: u8,
    focused: bool,
    hovered: bool,
    dragging: bool,
    resized: bool,
    callback: &'static str,
}

impl Default for SplitPaneStoryState {
    fn default() -> Self {
        Self {
            ratio_percent: DEFAULT_RATIO,
            focused: false,
            hovered: false,
            dragging: false,
            resized: false,
            callback: "callback=idle",
        }
    }
}

impl SplitPaneStoryState {
    pub(in crate::visual) fn apply_action(
        &mut self,
        action: SplitPaneStoryAction,
    ) -> SplitPaneStoryUpdate {
        match action {
            SplitPaneStoryAction::Drag => {
                self.dragging = true;
                self.apply_core_resize(
                    UiAction::SplitPaneResizeBy {
                        target: state_id(),
                        delta_percent: POINTER_DELTA,
                        source: SplitPaneResizeSource::Pointer,
                    },
                    "split_pane_drag_resize",
                    "split_pane_ratio_changed",
                    "ratio=64",
                )
            }
            SplitPaneStoryAction::Keyboard => {
                if !self.focused {
                    return SplitPaneStoryUpdate::new(
                        "split_pane_keyboard_without_focus",
                        "split_pane_keyboard_ignored",
                        "focused=false",
                    );
                }
                self.apply_core_resize(
                    UiAction::SplitPaneResizeBy {
                        target: state_id(),
                        delta_percent: KEYBOARD_DELTA,
                        source: SplitPaneResizeSource::Keyboard,
                    },
                    "split_pane_keyboard_resize",
                    "split_pane_ratio_changed",
                    "keyboard=58",
                )
            }
            SplitPaneStoryAction::Resize => {
                self.resized = true;
                self.apply_core_resize(
                    UiAction::SplitPaneResizeBy {
                        target: state_id(),
                        delta_percent: RESIZE_DELTA,
                        source: SplitPaneResizeSource::Pointer,
                    },
                    "split_pane_resize",
                    "split_pane_ratio_changed",
                    "resize=40",
                )
            }
            SplitPaneStoryAction::Focus => {
                let node: UiNode = split_pane().into();
                assert!(
                    node.props().split_pane.handle.focusable,
                    "core SplitPane handle must be focusable when resize is enabled"
                );
                self.focused = true;
                self.callback = "callback=focus";
                SplitPaneStoryUpdate::new("split_pane_focus", "focus", "focus=handle")
            }
            SplitPaneStoryAction::Hover => {
                let mut pane = split_pane().ratio_percent(self.ratio_percent);
                let result = pane.apply_action(&UiAction::hover(state_id(), true));
                debug_assert!(result.handled);
                self.hovered = true;
                self.callback = "callback=split_pane";
                SplitPaneStoryUpdate::new("split_pane_hover", "hover_start", "hover=handle")
            }
        }
    }

    pub(in crate::visual) const fn ratio_percent(&self) -> u8 {
        self.ratio_percent
    }

    pub(in crate::visual) const fn focused(&self) -> bool {
        self.focused
    }

    pub(in crate::visual) const fn hovered(&self) -> bool {
        self.hovered
    }

    pub(in crate::visual) const fn dragging(&self) -> bool {
        self.dragging
    }

    pub(in crate::visual) const fn resized(&self) -> bool {
        self.resized
    }

    pub(in crate::visual) const fn callback(&self) -> &'static str {
        self.callback
    }

    fn apply_core_resize(
        &mut self,
        action: UiAction,
        action_label: &'static str,
        event_label: &'static str,
        state_label: &'static str,
    ) -> SplitPaneStoryUpdate {
        let mut pane = split_pane().ratio_percent(self.ratio_percent);
        let result = pane.apply_action(&action);
        debug_assert!(result.handled);
        self.ratio_percent = pane.ratio_percent_value();
        self.callback = "callback=split_pane";
        SplitPaneStoryUpdate::new(action_label, event_label, state_label)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct SplitPaneStoryUpdate {
    pub(in crate::visual) action: &'static str,
    pub(in crate::visual) event: &'static str,
    pub(in crate::visual) state: &'static str,
}

impl SplitPaneStoryUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}

fn split_pane() -> SplitPane {
    SplitPane::new()
        .stable_state_id(state_id())
        .ratio_percent(DEFAULT_RATIO)
        .min_percent(MIN_RATIO)
        .max_percent(MAX_RATIO)
        .reset_percent(RESET_RATIO)
}

fn state_id() -> UiStateId {
    STATE_ID.into()
}

pub(super) fn operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<SplitPaneStoryAction> {
    if state.selected_page != "split-pane" {
        return None;
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    if dedicated_dod_molecule_split_pane::resize_handle_rect(origin.x, origin.y).contains(x, y) {
        return Some(SplitPaneStoryAction::Resize);
    }
    if dedicated_dod_molecule_split_pane::handle_drag_rect(origin.x, origin.y).contains(x, y) {
        return Some(SplitPaneStoryAction::Drag);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyboard_resize_requires_focus() {
        let mut state = SplitPaneStoryState::default();
        let update = state.apply_action(SplitPaneStoryAction::Keyboard);
        assert_eq!("split_pane_keyboard_without_focus", update.action);
        assert_eq!("focused=false", update.state);
    }
}
