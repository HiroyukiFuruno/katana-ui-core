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
        actions: Vec<ContextMenuAction>,
    ) -> Vec<ContextMenuEvent> {
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
        let events = self.apply_actions(vec![ContextMenuAction::Close { reason }]);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context_menu::types::MENU_PADDING_PX;
    use crate::context_menu::{
        ContextMenuPaintStyle, ContextMenuPresentation, ContextMenuPresentationItem,
        ContextMenuRasterStyle,
    };
    use crate::text_surface::TextSurfaceContextTargetAnchor;
    use katana_ui_core::text_selection::UiTextSelectionRange;
    use katana_ui_core::theme::{FontFamily, FontToken};

    #[test]
    fn closing_an_already_closed_menu_returns_an_empty_frame() {
        let context = egui::Context::default();
        let mut adapter = EguiContextMenuAdapter::default();
        let mut captured = None;
        let mut output = context.run_ui(Default::default(), |ui| {
            captured = Some(adapter.close(ui, ContextMenuCloseReason::Escape));
        });
        output.textures_delta.clear();
        let closed = captured.expect("closed output captured");
        assert!(closed.record.is_none());
        assert!(closed.events.is_empty());
        assert!(closed.artifact.is_none());
    }

    #[test]
    fn closing_an_open_menu_emits_the_requested_close_reason() {
        let context = egui::Context::default();
        let mut adapter = EguiContextMenuAdapter::default();
        let _ = adapter.apply_actions(vec![ContextMenuAction::OpenWithLayout {
            anchor: katana_ui_core::molecule::selection::ContextMenuAnchor::Pointer { x: 0, y: 0 },
            menu_size: ContextMenuSize::new(100, 100),
            viewport: ContextMenuViewport::new(100, 100),
        }]);
        let mut captured = None;
        let mut output = context.run_ui(Default::default(), |ui| {
            captured = Some(adapter.close(ui, ContextMenuCloseReason::Escape));
        });
        output.textures_delta.clear();
        let closed = captured.expect("closed output captured");
        assert!(matches!(
            closed.events.as_slice(),
            [ContextMenuEvent::Closed {
                reason: ContextMenuCloseReason::Escape
            }]
        ));
    }

    #[test]
    fn wheel_scroll_from_raw_input_ignores_zero_and_outside_and_clamps_both_bounds() {
        let context = egui::Context::default();
        let mut adapter = EguiContextMenuAdapter::default();
        let bounds = UiRect::new(0, 0, 100, 20);

        let mut output = context.run_ui(
            egui::RawInput {
                events: vec![
                    egui::Event::PointerMoved(egui::pos2(10.0, 10.0)),
                    egui::Event::MouseWheel {
                        unit: egui::MouseWheelUnit::Point,
                        delta: egui::vec2(0.0, 0.0),
                        modifiers: egui::Modifiers::NONE,
                        phase: egui::TouchPhase::Move,
                    },
                ],
                ..Default::default()
            },
            |ui| {
                adapter.apply_wheel_scroll(ui, bounds, 100);
            },
        );
        output.textures_delta.clear();
        assert_eq!(adapter.vertical_scroll_offset, 0.0);

        adapter.vertical_scroll_offset = 12.0;
        let mut output = context.run_ui(
            egui::RawInput {
                events: vec![
                    egui::Event::PointerMoved(egui::pos2(101.0, 10.0)),
                    egui::Event::MouseWheel {
                        unit: egui::MouseWheelUnit::Point,
                        delta: egui::vec2(0.0, 50.0),
                        modifiers: egui::Modifiers::NONE,
                        phase: egui::TouchPhase::Move,
                    },
                ],
                ..Default::default()
            },
            |ui| {
                adapter.apply_wheel_scroll(ui, bounds, 100);
            },
        );
        output.textures_delta.clear();
        assert_eq!(adapter.vertical_scroll_offset, 12.0);

        let mut output = context.run_ui(
            egui::RawInput {
                events: vec![
                    egui::Event::PointerMoved(egui::pos2(10.0, 10.0)),
                    egui::Event::MouseWheel {
                        unit: egui::MouseWheelUnit::Point,
                        delta: egui::vec2(0.0, 100.0),
                        modifiers: egui::Modifiers::NONE,
                        phase: egui::TouchPhase::Move,
                    },
                ],
                ..Default::default()
            },
            |ui| {
                adapter.apply_wheel_scroll(ui, bounds, 100);
            },
        );
        output.textures_delta.clear();
        assert_eq!(adapter.vertical_scroll_offset, 80.0);

        let mut output = context.run_ui(
            egui::RawInput {
                events: vec![
                    egui::Event::PointerMoved(egui::pos2(10.0, 10.0)),
                    egui::Event::MouseWheel {
                        unit: egui::MouseWheelUnit::Point,
                        delta: egui::vec2(0.0, -100.0),
                        modifiers: egui::Modifiers::NONE,
                        phase: egui::TouchPhase::Move,
                    },
                ],
                ..Default::default()
            },
            |ui| {
                adapter.apply_wheel_scroll(ui, bounds, 100);
            },
        );
        output.textures_delta.clear();
        assert_eq!(adapter.vertical_scroll_offset, 0.0);
    }

    #[test]
    fn keyboard_highlight_reveal_covers_viewport_below_viewport_and_invalid_paths() {
        let mut adapter = EguiContextMenuAdapter::default();
        let bounds = UiRect::new(0, 0, 100, 30);
        adapter.reveal_keyboard_highlight(bounds, 300, 1);

        adapter.synchronize_presentation(ContextMenuPresentation {
            visible: true,
            items: vec![ContextMenuPresentationItem::action("one", "One")],
        });
        let _ = adapter.apply_actions(vec![
            ContextMenuAction::OpenWithLayout {
                anchor: katana_ui_core::molecule::selection::ContextMenuAnchor::Pointer {
                    x: 0,
                    y: 0,
                },
                menu_size: ContextMenuSize::new(100, 100),
                viewport: ContextMenuViewport::new(100, 100),
            },
            ContextMenuAction::Highlight { path: vec![9] },
        ]);
        adapter.reveal_keyboard_highlight(bounds, 300, 1);

        let _ = adapter.apply_actions(vec![ContextMenuAction::Highlight { path: vec![0] }]);
        adapter.vertical_scroll_offset = 0.0;
        adapter.reveal_keyboard_highlight(bounds, 300, 1);
        assert_eq!(
            adapter.vertical_scroll_offset,
            crate::context_menu::types::MENU_PADDING_PX as f32
        );

        adapter.vertical_scroll_offset = 20.0;
        adapter.reveal_keyboard_highlight(bounds, 300, 1);
        assert_eq!(
            adapter.vertical_scroll_offset,
            crate::context_menu::types::MENU_PADDING_PX as f32
        );

        adapter.synchronize_presentation(ContextMenuPresentation {
            visible: true,
            items: vec![
                ContextMenuPresentationItem::action("one", "One"),
                ContextMenuPresentationItem::action("two", "Two"),
                ContextMenuPresentationItem::action("three", "Three"),
            ],
        });
        let _ = adapter.apply_actions(vec![ContextMenuAction::Highlight { path: vec![2] }]);
        adapter.vertical_scroll_offset = 0.0;
        adapter.reveal_keyboard_highlight(bounds, 300, 3);
        assert_eq!(adapter.vertical_scroll_offset, 66.0);

        let _ = adapter.apply_actions(vec![ContextMenuAction::Highlight { path: vec![3] }]);
        adapter.reveal_keyboard_highlight(bounds, 300, 3);
        assert_eq!(adapter.vertical_scroll_offset, 66.0);

        let _ = adapter.apply_actions(vec![ContextMenuAction::OpenSubmenu { path: vec![1] }]);
        let _ = adapter.apply_actions(vec![ContextMenuAction::Highlight { path: vec![0] }]);
        adapter.reveal_keyboard_highlight(bounds, 300, 3);
        assert_eq!(adapter.vertical_scroll_offset, 66.0);
    }

    #[test]
    fn normal_menu_input_reveals_previous_highlight_after_real_scroll() {
        let context = egui::Context::default();
        let mut adapter = EguiContextMenuAdapter::default();
        adapter.synchronize_presentation(ContextMenuPresentation {
            visible: true,
            items: (0..4)
                .map(|index| {
                    ContextMenuPresentationItem::action(
                        format!("item-{index}"),
                        format!("Item {index}"),
                    )
                })
                .collect(),
        });
        adapter.request_open(TextSurfaceContextTargetAnchor::pointer(
            10,
            10,
            UiTextSelectionRange::caret(0),
            UiRect::new(0, 0, 200, 100),
        ));

        let mut output = None;
        crate::run_ui_discard(&context, egui::RawInput::default(), |ui| {
            output = Some(adapter.show(ui, &raster_style(), &paint_style()));
        });
        assert!(output.expect("initial frame captured").is_ok());

        let mut output = None;
        crate::run_ui_discard(
            &context,
            egui::RawInput {
                events: vec![egui::Event::Key {
                    key: egui::Key::ArrowDown,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                }],
                ..Default::default()
            },
            |ui| {
                output = Some(adapter.show(ui, &raster_style(), &paint_style()));
            },
        );
        assert!(output.expect("first highlight frame captured").is_ok());

        let mut output = None;
        crate::run_ui_discard(
            &context,
            egui::RawInput {
                events: vec![
                    egui::Event::PointerMoved(egui::pos2(10.0, 10.0)),
                    egui::Event::MouseWheel {
                        unit: egui::MouseWheelUnit::Point,
                        delta: egui::vec2(0.0, 100.0),
                        modifiers: egui::Modifiers::NONE,
                        phase: egui::TouchPhase::Move,
                    },
                    egui::Event::Key {
                        key: egui::Key::ArrowDown,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
                ..Default::default()
            },
            |ui| {
                output = Some(adapter.show(ui, &raster_style(), &paint_style()));
            },
        );
        assert!(output.expect("scrolled highlight frame captured").is_ok());
        assert!(adapter.vertical_scroll_offset > MENU_PADDING_PX as f32);

        let mut output = None;
        crate::run_ui_discard(
            &context,
            egui::RawInput {
                events: vec![egui::Event::Key {
                    key: egui::Key::Home,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                }],
                ..Default::default()
            },
            |ui| {
                output = Some(adapter.show(ui, &raster_style(), &paint_style()));
            },
        );
        let output = output
            .expect("previous highlight frame captured")
            .expect("normal menu presentation succeeds");
        assert_eq!(
            output.record.expect("visible menu record").highlighted_path,
            vec![0]
        );
        assert_eq!(adapter.vertical_scroll_offset, MENU_PADDING_PX as f32);
    }

    fn raster_style() -> ContextMenuRasterStyle {
        ContextMenuRasterStyle {
            font: FontToken {
                name: "context-menu-state-test".into(),
                family: FontFamily::Proportional,
                size: 15.0,
                weight: 400,
            },
            text_color_rgba: [230, 230, 230, 255],
            icon_color_rgba: [230, 230, 230, 255],
            line_height_px: 22.0,
        }
    }

    const fn paint_style() -> ContextMenuPaintStyle {
        ContextMenuPaintStyle {
            background_rgba: [30, 30, 32, 255],
            highlighted_rgba: [60, 80, 112, 255],
            disabled_rgba: [45, 45, 48, 255],
        }
    }

    #[test]
    fn submenu_path_and_focus_return_are_consumed_by_close_finalization() {
        let context = egui::Context::default();
        let mut adapter = EguiContextMenuAdapter::default();
        let focus_id = egui::Id::new("context-focus-return");
        adapter.focus_return = Some(focus_id);
        let _ = adapter.apply_actions(vec![ContextMenuAction::OpenSubmenu { path: vec![2] }]);
        assert_eq!(adapter.submenu_path, vec![2]);

        let mut output = context.run_ui(Default::default(), |ui| {
            let _ = adapter.finish_closed(ui, Vec::new());
        });
        output.textures_delta.clear();
        assert!(adapter.submenu_path.is_empty());
        assert_eq!(context.memory(|memory| memory.focused()), Some(focus_id));
    }
}
