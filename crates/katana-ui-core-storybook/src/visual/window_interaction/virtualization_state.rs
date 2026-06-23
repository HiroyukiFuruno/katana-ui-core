use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{
    RowHeightProvider, UiAction, VirtualRange, VirtualizationConfig,
};
use katana_ui_core::molecule::VirtualizedList;

const TOTAL_ROWS: usize = 10_000;
const ROW_HEIGHT: u32 = 20;
const VIEWPORT_HEIGHT: u32 = 100;
const SCROLL_OFFSET: u32 = 1_260;
const KEYBOARD_FOCUS_INDEX: usize = 42;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum VirtualizationStoryAction {
    Scroll,
    Hover,
    Focus,
    KeyboardFocus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::visual) struct VirtualizationScreenState {
    list: VirtualizedList,
    pub(in crate::visual) hovered: bool,
    pub(in crate::visual) focused: bool,
    pub(in crate::visual) range: VirtualRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct VirtualizationUpdate {
    pub(in crate::visual) action: &'static str,
    pub(in crate::visual) event: &'static str,
    pub(in crate::visual) state: &'static str,
}

impl Default for VirtualizationScreenState {
    fn default() -> Self {
        let list = VirtualizedList::new("Virtualized list", virtualization_config());
        let range = list.visible_range();
        Self {
            list,
            hovered: false,
            focused: false,
            range,
        }
    }
}

impl VirtualizationScreenState {
    pub(in crate::visual) fn apply(
        &mut self,
        action: VirtualizationStoryAction,
    ) -> VirtualizationUpdate {
        match action {
            VirtualizationStoryAction::Scroll => self.scroll(),
            VirtualizationStoryAction::Hover => self.hover(),
            VirtualizationStoryAction::Focus => self.focus(),
            VirtualizationStoryAction::KeyboardFocus => self.keyboard_focus(),
        }
    }

    fn scroll(&mut self) -> VirtualizationUpdate {
        let action = UiAction::set_value(self.list.state_id().clone(), SCROLL_OFFSET.to_string());
        self.list.apply_action(&action);
        self.range = self.list.visible_range();
        VirtualizationUpdate::new(
            "virtualized_scroll",
            "virtual_range_changed",
            "rows=visible",
        )
    }

    fn hover(&mut self) -> VirtualizationUpdate {
        self.hovered = true;
        VirtualizationUpdate::new("virtualized_hover", "hover_start", "hover=viewport")
    }

    fn focus(&mut self) -> VirtualizationUpdate {
        self.focused = true;
        self.apply_focus(KEYBOARD_FOCUS_INDEX);
        VirtualizationUpdate::new("virtualized_focus", "virtualized_focus_kept", "focus=42")
    }

    fn keyboard_focus(&mut self) -> VirtualizationUpdate {
        self.apply_focus(KEYBOARD_FOCUS_INDEX + 1);
        VirtualizationUpdate::new(
            "virtualized_keyboard_focus",
            "virtualized_focus_kept",
            "focus=43",
        )
    }

    fn apply_focus(&mut self, index: usize) {
        let action = UiAction::set_selected_index(self.list.state_id().clone(), index);
        self.list.apply_action(&action);
        self.range = self.list.visible_range();
    }
}

impl VirtualizationUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}

fn virtualization_config() -> VirtualizationConfig {
    VirtualizationConfig {
        enabled: true,
        total_count: TOTAL_ROWS,
        viewport_offset: 0,
        viewport_height: VIEWPORT_HEIGHT,
        overscan: 2,
        row_height_provider: RowHeightProvider::Fixed { height: ROW_HEIGHT },
        keep_focused_in_window: true,
        focused_index: None,
    }
}
