use super::{ComboBox, MenuButton, SelectBox};
use crate::interaction::placement::{
    PlacementConsumer, PlacementEngine, PlacementRequest, PlacementResult,
};

impl MenuButton {
    #[must_use]
    pub fn resolve_panel_placement(&self, request: &PlacementRequest) -> PlacementResult {
        PlacementEngine::resolve_for(PlacementConsumer::MenuButton, request)
    }
}

impl SelectBox {
    #[must_use]
    pub fn resolve_panel_placement(&self, request: &PlacementRequest) -> PlacementResult {
        PlacementEngine::resolve_for(PlacementConsumer::SelectBox, request)
    }
}

impl ComboBox {
    #[must_use]
    pub fn resolve_panel_placement(&self, request: &PlacementRequest) -> PlacementResult {
        PlacementEngine::resolve_for(PlacementConsumer::ComboBox, request)
    }
}
