#[path = "adapter_types.rs"]
mod adapter_types;
#[path = "input_policy.rs"]
mod input_policy;
#[path = "paint_types.rs"]
mod paint_types;

pub use adapter_types::EguiTextSurfaceAdapter;
pub use input_policy::{EguiTextSurfaceInputPolicy, EguiTextSurfaceKey};
pub use paint_types::{
    EguiTextSurfaceDrawLayer, TextSurfaceAnnotationPaint, TextSurfaceGutterPaint,
    TextSurfacePaintOperation, TextSurfacePaintOperationKind, TextSurfacePaintPlan,
    TextSurfacePaintStyle, TextSurfacePaintTexture, TextSurfaceRasterStyle,
};
