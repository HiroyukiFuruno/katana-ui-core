use super::{Alignment, Length};
use crate::layout::split_pane_ratio::{
    DEFAULT_HANDLE_WIDTH_PX, DEFAULT_MAX_PERCENT, DEFAULT_MIN_PERCENT, DEFAULT_RATIO_PERCENT,
    interaction_with_ratio, parse_ratio_percent,
};
use crate::render_model::{UiInteractionState, UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitPaneAxis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplitPane {
    pub(super) state_id: UiStateId,
    pub(super) children: Vec<UiNode>,
    gap: Length,
    alignment: Alignment,
    pub(super) interaction: UiInteractionState,
    axis: SplitPaneAxis,
    pub(super) ratio_percent: u8,
    min_percent: u8,
    max_percent: u8,
    handle_width_px: u8,
    pub(super) reset_percent: u8,
}

impl SplitPane {
    #[must_use]
    pub fn new() -> Self {
        Self {
            state_id: UiStateId::next_for(UiNodeKind::SplitPane),
            children: Vec::new(),
            gap: Length::Px(0.0),
            alignment: Alignment::Start,
            interaction: interaction_with_ratio(DEFAULT_RATIO_PERCENT),
            axis: SplitPaneAxis::Horizontal,
            ratio_percent: DEFAULT_RATIO_PERCENT,
            min_percent: DEFAULT_MIN_PERCENT,
            max_percent: DEFAULT_MAX_PERCENT,
            handle_width_px: DEFAULT_HANDLE_WIDTH_PX,
            reset_percent: DEFAULT_RATIO_PERCENT,
        }
    }

    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }

    #[must_use]
    pub fn gap(mut self, gap: Length) -> Self {
        self.gap = gap;
        self
    }

    #[must_use]
    pub fn align(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    #[must_use]
    pub fn axis(mut self, axis: SplitPaneAxis) -> Self {
        self.axis = axis;
        self
    }

    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        let value = value.into();
        if let Some(percent) = parse_ratio_percent(value.as_str()) {
            self.ratio_percent = self.clamped(percent);
        }
        self.interaction.value = value;
        self
    }

    #[must_use]
    pub fn ratio_percent(mut self, percent: u8) -> Self {
        self.set_ratio_percent(percent);
        self
    }

    #[must_use]
    pub fn min_percent(mut self, percent: u8) -> Self {
        self.min_percent = percent.min(self.max_percent);
        self.set_ratio_percent(self.ratio_percent);
        self
    }

    #[must_use]
    pub fn max_percent(mut self, percent: u8) -> Self {
        self.max_percent = percent.max(self.min_percent);
        self.set_ratio_percent(self.ratio_percent);
        self
    }

    #[must_use]
    pub fn handle_width_px(mut self, value: u8) -> Self {
        self.handle_width_px = value;
        self
    }

    #[must_use]
    pub fn reset_percent(mut self, percent: u8) -> Self {
        self.reset_percent = self.clamped(percent);
        self
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub const fn axis_value(&self) -> SplitPaneAxis {
        self.axis
    }

    #[must_use]
    pub const fn ratio_percent_value(&self) -> u8 {
        self.ratio_percent
    }

    #[must_use]
    pub const fn min_percent_value(&self) -> u8 {
        self.min_percent
    }

    #[must_use]
    pub const fn max_percent_value(&self) -> u8 {
        self.max_percent
    }

    #[must_use]
    pub const fn handle_width_px_value(&self) -> u8 {
        self.handle_width_px
    }

    #[must_use]
    pub const fn reset_percent_value(&self) -> u8 {
        self.reset_percent
    }

    #[must_use]
    pub fn children(&self) -> &[UiNode] {
        &self.children
    }

    pub(super) fn set_ratio_percent(&mut self, percent: u8) {
        self.ratio_percent = self.clamped(percent);
        self.interaction.value = self.ratio_percent.to_string();
    }

    fn clamped(&self, percent: u8) -> u8 {
        percent.clamp(self.min_percent, self.max_percent)
    }
}

impl Default for SplitPane {
    fn default() -> Self {
        Self::new()
    }
}

impl From<SplitPane> for UiNode {
    fn from(value: SplitPane) -> Self {
        let mut node = UiNode::from_state(UiNodeKind::SplitPane, "SplitPane", value.state_id)
            .interaction(value.interaction);
        for child in value.children {
            node = node.child(child);
        }
        node
    }
}
