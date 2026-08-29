//! Actual egui boundary for KUC-owned component rendering.
//!
//! Host applications bind domain actions outside this crate. Shared geometry,
//! texture upload, input conversion, and accessibility projection stay here.

mod closeable_tab_strip_adapter;
#[cfg(feature = "storybook-artifacts")]
mod system;
mod texture_cache;

pub mod artifact_compositor;
pub mod command_chrome;
pub mod context_menu;
#[cfg(feature = "storybook-artifacts")]
mod full_root_artifact_writer;
#[cfg(feature = "storybook-artifacts")]
mod motion_artifact_writer;
#[cfg(feature = "storybook-artifacts")]
mod opaque_motion_receipt;
pub mod text_command_surface;
pub mod text_surface;

#[cfg(feature = "storybook-artifacts")]
pub use full_root_artifact_writer::{
    FullRootArtifact, FullRootArtifactError, FullRootArtifactWriter,
};
#[cfg(feature = "storybook-artifacts")]
pub use motion_artifact_writer::{
    MotionArtifact, MotionArtifactError, MotionArtifactManifest, MotionArtifactSettings,
    MotionArtifactWriter,
};
#[cfg(feature = "storybook-artifacts")]
pub use opaque_motion_receipt::{
    OpaqueMotionReceiptSequence, OpaqueMotionReceiptSequenceError, OpaqueRootArtifactReceipt,
    OpaqueRootArtifactReceiptError, OpaqueRootArtifactReceiptWriter,
};

#[cfg(test)]
fn run_ui_discard(
    context: &egui::Context,
    input: egui::RawInput,
    run_ui: impl FnMut(&mut egui::Ui),
) {
    let mut output = context.run_ui(input, run_ui);
    output.textures_delta.clear();
}
