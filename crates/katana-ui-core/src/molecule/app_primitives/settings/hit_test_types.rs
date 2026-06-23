use super::SettingsListAction;
use crate::render_model::{UiCursor, UiNodeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingsListHitTestInput {
    pub pointer_x: u32,
    pub pointer_y: u32,
    pub scroll_offset_y: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsListHitTestResult {
    Field { field_id: String },
    ToggleSection { section_id: String },
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingsListHitRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SettingsListHitTarget {
    pub rect: SettingsListHitRect,
    pub result: SettingsListHitTestResult,
    pub cursor: UiCursor,
    pub hover_node_id: Option<UiNodeId>,
    pub hover_action: Option<SettingsListAction>,
    pub action: Option<SettingsListAction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SettingsListInteraction {
    pub result: SettingsListHitTestResult,
    pub target: Option<SettingsListHitTarget>,
    pub cursor: UiCursor,
    pub hover_node_id: Option<UiNodeId>,
    pub hover_action: Option<SettingsListAction>,
    pub action: Option<SettingsListAction>,
}

impl SettingsListHitRect {
    pub(super) fn contains(&self, pointer_x: u32, absolute_y: u32) -> bool {
        pointer_x >= self.x
            && pointer_x < self.x.saturating_add(self.width)
            && absolute_y >= self.y
            && absolute_y < self.y.saturating_add(self.height)
    }
}

impl SettingsListInteraction {
    pub(super) fn none() -> Self {
        Self {
            result: SettingsListHitTestResult::None,
            target: None,
            cursor: UiCursor::Default,
            hover_node_id: None,
            hover_action: None,
            action: None,
        }
    }

    pub(super) fn from_result_and_target(
        result: SettingsListHitTestResult,
        target: Option<SettingsListHitTarget>,
    ) -> Self {
        Self {
            result,
            cursor: target
                .as_ref()
                .map(|target| target.cursor)
                .unwrap_or(UiCursor::Default),
            hover_node_id: target
                .as_ref()
                .and_then(|target| target.hover_node_id.clone()),
            hover_action: target
                .as_ref()
                .and_then(|target| target.hover_action.clone()),
            action: target.as_ref().and_then(|target| target.action.clone()),
            target,
        }
    }
}

pub(super) fn hover_action_for_result(
    result: &SettingsListHitTestResult,
) -> Option<SettingsListAction> {
    match result {
        SettingsListHitTestResult::Field { field_id } => Some(SettingsListAction::HoverField {
            field_id: field_id.clone(),
            hovered: true,
        }),
        SettingsListHitTestResult::ToggleSection { section_id } => {
            Some(SettingsListAction::HoverSection {
                section_id: section_id.clone(),
                hovered: true,
            })
        }
        SettingsListHitTestResult::None => None,
    }
}

pub(super) fn action_for_result(result: &SettingsListHitTestResult) -> Option<SettingsListAction> {
    match result {
        SettingsListHitTestResult::ToggleSection { section_id } => {
            Some(SettingsListAction::ToggleSection {
                section_id: section_id.clone(),
            })
        }
        SettingsListHitTestResult::Field { .. } | SettingsListHitTestResult::None => None,
    }
}
