//! KatanA ecosystem framework-neutral UI Core.

pub mod accessibility;
pub mod adapter_contract;
pub mod atom;
pub mod component;
#[cfg(feature = "egui")]
pub mod egui;
pub mod event;
pub mod facade;
pub mod interaction;
pub mod layout;
pub mod molecule;
pub mod panel;
pub mod render_model;
pub mod runtime;
pub mod state;
pub mod style;
pub mod surface;
#[cfg(feature = "svg-raster")]
pub mod svg_raster;
#[cfg(feature = "text-raster")]
pub mod text_raster;
pub mod text_selection;
pub mod text_surface;
pub mod theme;
pub mod widget;
pub mod window;

/// Registry marker for KUC-owned neutral UI capabilities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CoreRegistry;

impl CoreRegistry {
    /// Creates an empty registry marker.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::CoreRegistry;

    #[test]
    fn new_returns_empty_registry_marker() {
        assert_eq!(CoreRegistry::new(), CoreRegistry);
    }
}
