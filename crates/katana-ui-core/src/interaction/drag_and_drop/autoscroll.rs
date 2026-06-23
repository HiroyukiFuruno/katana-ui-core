use super::{DndPoint, DndRect};
use serde::{Deserialize, Serialize};

const DEFAULT_EDGE_ZONE_PX: f32 = 24.0;
const DEFAULT_MAX_SPEED_PX_PER_TICK: f32 = 16.0;
const ACCELERATION_TICKS: u32 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AutoScrollPolicy {
    pub enabled: bool,
    pub edge_zone_px: f32,
    pub max_speed_px_per_tick: f32,
}

impl Default for AutoScrollPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            edge_zone_px: DEFAULT_EDGE_ZONE_PX,
            max_speed_px_per_tick: DEFAULT_MAX_SPEED_PX_PER_TICK,
        }
    }
}

impl AutoScrollPolicy {
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            enabled: false,
            edge_zone_px: 0.0,
            max_speed_px_per_tick: 0.0,
        }
    }

    #[must_use]
    pub const fn edge_zone_px(mut self, value: f32) -> Self {
        self.edge_zone_px = value;
        self
    }

    #[must_use]
    pub const fn max_speed_px_per_tick(mut self, value: f32) -> Self {
        self.max_speed_px_per_tick = value;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutoScrollAxis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutoScrollDirection {
    Negative,
    Positive,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AutoScrollRequest {
    pub axis: AutoScrollAxis,
    pub direction: AutoScrollDirection,
    pub speed_px_per_tick: f32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutoScrollEngine;

impl AutoScrollEngine {
    #[must_use]
    pub fn request(
        policy: &AutoScrollPolicy,
        viewport: DndRect,
        position: DndPoint,
        hold_ticks: u32,
    ) -> Option<AutoScrollRequest> {
        if !policy.enabled || policy.edge_zone_px <= 0.0 || !viewport.contains(position) {
            return None;
        }
        nearest_edge(policy, viewport, position).map(|edge| AutoScrollRequest {
            axis: edge.axis,
            direction: edge.direction,
            speed_px_per_tick: speed(policy, edge.depth_ratio, hold_ticks),
        })
    }
}

struct EdgeCandidate {
    axis: AutoScrollAxis,
    direction: AutoScrollDirection,
    distance_px: f32,
    depth_ratio: f32,
}

fn nearest_edge(
    policy: &AutoScrollPolicy,
    viewport: DndRect,
    position: DndPoint,
) -> Option<EdgeCandidate> {
    [
        edge(
            policy,
            AutoScrollAxis::Vertical,
            AutoScrollDirection::Negative,
            position.y - viewport.y,
        ),
        edge(
            policy,
            AutoScrollAxis::Vertical,
            AutoScrollDirection::Positive,
            viewport.y + viewport.height - position.y,
        ),
        edge(
            policy,
            AutoScrollAxis::Horizontal,
            AutoScrollDirection::Negative,
            position.x - viewport.x,
        ),
        edge(
            policy,
            AutoScrollAxis::Horizontal,
            AutoScrollDirection::Positive,
            viewport.x + viewport.width - position.x,
        ),
    ]
    .into_iter()
    .flatten()
    .min_by(|left, right| left.distance_px.total_cmp(&right.distance_px))
}

fn edge(
    policy: &AutoScrollPolicy,
    axis: AutoScrollAxis,
    direction: AutoScrollDirection,
    distance_px: f32,
) -> Option<EdgeCandidate> {
    if distance_px > policy.edge_zone_px {
        return None;
    }
    Some(EdgeCandidate {
        axis,
        direction,
        distance_px,
        depth_ratio: (policy.edge_zone_px - distance_px) / policy.edge_zone_px,
    })
}

fn speed(policy: &AutoScrollPolicy, depth_ratio: f32, hold_ticks: u32) -> f32 {
    let time_ratio = (hold_ticks.min(ACCELERATION_TICKS) as f32 / ACCELERATION_TICKS as f32)
        .max(1.0 / ACCELERATION_TICKS as f32);
    policy.max_speed_px_per_tick * depth_ratio * depth_ratio * time_ratio
}
