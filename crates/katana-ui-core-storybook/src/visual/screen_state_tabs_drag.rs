use super::screen_state_tabs_core::core_event_name;
use super::screen_state_tabs_types::{TabsScreenState, TabsScreenUpdate, tabs_update};
use katana_ui_core::widget::molecules::{CloseableTabId, CloseableTabStripAction};

impl TabsScreenState {
    pub(in crate::visual) fn start_drag_tab(&mut self, tab_id: &str) -> TabsScreenUpdate {
        let events = self.apply_core_tab_action(CloseableTabStripAction::StartDrag {
            tab_id: CloseableTabId::new(tab_id),
        });
        tabs_update(
            "tab_drag_start",
            core_event_name(&events, "closeable_tab_drag_missing"),
            "tabs.drag",
            "start",
            "tabs.dragging=true",
        )
    }

    pub(in crate::visual) fn drag_tab_to_visual_index(
        &mut self,
        tab_id: &str,
        to_visual_index: usize,
    ) -> TabsScreenUpdate {
        let events = self.apply_core_tab_action(CloseableTabStripAction::MoveTab {
            tab_id: CloseableTabId::new(tab_id),
            to_visual_index,
        });
        tabs_update(
            "tab_drag_move",
            core_event_name(&events, "closeable_tab_move_blocked"),
            "tabs.drag",
            "move",
            "tabs.order=changed",
        )
    }

    pub(in crate::visual) fn end_drag_tab(
        &mut self,
        tab_id: &str,
        committed: bool,
    ) -> TabsScreenUpdate {
        let events = self.apply_core_tab_drag_end(CloseableTabId::new(tab_id), committed);
        let value = if committed { "committed" } else { "cancelled" };
        tabs_update(
            "tab_drag_end",
            core_event_name(&events, "closeable_tab_drag_missing"),
            "tabs.drag",
            value,
            "tabs.dragging=false",
        )
    }
}
