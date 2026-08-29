use super::interaction::collect_events;
use super::surface::{contains, local_anchor};
use super::types::{EguiContextMenuAdapter, EguiContextMenuOutput};
use crate::text_surface::TextSurfaceContextTargetAnchor;
use katana_ui_core::molecule::selection::{
    ContextMenuAction, ContextMenuCloseReason, ContextMenuEvent, ContextMenuSize,
    ContextMenuViewport,
};
use katana_ui_core::render_model::UiRect;

impl EguiContextMenuAdapter {
    pub(super) fn open_if_needed(
        &mut self,
        ui: &egui::Ui,
        anchor: &TextSurfaceContextTargetAnchor,
        width: u32,
        height: u32,
    ) -> Vec<ContextMenuEvent> {
        if self.menu.is_open() {
            return Vec::new();
        }
        self.focus_return = ui.ctx().memory(|memory| memory.focused());
        let viewport = anchor.viewport_bounds();
        collect_events(
            &mut self.menu,
            [ContextMenuAction::OpenWithLayout {
                anchor: local_anchor(anchor, viewport),
                menu_size: ContextMenuSize::new(width, height),
                viewport: ContextMenuViewport::new(viewport.width, viewport.height),
            }],
        )
    }

    pub(super) fn apply_actions(
        &mut self,
        actions: impl IntoIterator<Item = ContextMenuAction>,
    ) -> Vec<ContextMenuEvent> {
        let actions = actions.into_iter().collect::<Vec<_>>();
        for action in &actions {
            if let ContextMenuAction::OpenSubmenu { path } = action {
                self.submenu_path = path.clone();
            }
        }
        collect_events(&mut self.menu, actions)
    }

    pub(super) fn close(
        &mut self,
        ui: &egui::Ui,
        reason: ContextMenuCloseReason,
    ) -> EguiContextMenuOutput {
        if !self.menu.is_open() {
            return EguiContextMenuOutput {
                record: None,
                events: Vec::new(),
                artifact: None,
            };
        }
        let events = self.apply_actions([ContextMenuAction::Close { reason }]);
        self.finish_closed(ui, events)
    }

    pub(super) fn finish_closed(
        &mut self,
        ui: &egui::Ui,
        events: Vec<ContextMenuEvent>,
    ) -> EguiContextMenuOutput {
        if let Some(id) = self.focus_return.take() {
            ui.ctx().memory_mut(|memory| memory.request_focus(id));
        }
        self.submenu_path.clear();
        self.reset_scroll();
        self.anchor = None;
        EguiContextMenuOutput {
            record: None,
            events,
            artifact: None,
        }
    }

    pub(super) fn reset_scroll(&mut self) {
        self.scroll_path.clear();
        self.vertical_scroll_offset = 0.0;
    }

    pub(super) fn reset_scroll_for_current_path(&mut self) {
        if self.scroll_path != self.submenu_path {
            self.scroll_path = self.submenu_path.clone();
            self.vertical_scroll_offset = 0.0;
        }
    }

    pub(super) fn apply_wheel_scroll(
        &mut self,
        ui: &egui::Ui,
        bounds: UiRect,
        content_height: u32,
    ) {
        let delta = ui.input(|input| {
            input
                .events
                .iter()
                .filter_map(|event| match event {
                    egui::Event::MouseWheel { delta, .. } => Some(delta.y),
                    _ => None,
                })
                .sum::<f32>()
        });
        let hovered = ui.input(|input| input.pointer.hover_pos());
        if delta == 0.0 || !hovered.is_some_and(|point| contains(bounds, point)) {
            return;
        }
        let maximum = content_height.saturating_sub(bounds.height) as f32;
        self.vertical_scroll_offset = (self.vertical_scroll_offset + delta).clamp(0.0, maximum);
    }

    pub(super) fn reveal_keyboard_highlight(
        &mut self,
        bounds: UiRect,
        content_height: u32,
        item_count: usize,
    ) {
        let highlighted_path = self.menu.current_highlighted_path();
        let Some(index) = highlighted_path.last().copied() else {
            return;
        };
        if index >= item_count
            || highlighted_path.len() != self.submenu_path.len().saturating_add(1)
            || !highlighted_path.starts_with(&self.submenu_path)
        {
            return;
        }

        let row_top = super::types::MENU_PADDING_PX.saturating_add(
            u32::try_from(index)
                .unwrap_or(u32::MAX)
                .saturating_mul(super::types::ROW_HEIGHT_PX),
        ) as f32;
        let row_bottom = row_top + super::types::ROW_HEIGHT_PX as f32;
        let viewport_bottom = self.vertical_scroll_offset + bounds.height as f32;
        let next_offset = if row_top < self.vertical_scroll_offset {
            row_top
        } else if row_bottom > viewport_bottom {
            row_bottom - bounds.height as f32
        } else {
            self.vertical_scroll_offset
        };
        let maximum = content_height.saturating_sub(bounds.height) as f32;
        self.vertical_scroll_offset = next_offset.clamp(0.0, maximum);
    }
}
