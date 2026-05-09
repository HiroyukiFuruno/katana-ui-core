//! KatanA ecosystem shared UI widgets.

/// Registry marker for shared widget components.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WidgetRegistry;

impl WidgetRegistry {
    /// Creates an empty registry marker.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::WidgetRegistry;

    #[test]
    fn new_returns_empty_registry_marker() {
        assert_eq!(WidgetRegistry::new(), WidgetRegistry);
    }
}
