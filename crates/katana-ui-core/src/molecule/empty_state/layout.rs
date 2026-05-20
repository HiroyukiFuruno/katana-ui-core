use super::{EmptyState, EmptyStateAlignment, EmptyStateSize};
use serde::{Deserialize, Serialize};

const COMPACT_WIDTH: u16 = 280;
const DEFAULT_WIDTH: u16 = 360;
const LARGE_WIDTH: u16 = 440;
const COMPACT_CONTENT_WIDTH: u16 = 220;
const DEFAULT_CONTENT_WIDTH: u16 = 280;
const LARGE_CONTENT_WIDTH: u16 = 340;
const COMPACT_HEADING_HEIGHT: u16 = 20;
const DEFAULT_HEADING_HEIGHT: u16 = 24;
const LARGE_HEADING_HEIGHT: u16 = 32;
const BODY_HEIGHT: u16 = 20;
const ACTION_HEIGHT: u16 = 36;
const BLOCK_GAP: u16 = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmptyStateLayoutRect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmptyStateLayoutSnapshot {
    pub size: EmptyStateSize,
    pub alignment: EmptyStateAlignment,
    pub has_body: bool,
    pub action_count: usize,
    pub heading_rect: EmptyStateLayoutRect,
    pub body_rect: Option<EmptyStateLayoutRect>,
    pub action_rect: Option<EmptyStateLayoutRect>,
}

pub(super) fn snapshot(value: &EmptyState) -> EmptyStateLayoutSnapshot {
    let metrics = LayoutMetrics::for_size(value.size);
    let x = match value.alignment {
        EmptyStateAlignment::Center => (metrics.container_width - metrics.content_width) / 2,
        EmptyStateAlignment::Leading => 0,
    };
    let heading_rect = EmptyStateLayoutRect {
        x,
        y: 0,
        width: metrics.content_width,
        height: metrics.heading_height,
    };
    let body_rect = value.body.as_ref().map(|_| EmptyStateLayoutRect {
        x,
        y: heading_rect.height + BLOCK_GAP,
        width: metrics.content_width,
        height: BODY_HEIGHT,
    });
    let action_y = body_rect.map_or(heading_rect.height + BLOCK_GAP, |rect| {
        rect.y + rect.height + BLOCK_GAP
    });
    let action_count =
        usize::from(value.primary_action.is_some()) + usize::from(value.secondary_action.is_some());
    EmptyStateLayoutSnapshot {
        size: value.size,
        alignment: value.alignment,
        has_body: body_rect.is_some(),
        action_count,
        heading_rect,
        body_rect,
        action_rect: (action_count > 0).then_some(EmptyStateLayoutRect {
            x,
            y: action_y,
            width: metrics.content_width,
            height: ACTION_HEIGHT,
        }),
    }
}

#[derive(Debug, Clone, Copy)]
struct LayoutMetrics {
    container_width: u16,
    content_width: u16,
    heading_height: u16,
}

impl LayoutMetrics {
    const fn for_size(size: EmptyStateSize) -> Self {
        match size {
            EmptyStateSize::Compact => Self {
                container_width: COMPACT_WIDTH,
                content_width: COMPACT_CONTENT_WIDTH,
                heading_height: COMPACT_HEADING_HEIGHT,
            },
            EmptyStateSize::Default => Self {
                container_width: DEFAULT_WIDTH,
                content_width: DEFAULT_CONTENT_WIDTH,
                heading_height: DEFAULT_HEADING_HEIGHT,
            },
            EmptyStateSize::Large => Self {
                container_width: LARGE_WIDTH,
                content_width: LARGE_CONTENT_WIDTH,
                heading_height: LARGE_HEADING_HEIGHT,
            },
        }
    }
}
