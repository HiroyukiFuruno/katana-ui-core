#[path = "status_bar_parts/interaction.rs"]
mod interaction;
#[path = "status_bar_parts/model.rs"]
mod model;
#[path = "status_bar_parts/render.rs"]
mod render;

use crate::interaction::placement::{
    PlacementConsumer, PlacementEngine, PlacementRequest, PlacementResult,
};
use crate::render_model::{
    UiCommonProps, UiDismissAction, UiNode, UiNodeKind, UiStateId, UiStatusProps, UiTone, UiVariant,
};
pub use interaction::{StatusBarAction, StatusBarEvent, StatusBarState};
pub use model::{
    ProgressMeterShape, ProgressMeterSpec, StatusBarContractViolation, StatusBarDensity,
    StatusBarMode, StatusBarPopoverSpec, StatusBarSegment, StatusBarSegmentAlignment,
};
use render::segment_nodes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusBar {
    label: String,
    state_id: UiStateId,
    common: UiCommonProps,
    status: UiStatusProps,
    children: Vec<UiNode>,
    pub(super) mode: StatusBarMode,
    pub(super) density: StatusBarDensity,
    pub(super) segments: Vec<StatusBarSegment>,
    single_message: Option<String>,
    state: StatusBarState,
}

impl StatusBar {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::StatusBar),
            common: UiCommonProps::default(),
            status: UiStatusProps::default(),
            children: Vec::new(),
            mode: StatusBarMode::SingleMessage,
            density: StatusBarDensity::Default,
            segments: Vec::new(),
            single_message: None,
            state: StatusBarState::default(),
        }
    }

    #[must_use]
    pub fn mode(mut self, value: StatusBarMode) -> Self {
        self.mode = value;
        self
    }

    #[must_use]
    pub fn density(mut self, value: StatusBarDensity) -> Self {
        self.density = value;
        self
    }

    #[must_use]
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.single_message = Some(value.into());
        self
    }

    #[must_use]
    pub fn segment(mut self, value: StatusBarSegment) -> Self {
        self.segments.push(value);
        self
    }

    #[must_use]
    pub fn severity(mut self, value: UiTone) -> Self {
        self.status.severity = value;
        self
    }

    #[must_use]
    pub fn variant(mut self, value: UiVariant) -> Self {
        self.status.variant = value;
        self
    }

    #[must_use]
    pub fn dismiss_action(mut self, value: UiDismissAction) -> Self {
        self.status.dismiss_action = value;
        self
    }

    #[must_use]
    pub fn common(mut self, value: UiCommonProps) -> Self {
        self.common = value;
        self
    }

    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }

    #[must_use]
    pub fn validate(&self) -> Vec<StatusBarContractViolation> {
        match self.mode {
            StatusBarMode::MultiSegment if self.single_message.is_some() => {
                vec![StatusBarContractViolation::MultiSegmentHasSingleMessage]
            }
            StatusBarMode::SingleMessage if !self.segments.is_empty() => {
                vec![StatusBarContractViolation::SingleMessageHasSegments]
            }
            StatusBarMode::SingleMessage | StatusBarMode::MultiSegment => Vec::new(),
        }
    }

    #[must_use]
    pub fn segments_for(&self, alignment: StatusBarSegmentAlignment) -> Vec<&StatusBarSegment> {
        self.segments
            .iter()
            .filter(|segment| segment.alignment == alignment)
            .collect()
    }

    #[must_use]
    pub fn live_region_labels(&self) -> Vec<&str> {
        [
            StatusBarSegmentAlignment::Leading,
            StatusBarSegmentAlignment::Center,
            StatusBarSegmentAlignment::Trailing,
        ]
        .into_iter()
        .flat_map(|alignment| self.segments_for(alignment))
        .map(|segment| segment.accessibility_label.as_str())
        .collect()
    }

    #[must_use]
    pub fn apply_action(&mut self, action: &StatusBarAction) -> Vec<StatusBarEvent> {
        match action {
            StatusBarAction::PressSegment { id } | StatusBarAction::ActivateSegment { id } => {
                self.press_segment(id)
            }
            StatusBarAction::ShowTooltip { id } => {
                vec![StatusBarEvent::SegmentTooltipShown { id: id.clone() }]
            }
            StatusBarAction::ClosePopover { id } => {
                self.state.open_popover = None;
                vec![StatusBarEvent::SegmentPopoverClosed { id: id.clone() }]
            }
            StatusBarAction::Dismiss => vec![StatusBarEvent::Dismissed],
        }
    }

    #[must_use]
    pub const fn state(&self) -> &StatusBarState {
        &self.state
    }

    #[must_use]
    pub fn resolve_popover_placement(
        &self,
        id: &str,
        request: &PlacementRequest,
    ) -> Option<PlacementResult> {
        self.segments
            .iter()
            .any(|segment| segment.id == id && segment.popover.is_some())
            .then(|| PlacementEngine::resolve_for(PlacementConsumer::Popover, request))
    }

    fn press_segment(&mut self, id: &str) -> Vec<StatusBarEvent> {
        let Some(segment) = self.segments.iter().find(|segment| segment.id == id) else {
            return Vec::new();
        };
        if !segment.interactive {
            return Vec::new();
        }
        if segment.popover.is_some() {
            self.state.open_popover = Some(id.to_string());
            return vec![
                StatusBarEvent::SegmentPressed { id: id.to_string() },
                StatusBarEvent::SegmentPopoverOpened { id: id.to_string() },
            ];
        }
        vec![StatusBarEvent::SegmentPressed { id: id.to_string() }]
    }
}

impl From<StatusBar> for UiNode {
    fn from(value: StatusBar) -> Self {
        let segment_nodes = segment_nodes(&value);
        let mut node = UiNode::from_state(UiNodeKind::StatusBar, value.label, value.state_id)
            .common(value.common)
            .status(value.status)
            .size(value.density.into());
        for child in value.children.into_iter().chain(segment_nodes) {
            node = node.child(child);
        }
        node
    }
}
