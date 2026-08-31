//! Generic KUC status-bar projection.

mod accessibility;
mod adapter;
mod alignment;
mod paint;
mod paint_plan;
mod popover;
mod progress;
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

#[cfg(test)]
#[path = "status_bar_layout_tests.rs"]
mod layout_tests;

#[cfg(test)]
#[path = "status_bar_alignment_layout_tests.rs"]
mod alignment_layout_tests;

#[cfg(test)]
#[path = "status_bar_failure_tests.rs"]
mod failure_tests;
