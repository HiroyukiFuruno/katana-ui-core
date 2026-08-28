mod model_bounds;
mod model_error;
mod model_raster;
mod model_request;

pub(crate) const RGBA_CHANNEL_COUNT: usize = 4;
pub(crate) const RGBA_ALPHA_INDEX: usize = 3;
pub(crate) const TRANSPARENT_RGBA: [u8; RGBA_CHANNEL_COUNT] = [0, 0, 0, 0];

pub use model_bounds::{PlatformTextGraphemeBounds, PlatformTextGraphemeRange};
pub use model_error::{PlatformTextRasterError, PlatformTextRasterReport, PlatformTextRasterStats};
pub use model_raster::{PlatformTextHit, PlatformTextRaster, PlatformTextRasterCrop};
pub use model_request::PlatformTextRasterRequest;
