//! egui projection for the generic source-address strip.

mod adapter;
mod interaction;
mod lifecycle;
mod paint;
mod raster;
mod render;
mod types;

pub use adapter::EguiSourceAddressStripAdapter;
pub use types::{
    EguiSourceAddressStripError, EguiSourceAddressStripOutput, SourceAddressFrameEventClass,
    SourceAddressLabelRasterEvidence, SourceAddressPaintOperation, SourceAddressPaintOperationKind,
    SourceAddressPaintPlan, SourceAddressPaintTexture, SourceAddressRasterEvidenceReceipt,
    SourceAddressRenderStyle, SourceAddressSubmissionForwarder,
};
