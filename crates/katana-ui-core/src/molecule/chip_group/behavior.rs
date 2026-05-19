use super::{
    ChipGroup, ChipGroupAction, ChipGroupEvent, ChipGroupFocusTarget, ChipGroupLayout,
    ChipGroupOverflow, MeasuredChip,
};
use crate::atom::{ChipAction, ChipEvent, ChipKeyboardInput, Text};
use crate::molecule::Menu;

impl ChipGroup {
    #[must_use]
    pub fn layout(&self) -> ChipGroupLayout {
        match self.overflow {
            ChipGroupOverflow::Menu if !self.wrap => self.menu_layout(),
            ChipGroupOverflow::ScrollHorizontal => self.scroll_layout(),
            ChipGroupOverflow::None | ChipGroupOverflow::Menu => self.all_visible_layout(false),
        }
    }

    #[must_use]
    pub fn overflow_menu(&self) -> Option<Menu> {
        let layout = self.layout();
        if !layout.overflow_trigger_visible {
            return None;
        }
        let mut menu = Menu::new(format!("{} hidden chips", layout.hidden_chip_ids.len()));
        for chip_id in &layout.hidden_chip_ids {
            if let Some(item) = self.chips.iter().find(|it| it.chip.state_id() == chip_id) {
                menu = menu.child(Text::new(item.chip.label()));
            }
        }
        Some(menu)
    }

    #[must_use]
    pub fn apply_action(&mut self, action: ChipGroupAction) -> Vec<ChipGroupEvent> {
        match action {
            ChipGroupAction::OpenOverflow => self.open_overflow(),
            ChipGroupAction::ScrollHorizontal { offset } => self.scroll_horizontal(offset),
            ChipGroupAction::Reorder { from, to } => self.reorder_chip(from, to),
        }
    }

    #[must_use]
    pub fn dismiss_focused_with_keyboard(
        &mut self,
        input: ChipKeyboardInput,
    ) -> Vec<ChipGroupEvent> {
        let Some(index) = self.chips.iter().position(|it| it.chip.focused_value()) else {
            return Vec::new();
        };
        let chip_events = self.chips[index]
            .chip
            .apply_action(ChipAction::Keyboard(input));
        if !contains_chip_dismiss(&chip_events) {
            return Vec::new();
        }
        let chip_id = self.chips.remove(index).chip.state_id().clone();
        let focus_target = self.focus_after_removal(index);
        self.record_event(ChipGroupEvent::ChipDismissed {
            chip_id,
            focus_target,
        })
    }

    fn menu_layout(&self) -> ChipGroupLayout {
        if measured_width(&self.chips, self.gap) <= self.available_width {
            return self.all_visible_layout(false);
        }
        let budget = self
            .available_width
            .saturating_sub(self.overflow_trigger_width);
        let mut used = 0;
        let mut visible = Vec::new();
        let mut hidden = Vec::new();
        for item in &self.chips {
            let next = used + gap_before(visible.is_empty(), self.gap) + item.width;
            if next <= budget {
                used = next;
                visible.push(item.chip.state_id().clone());
            } else {
                hidden.push(item.chip.state_id().clone());
            }
        }
        ChipGroupLayout {
            visible_chip_ids: visible,
            overflow_trigger_visible: !hidden.is_empty(),
            hidden_chip_ids: hidden,
            scroll_offset: 0,
        }
    }

    fn scroll_layout(&self) -> ChipGroupLayout {
        let mut layout = self.all_visible_layout(false);
        layout.scroll_offset = self.scroll_offset;
        layout
    }

    fn all_visible_layout(&self, overflow_trigger_visible: bool) -> ChipGroupLayout {
        ChipGroupLayout {
            visible_chip_ids: self
                .chips
                .iter()
                .map(|item| item.chip.state_id().clone())
                .collect(),
            hidden_chip_ids: Vec::new(),
            overflow_trigger_visible,
            scroll_offset: self.scroll_offset,
        }
    }

    fn open_overflow(&mut self) -> Vec<ChipGroupEvent> {
        let layout = self.layout();
        if self.overflow != ChipGroupOverflow::Menu || layout.hidden_chip_ids.is_empty() {
            return Vec::new();
        }
        self.overflow_open = true;
        self.record_event(ChipGroupEvent::OverflowOpened {
            hidden_chip_ids: layout.hidden_chip_ids,
        })
    }

    fn scroll_horizontal(&mut self, offset: u16) -> Vec<ChipGroupEvent> {
        if self.overflow != ChipGroupOverflow::ScrollHorizontal {
            return Vec::new();
        }
        self.scroll_offset = offset;
        self.record_event(ChipGroupEvent::Scrolled { offset })
    }

    fn reorder_chip(&mut self, from: usize, to: usize) -> Vec<ChipGroupEvent> {
        if !self.reorder || from >= self.chips.len() || to >= self.chips.len() || from == to {
            return Vec::new();
        }
        let item = self.chips.remove(from);
        let chip_id = item.chip.state_id().clone();
        self.chips.insert(to, item);
        self.record_event(ChipGroupEvent::ChipReordered { chip_id, from, to })
    }

    fn focus_after_removal(&mut self, removed_index: usize) -> ChipGroupFocusTarget {
        if self.chips.is_empty() {
            return ChipGroupFocusTarget::PriorFocusHolder;
        }
        let index = removed_index.saturating_sub(1).min(self.chips.len() - 1);
        let measured = self.chips.remove(index);
        let focus_id = measured.chip.state_id().clone();
        let chip = measured.chip.focused(true);
        self.chips
            .insert(index, MeasuredChip::new(chip, measured.width));
        ChipGroupFocusTarget::Chip(focus_id)
    }

    fn record_event(&mut self, event: ChipGroupEvent) -> Vec<ChipGroupEvent> {
        self.callback_log.push(event.clone());
        vec![event]
    }
}

fn measured_width(items: &[MeasuredChip], gap: u16) -> u16 {
    let mut width = 0u16;
    for item in items {
        width = width
            .saturating_add(gap_before(width == 0, gap))
            .saturating_add(item.width);
    }
    width
}

fn gap_before(is_first: bool, gap: u16) -> u16 {
    if is_first { 0 } else { gap }
}

fn contains_chip_dismiss(events: &[ChipEvent]) -> bool {
    events
        .iter()
        .any(|event| matches!(event, ChipEvent::ChipDismissed { .. }))
}
