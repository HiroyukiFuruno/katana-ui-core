use super::{item::SanitizedContextMenuItem, projection::SanitizedContextMenuProjection};

/// Consuming builder for a generic context-menu projection.
#[derive(Debug, Default)]
pub struct SanitizedContextMenuProjectionBuilder {
    items: Vec<SanitizedContextMenuItem>,
}

impl SanitizedContextMenuProjectionBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn item(mut self, item: SanitizedContextMenuItem) -> Self {
        self.items.push(item);
        self
    }

    #[must_use]
    pub fn build(self) -> SanitizedContextMenuProjection {
        SanitizedContextMenuProjection::new(self.items)
    }
}
