//! Shared actual-egui adapter for the generic KUC context-menu model.

mod accessibility;
mod adapter;
mod artifact;
mod interaction;
mod paint;
mod presentation;
mod state;
mod surface;
mod types;

#[cfg(test)]
mod tests;

pub use types::{
    ContextMenuAdapterError, ContextMenuArtifactFrame, ContextMenuPaintOperation,
    ContextMenuPaintOperationKind, ContextMenuPaintPlan, ContextMenuPaintStyle,
    ContextMenuPaintTexture, ContextMenuPresentation, ContextMenuPresentationItem,
    ContextMenuRasterStyle, EguiContextMenuAdapter, EguiContextMenuFrameRecord,
    EguiContextMenuItemFrame, EguiContextMenuOutput,
};
