use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::layout::{ScrollArea, ScrollAxis};
use katana_ui_core::render_model::{UiScrollbarVisibility, UiStateId};

use super::StorybookWindowState;
use crate::visual::{dedicated_dod_layout_scroll_area, preview_detail};

const STATE_ID: &str = "scroll-area.storybook";
const VIEWPORT_WIDTH: u32 = 120;
const VIEWPORT_HEIGHT: u32 = 72;
const CONTENT_WIDTH: u32 = 220;
const CONTENT_HEIGHT: u32 = 180;
const WHEEL_DY: i32 = 48;
const DRAG_DY: i32 = 72;
const KEYBOARD_DY: i32 = 36;
const RESIZE_VIEWPORT_HEIGHT: u32 = 48;
const EDGE_THRESHOLD: u32 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum ScrollAreaStoryAction {
    Scroll,
    Drag,
    Focus,
    Hover,
    Keyboard,
    Resize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::visual) struct ScrollAreaStoryState {
    offset_x: u32,
    offset_y: u32,
    focused: bool,
    hovered: bool,
    dragging: bool,
    resized: bool,
    callback: &'static str,
}

impl Default for ScrollAreaStoryState {
    fn default() -> Self {
        Self {
            offset_x: 0,
            offset_y: 0,
            focused: false,
            hovered: false,
            dragging: false,
            resized: false,
            callback: "callback=idle",
        }
    }
}

impl ScrollAreaStoryState {
    pub(in crate::visual) fn apply_action(
        &mut self,
        action: ScrollAreaStoryAction,
    ) -> ScrollAreaStoryUpdate {
        match action {
            ScrollAreaStoryAction::Scroll => self.apply_core_scroll(
                UiAction::ScrollBy {
                    target: state_id(),
                    dx: 0,
                    dy: WHEEL_DY,
                },
                "scroll_area_scroll",
                "scroll=48",
            ),
            ScrollAreaStoryAction::Drag => {
                self.dragging = true;
                self.apply_core_scroll(
                    UiAction::ScrollBy {
                        target: state_id(),
                        dx: 0,
                        dy: DRAG_DY,
                    },
                    "scroll_area_drag_thumb",
                    "drag=72",
                )
            }
            ScrollAreaStoryAction::Keyboard => {
                if !self.focused {
                    return ScrollAreaStoryUpdate::new(
                        "scroll_area_keyboard_without_focus",
                        "scroll_area_keyboard_ignored",
                        "focused=false",
                    );
                }
                self.apply_core_scroll(
                    UiAction::ScrollBy {
                        target: state_id(),
                        dx: 0,
                        dy: KEYBOARD_DY,
                    },
                    "scroll_area_keyboard_scroll",
                    "keyboard=36",
                )
            }
            ScrollAreaStoryAction::Resize => {
                self.resized = true;
                let mut area = scroll_area()
                    .viewport(VIEWPORT_WIDTH, RESIZE_VIEWPORT_HEIGHT)
                    .offset(self.offset_x, self.offset_y);
                let _ = area.apply_action(&UiAction::SetScrollbarVisibility {
                    target: state_id(),
                    visibility: UiScrollbarVisibility::Always,
                });
                self.offset_x = area.offset_x();
                self.offset_y = area.offset_y();
                self.callback = "callback=scroll_area";
                ScrollAreaStoryUpdate::new(
                    "scrollbar_visibility_changed",
                    "scroll_area_resized",
                    "resize=viewport",
                )
            }
            ScrollAreaStoryAction::Focus => {
                let _ = scroll_area().apply_action(&UiAction::focus(state_id()));
                self.focused = true;
                self.callback = "callback=focus";
                ScrollAreaStoryUpdate::new("scroll_area_focus", "focus", "focus=viewport")
            }
            ScrollAreaStoryAction::Hover => {
                let _ = scroll_area().apply_action(&UiAction::hover(state_id(), true));
                self.hovered = true;
                self.callback = "callback=hover";
                ScrollAreaStoryUpdate::new("scroll_area_hover", "hover_start", "hover=viewport")
            }
        }
    }

    pub(in crate::visual) const fn offset_y(&self) -> u32 {
        self.offset_y
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

    fn apply_core_scroll(
        &mut self,
        action: UiAction,
        action_label: &'static str,
        state_label: &'static str,
    ) -> ScrollAreaStoryUpdate {
        let mut area = scroll_area().offset(self.offset_x, self.offset_y);
        let _ = area.apply_action(&action);
        self.offset_x = area.offset_x();
        self.offset_y = area.offset_y();
        self.callback = "callback=scroll_area";
        ScrollAreaStoryUpdate::new(action_label, "scroll_area_scrolled", state_label)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct ScrollAreaStoryUpdate {
    pub(in crate::visual) action: &'static str,
    pub(in crate::visual) event: &'static str,
    pub(in crate::visual) state: &'static str,
}

impl ScrollAreaStoryUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}

fn scroll_area() -> ScrollArea {
    ScrollArea::new()
        .stable_state_id(state_id())
        .axis(ScrollAxis::Both)
        .viewport(VIEWPORT_WIDTH, VIEWPORT_HEIGHT)
        .content_extent(CONTENT_WIDTH, CONTENT_HEIGHT)
        .edge_threshold(EDGE_THRESHOLD)
}

fn state_id() -> UiStateId {
    STATE_ID.into()
}

pub(super) fn operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<ScrollAreaStoryAction> {
    if state.selected_page != "scroll-area" {
        return None;
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    if dedicated_dod_layout_scroll_area::resize_handle_rect(origin.x, origin.y).contains(x, y) {
        return Some(ScrollAreaStoryAction::Resize);
    }
    if dedicated_dod_layout_scroll_area::scrollbar_drag_rect(origin.x, origin.y).contains(x, y) {
        return Some(ScrollAreaStoryAction::Drag);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyboard_requires_focus_and_scroll_clamps_at_the_content_boundary() {
        let mut state = ScrollAreaStoryState::default();

        let ignored = state.apply_action(ScrollAreaStoryAction::Keyboard);
        assert_eq!("scroll_area_keyboard_without_focus", ignored.action);
        assert_eq!("focused=false", ignored.state);

        for _ in 0..4 {
            let _ = state.apply_action(ScrollAreaStoryAction::Scroll);
        }
        assert_eq!("callback=scroll_area", state.callback());
        assert_eq!(CONTENT_HEIGHT - VIEWPORT_HEIGHT, state.offset_y());
    }
}
