use super::{ChipGroupEvent, ChipGroupOverflow, MeasuredChip};
use crate::atom::Chip;
use crate::render_model::{UiInteractionState, UiNode, UiNodeKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChipGroup {
    pub(super) label: String,
    pub(super) chips: Vec<MeasuredChip>,
    pub(super) overflow: ChipGroupOverflow,
    pub(super) wrap: bool,
    pub(super) reorder: bool,
    pub(super) gap: u16,
    pub(super) available_width: u16,
    pub(super) overflow_trigger_width: u16,
    pub(super) scroll_offset: u16,
    pub(super) overflow_open: bool,
    pub(super) callback_log: Vec<ChipGroupEvent>,
}

impl From<ChipGroup> for UiNode {
    fn from(value: ChipGroup) -> Self {
        let layout = value.layout();
        let mut node =
            UiNode::new(UiNodeKind::ChipGroup, value.label).interaction(UiInteractionState {
                open: value.overflow_open,
                item_count: value.chips.len(),
                ..UiInteractionState::default()
            });
        for item in value.chips {
            if layout.visible_chip_ids().contains(item.chip.state_id()) {
                node = node.child(item.chip);
            }
        }
        if layout.overflow_trigger_visible() {
            node = node.child(Chip::new(format!("+{}", layout.hidden_chip_ids().len())));
        }
        node
    }
}

impl ChipGroup {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            chips: Vec::new(),
            overflow: ChipGroupOverflow::None,
            wrap: true,
            reorder: false,
            gap: 0,
            available_width: u16::MAX,
            overflow_trigger_width: 0,
            scroll_offset: 0,
            overflow_open: false,
            callback_log: Vec::new(),
        }
    }

    #[must_use]
    pub fn chip(mut self, chip: Chip, width: u16) -> Self {
        self.chips.push(MeasuredChip::new(chip, width));
        self
    }

    #[must_use]
    pub const fn overflow(mut self, value: ChipGroupOverflow) -> Self {
        self.overflow = value;
        self
    }

    #[must_use]
    pub const fn wrap(mut self, value: bool) -> Self {
        self.wrap = value;
        self
    }

    #[must_use]
    pub const fn reorder(mut self, value: bool) -> Self {
        self.reorder = value;
        self
    }

    #[must_use]
    pub const fn gap(mut self, value: u16) -> Self {
        self.gap = value;
        self
    }

    #[must_use]
    pub const fn available_width(mut self, value: u16) -> Self {
        self.available_width = value;
        self
    }

    #[must_use]
    pub const fn overflow_trigger_width(mut self, value: u16) -> Self {
        self.overflow_trigger_width = value;
        self
    }

    #[must_use]
    pub fn chips(&self) -> &[MeasuredChip] {
        &self.chips
    }

    #[must_use]
    pub const fn overflow_open(&self) -> bool {
        self.overflow_open
    }

    #[must_use]
    pub fn callback_log(&self) -> &[ChipGroupEvent] {
        &self.callback_log
    }
}
