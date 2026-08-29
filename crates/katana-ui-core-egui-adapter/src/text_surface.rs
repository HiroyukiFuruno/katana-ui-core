mod accessibility;
mod adapter;
mod artifact_model;
mod controlled_focus;
mod controlled_scroll;
mod gutter_icon;
mod gutter_interaction;
mod input;
mod measurement;
mod model;
mod paint;
mod raster;

pub use artifact_model::{
    EguiTextSurfaceError, EguiTextSurfaceFrameRecord, EguiTextSurfaceOutput,
    TextSurfaceArtifactFrame, TextSurfaceContextTargetAnchor,
};
pub use model::SharedTextMetrics;
pub use model::{
    EguiTextSurfaceAdapter, EguiTextSurfaceDrawLayer, EguiTextSurfaceInputPolicy,
    EguiTextSurfaceKey, TextSurfaceAnnotationPaint, TextSurfaceGutterPaint,
    TextSurfacePaintOperation, TextSurfacePaintOperationKind, TextSurfacePaintPlan,
    TextSurfacePaintStyle, TextSurfacePaintTexture, TextSurfaceRasterStyle,
};
