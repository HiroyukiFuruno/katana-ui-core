use super::action::{TextSurfaceActionOutcome, TextSurfaceEvent};
use super::surface_model::TextSurface;

impl TextSurface {
    pub(super) fn gutter_outcome(
        &self,
        handled: bool,
        event: TextSurfaceEvent,
    ) -> TextSurfaceActionOutcome {
        TextSurfaceActionOutcome {
            handled,
            events: handled.then_some(event).into_iter().collect(),
            state: self.state.clone(),
        }
    }
}
