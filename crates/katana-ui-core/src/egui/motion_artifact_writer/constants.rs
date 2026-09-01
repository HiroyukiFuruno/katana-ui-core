pub(super) const DEFAULT_ENCODER: &str = "libx264rgb";
pub(super) const DEFAULT_MUXER: &str = "mp4";
pub(super) const DEFAULT_PIXEL_FORMAT: &str = "rgb24";

pub(super) const MOTION_SCHEMA: &str = "kuc.retained-root-motion.v1";
pub(super) const ROOT_IMAGE_PATTERN: &str = "frame-%03d.png";
pub(super) const DEFAULT_GIF_FILENAME: &str = "motion.gif";
pub(super) const DEFAULT_MP4_FILENAME: &str = "motion.mp4";
pub(super) const DEFAULT_MANIFEST_FILENAME: &str = "motion-manifest.json";

pub(super) const STAGE_NAME_PREFIX: &str = "frame-";
pub(super) const STAGE_NAME_WIDTH: usize = 3;
pub(super) const STAGE_DIMENSIONS_PREFIX: &str = "#dimensions 0:";
pub(super) const GIF_DELAY_DENOMINATOR_MS: u32 = 1;
pub(super) const DEFAULT_FPS_NUMERATOR: u32 = 1_000;
pub(super) const DEFAULT_FPS_DENOMINATOR: u32 = 180;
