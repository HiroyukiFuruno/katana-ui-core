mod types;

use super::{Alignment, Length, OverflowBehavior, SplitPaneOptions};
use crate::layout::split_pane_ratio::{
    DEFAULT_HANDLE_WIDTH_PX, DEFAULT_MAX_PERCENT, DEFAULT_MIN_PERCENT, DEFAULT_RATIO_PERCENT,
    interaction_with_ratio, parse_ratio_percent,
};
use crate::render_model::{
    UiCommonProps, UiDisplay, UiLayoutAxis, UiNode, UiNodeKind, UiSplitPaneAxis,
    UiSplitPaneHandleProps, UiSplitPaneProps, UiSplitPaneResizeMode, UiStateId,
};
pub use types::{SplitPane, SplitPaneAxis, SplitPaneResizeMode};

const SPLIT_PANE_VISIBLE_CHILD_COUNT: usize = 2;
const SPLIT_PANE_HANDLE_MIN_HIT_TARGET_PX: u8 = 24;

impl SplitPane {
    #[must_use]
    pub fn new() -> Self {
        Self {
            state_id: UiStateId::next_for(UiNodeKind::SplitPane),
            children: Vec::new(),
            gap: Length::Px(0.0),
            alignment: Alignment::Start,
            overflow: OverflowBehavior::Fit,
            interaction: interaction_with_ratio(DEFAULT_RATIO_PERCENT),
            axis: SplitPaneAxis::Horizontal,
            ratio_percent: DEFAULT_RATIO_PERCENT,
            min_percent: DEFAULT_MIN_PERCENT,
            max_percent: DEFAULT_MAX_PERCENT,
            handle_width_px: DEFAULT_HANDLE_WIDTH_PX,
            reset_percent: DEFAULT_RATIO_PERCENT,
            resize_mode: SplitPaneResizeMode::PointerAndKeyboard,
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
    pub fn overflow(mut self, overflow: OverflowBehavior) -> Self {
        self.overflow = overflow;
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
    pub fn resize_mode(mut self, value: SplitPaneResizeMode) -> Self {
        self.resize_mode = value;
        self
    }

    #[must_use]
    pub fn options(mut self, options: SplitPaneOptions) -> Self {
        self.axis = options.axis;
        self.min_percent = options.min_percent.min(options.max_percent);
        self.max_percent = options.max_percent.max(self.min_percent);
        self.reset_percent = self.clamped(options.reset_percent);
        self.handle_width_px = options.handle_width_px;
        self.resize_mode = options.resize_mode;
        self.overflow = options.overflow;
        self.set_ratio_percent(options.ratio_percent);
        self
    }

    #[must_use]
    pub const fn options_value(&self) -> SplitPaneOptions {
        SplitPaneOptions {
            axis: self.axis,
            ratio_percent: self.ratio_percent,
            min_percent: self.min_percent,
            max_percent: self.max_percent,
            reset_percent: self.reset_percent,
            handle_width_px: self.handle_width_px,
            resize_mode: self.resize_mode,
            overflow: self.overflow,
        }
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn stable_state_id(mut self, value: impl Into<UiStateId>) -> Self {
        self.state_id = value.into();
        self
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
    pub const fn resize_mode_value(&self) -> SplitPaneResizeMode {
        self.resize_mode
    }

    #[must_use]
    pub const fn overflow_value(&self) -> OverflowBehavior {
        self.overflow
    }

    #[must_use]
    pub fn children(&self) -> &[UiNode] {
        &self.children
    }

    pub(super) fn set_ratio_percent(&mut self, percent: u8) {
        self.ratio_percent = self.clamped(percent);
        self.interaction.value = self.ratio_percent.to_string();
    }

    pub(super) fn clamped(&self, percent: u8) -> u8 {
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
        let child_count = value.children.len();
        let ignored_children = child_count.saturating_sub(SPLIT_PANE_VISIBLE_CHILD_COUNT);
        let mut interaction = value.interaction;
        if ignored_children > 0 {
            interaction.dismiss_reason = format!("ignored_extra_children={ignored_children}");
        }
        let split_pane = UiSplitPaneProps {
            axis: to_render_axis(value.axis),
            ratio_percent: value.ratio_percent,
            min_percent: value.min_percent,
            max_percent: value.max_percent,
            reset_percent: value.reset_percent,
            handle_width_px: value.handle_width_px,
            handle: UiSplitPaneHandleProps {
                width_px: value.handle_width_px,
                focusable: value.resize_mode != SplitPaneResizeMode::Disabled,
                hit_target_px: value
                    .handle_width_px
                    .max(SPLIT_PANE_HANDLE_MIN_HIT_TARGET_PX),
            },
            resize_mode: to_render_resize_mode(value.resize_mode),
        };
        let mut node = UiNode::from_state(UiNodeKind::SplitPane, "SplitPane", value.state_id)
            .common(split_pane_common(
                value.axis,
                value.gap,
                value.alignment,
                value.overflow,
            ))
            .interaction(interaction)
            .split_pane(split_pane);
        for child in value
            .children
            .into_iter()
            .take(SPLIT_PANE_VISIBLE_CHILD_COUNT)
        {
            node = node.child(child);
        }
        node
    }
}

const fn to_render_axis(value: SplitPaneAxis) -> UiSplitPaneAxis {
    match value {
        SplitPaneAxis::Horizontal => UiSplitPaneAxis::Horizontal,
        SplitPaneAxis::Vertical => UiSplitPaneAxis::Vertical,
    }
}

const fn to_render_resize_mode(value: SplitPaneResizeMode) -> UiSplitPaneResizeMode {
    match value {
        SplitPaneResizeMode::PointerOnly => UiSplitPaneResizeMode::PointerOnly,
        SplitPaneResizeMode::KeyboardOnly => UiSplitPaneResizeMode::KeyboardOnly,
        SplitPaneResizeMode::PointerAndKeyboard => UiSplitPaneResizeMode::PointerAndKeyboard,
        SplitPaneResizeMode::Disabled => UiSplitPaneResizeMode::Disabled,
    }
}

fn split_pane_common(
    axis: SplitPaneAxis,
    gap: Length,
    alignment: Alignment,
    overflow: OverflowBehavior,
) -> UiCommonProps {
    UiCommonProps::default()
        .display(UiDisplay::Flex)
        .layout_axis(to_render_layout_axis(axis))
        .gap(gap.into())
        .overflow(overflow.into())
        .align_items(alignment.into())
        .justify_content(alignment.into())
}

const fn to_render_layout_axis(value: SplitPaneAxis) -> UiLayoutAxis {
    match value {
        SplitPaneAxis::Horizontal => UiLayoutAxis::Horizontal,
        SplitPaneAxis::Vertical => UiLayoutAxis::Vertical,
    }
}
