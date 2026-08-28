use crate::render_model::UiRect;
use serde::{Deserialize, Serialize};

/// Opaque idempotency key supplied by a controlled consumer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TextSurfaceScrollRequestToken(pub(super) String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSurfaceScrollAlignment {
    Nearest,
    Start,
    Center,
    End,
}

/// Logical-pixel transport value for a controlled scroll request.
///
/// The wire form is this value's normalized IEEE-754 bit pattern as an unsigned integer.
#[derive(Debug, Clone, Copy)]
pub struct TextSurfaceLogicalPixels(pub(super) f32);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSurfaceScrollTarget {
    LogicalRow {
        logical_row: usize,
    },
    ByteOffset {
        byte_offset: usize,
    },
    ByteRange {
        byte_start: usize,
        byte_end: usize,
    },
    RelativePixels {
        delta_x: TextSurfaceLogicalPixels,
        delta_y: TextSurfaceLogicalPixels,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceScrollRequest {
    pub token: TextSurfaceScrollRequestToken,
    pub target: TextSurfaceScrollTarget,
    pub alignment: TextSurfaceScrollAlignment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSurfaceScrollRequestRejection {
    NonFiniteRelativePixels,
    InvalidUtf8Boundary,
    InvalidByteRange,
    LogicalRowNotFound,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceScrollRequestAcknowledgement {
    pub token: TextSurfaceScrollRequestToken,
    pub target_bounds: Option<UiRect>,
    pub scroll_x: i32,
    pub scroll_y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSurfaceScrollRequestResult {
    Acknowledged(TextSurfaceScrollRequestAcknowledgement),
    Rejected {
        token: TextSurfaceScrollRequestToken,
        reason: TextSurfaceScrollRequestRejection,
    },
}
