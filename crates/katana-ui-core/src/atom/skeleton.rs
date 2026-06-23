mod render;
mod types;

use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
pub use types::{Skeleton, SkeletonAnimation, SkeletonAspectRatio, SkeletonShape, SkeletonSize};

impl ComponentAction for Skeleton {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = render::state(self);
        if action.target() != &self.state_id {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        match action {
            UiAction::SetReducedMotion { reduced_motion, .. } => {
                self.reduced_motion = *reduced_motion;
            }
            _ => return UiActionResult::ignored(self.state_id.clone(), before),
        }
        UiActionResult::handled(self.state_id.clone(), action, before, render::state(self))
    }
}
