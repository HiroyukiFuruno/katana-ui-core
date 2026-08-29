use super::scroll_request_types::{
    TextSurfaceLogicalPixels, TextSurfaceScrollAlignment, TextSurfaceScrollRequest,
    TextSurfaceScrollRequestToken, TextSurfaceScrollTarget,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl TextSurfaceScrollRequestToken {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TextSurfaceLogicalPixels {
    #[must_use]
    pub fn new(value: f32) -> Self {
        Self(if value == 0.0 { 0.0 } else { value })
    }

    #[must_use]
    pub const fn value(self) -> f32 {
        self.0
    }

    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    #[must_use]
    pub const fn bit_pattern(self) -> u32 {
        self.0.to_bits()
    }
}

impl From<f32> for TextSurfaceLogicalPixels {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl From<i32> for TextSurfaceLogicalPixels {
    fn from(value: i32) -> Self {
        Self::new(value as f32)
    }
}

impl PartialEq for TextSurfaceLogicalPixels {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for TextSurfaceLogicalPixels {}

impl Serialize for TextSurfaceLogicalPixels {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bit_pattern())
    }
}

impl<'de> Deserialize<'de> for TextSurfaceLogicalPixels {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::new(f32::from_bits(u32::deserialize(deserializer)?)))
    }
}

impl TextSurfaceScrollTarget {
    /// Creates a relative logical-pixel target without consumer-side integer conversion.
    #[must_use]
    pub fn relative_pixels(
        delta_x: impl Into<TextSurfaceLogicalPixels>,
        delta_y: impl Into<TextSurfaceLogicalPixels>,
    ) -> Self {
        Self::RelativePixels {
            delta_x: delta_x.into(),
            delta_y: delta_y.into(),
        }
    }
}

impl TextSurfaceScrollRequest {
    #[must_use]
    pub fn new(
        token: TextSurfaceScrollRequestToken,
        target: TextSurfaceScrollTarget,
        alignment: TextSurfaceScrollAlignment,
    ) -> Self {
        Self {
            token,
            target,
            alignment,
        }
    }
}
