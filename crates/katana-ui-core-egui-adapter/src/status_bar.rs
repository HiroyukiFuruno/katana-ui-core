//! Generic KUC status-bar projection.

mod accessibility;
mod adapter;
mod paint;
mod paint_plan;
mod popover;
mod render;
mod types;

pub use adapter::EguiStatusBarAdapter;
pub use types::{
    EguiStatusBarError, EguiStatusBarOutput, StatusBarLabelRasterEvidence, StatusBarPaintOperation,
    StatusBarPaintOperationKind, StatusBarPaintPlan, StatusBarPaintTexture, StatusBarRenderStyle,
};

#[cfg(test)]
#[path = "status_bar_tests.rs"]
mod tests;
