use crate::atom::Chip;
use crate::render_model::UiStateId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChipGroupOverflow {
    None,
    Menu,
    ScrollHorizontal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeasuredChip {
    pub(super) chip: Chip,
    pub(super) width: u16,
}

impl MeasuredChip {
    #[must_use]
    pub const fn new(chip: Chip, width: u16) -> Self {
        Self { chip, width }
    }

    #[must_use]
    pub const fn chip(&self) -> &Chip {
        &self.chip
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChipGroupLayout {
    pub(super) visible_chip_ids: Vec<UiStateId>,
    pub(super) hidden_chip_ids: Vec<UiStateId>,
    pub(super) overflow_trigger_visible: bool,
    pub(super) scroll_offset: u16,
}

impl ChipGroupLayout {
    #[must_use]
    pub fn visible_chip_ids(&self) -> &[UiStateId] {
        &self.visible_chip_ids
    }

    #[must_use]
    pub fn hidden_chip_ids(&self) -> &[UiStateId] {
        &self.hidden_chip_ids
    }

    #[must_use]
    pub const fn overflow_trigger_visible(&self) -> bool {
        self.overflow_trigger_visible
    }

    #[must_use]
    pub const fn scroll_offset(&self) -> u16 {
        self.scroll_offset
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChipGroupAction {
    OpenOverflow,
    ScrollHorizontal { offset: u16 },
    Reorder { from: usize, to: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChipGroupFocusTarget {
    Chip(UiStateId),
    PriorFocusHolder,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChipGroupEvent {
    OverflowOpened {
        hidden_chip_ids: Vec<UiStateId>,
    },
    ChipReordered {
        chip_id: UiStateId,
        from: usize,
        to: usize,
    },
    Scrolled {
        offset: u16,
    },
    ChipDismissed {
        chip_id: UiStateId,
        focus_target: ChipGroupFocusTarget,
    },
}
