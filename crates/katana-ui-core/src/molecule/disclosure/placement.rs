use super::{HoverCard, Popover, Tooltip};
use crate::interaction::placement::{
    PlacementConsumer, PlacementEngine, PlacementRequest, PlacementResult,
};

impl Tooltip {
    #[must_use]
    pub fn resolve_panel_placement(&self, request: &PlacementRequest) -> PlacementResult {
        PlacementEngine::resolve_for(PlacementConsumer::Tooltip, request)
    }
}

impl Popover {
    #[must_use]
    pub fn resolve_panel_placement(&self, request: &PlacementRequest) -> PlacementResult {
        PlacementEngine::resolve_for(PlacementConsumer::Popover, request)
    }
}

impl HoverCard {
    #[must_use]
    pub fn resolve_panel_placement(&self, request: &PlacementRequest) -> PlacementResult {
        PlacementEngine::resolve_for(PlacementConsumer::HoverCard, request)
    }
}
