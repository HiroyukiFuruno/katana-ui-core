use crate::interaction::{MotionPrimitiveKind, MotionSpec, ReducedMotionPolicy};
use serde::{Deserialize, Serialize};

const STARTUP_MOTION_DURATION_MS: u16 = 180;
const STARTUP_MOTION_DISTANCE_PX: u16 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartupStatePanelOptions {
    pub live_region_label: String,
    pub reduced_motion: bool,
    pub motion: MotionSpec,
}

impl StartupStatePanelOptions {
    #[must_use]
    pub fn live_region_label(mut self, value: impl Into<String>) -> Self {
        self.live_region_label = value.into();
        self
    }

    #[must_use]
    pub fn reduced_motion(mut self, value: bool) -> Self {
        self.reduced_motion = value;
        self
    }

    #[must_use]
    pub fn motion(mut self, value: MotionSpec) -> Self {
        self.motion = value;
        self
    }
}

impl Default for StartupStatePanelOptions {
    fn default() -> Self {
        Self {
            live_region_label: String::new(),
            reduced_motion: false,
            motion: MotionSpec::new(
                MotionPrimitiveKind::Shimmer,
                STARTUP_MOTION_DURATION_MS,
                STARTUP_MOTION_DISTANCE_PX,
                ReducedMotionPolicy::Respect,
            ),
        }
    }
}
