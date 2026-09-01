//! Public deterministic RGBA composition for KUC adapter paint plans.

#[path = "artifact_compositor_blend.rs"]
mod artifact_compositor_blend;
#[path = "artifact_compositor_geometry.rs"]
mod artifact_compositor_geometry;
#[path = "artifact_compositor_hash.rs"]
mod artifact_compositor_hash;
#[path = "artifact_compositor_paint.rs"]
mod artifact_compositor_paint;
#[path = "artifact_compositor_types.rs"]
mod artifact_compositor_types;

pub use artifact_compositor_types::{
    ArtifactCanvasBounds, ArtifactCompositeError, ArtifactCompositeFrame, ArtifactCompositeRequest,
    ArtifactPaintPlanRef,
};

/// KUC-owned compositor for adapter artifact paint plans.
#[derive(Debug, Default, Clone, Copy)]
pub struct ArtifactCompositor;

impl ArtifactCompositor {
    /// Composes the caller-supplied actual root canvas in the supplied paint order.
    pub fn compose(
        request: ArtifactCompositeRequest<'_>,
    ) -> Result<ArtifactCompositeFrame, ArtifactCompositeError> {
        artifact_compositor_paint::compose(request)
    }
}

#[cfg(test)]
#[path = "artifact_compositor_tests.rs"]
mod artifact_compositor_tests;
