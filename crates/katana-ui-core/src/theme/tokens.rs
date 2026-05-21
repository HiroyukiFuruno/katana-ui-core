use crate::interaction::{MotionDistanceToken, MotionDurationToken, MotionEasingToken};
use serde::{Deserialize, Serialize};

pub const RGBA_CHANNEL_COUNT: usize = 4;
pub type Rgba = [u8; RGBA_CHANNEL_COUNT];

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThemeId(String);

impl ThemeId {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorToken {
    pub name: String,
    pub rgba: Rgba,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FontFamily {
    Proportional,
    Monospace,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FontToken {
    pub name: String,
    pub family: FontFamily,
    pub size: f32,
    pub weight: u16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpacingToken {
    pub name: String,
    pub px: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadiusToken {
    pub name: String,
    pub px: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowToken {
    pub name: String,
    pub blur: f32,
    pub spread: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorderToken {
    pub name: String,
    pub width: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZIndexToken {
    pub name: String,
    pub value: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotionToken {
    pub name: String,
    pub duration_ms: u16,
    pub easing: String,
    pub distance_px: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotionTokenSet {
    pub instant_ms: u16,
    pub fast_ms: u16,
    pub default_ms: u16,
    pub slow_ms: u16,
    pub compact_px: u16,
    pub default_px: u16,
    pub spacious_px: u16,
}

impl MotionTokenSet {
    #[must_use]
    pub const fn duration(&self, token: MotionDurationToken) -> u16 {
        match token {
            MotionDurationToken::Instant => self.instant_ms,
            MotionDurationToken::Fast => self.fast_ms,
            MotionDurationToken::Default => self.default_ms,
            MotionDurationToken::Slow => self.slow_ms,
        }
    }

    #[must_use]
    pub const fn distance(&self, token: MotionDistanceToken) -> u16 {
        match token {
            MotionDistanceToken::Compact => self.compact_px,
            MotionDistanceToken::Default => self.default_px,
            MotionDistanceToken::Spacious => self.spacious_px,
        }
    }

    #[must_use]
    pub const fn easing(&self, token: MotionEasingToken) -> &'static str {
        match token {
            MotionEasingToken::Linear => "linear",
            MotionEasingToken::Standard => "standard",
            MotionEasingToken::Emphasized => "emphasized",
            MotionEasingToken::Decelerate => "decelerate",
            MotionEasingToken::Accelerate => "accelerate",
        }
    }
}

impl Default for MotionTokenSet {
    fn default() -> Self {
        const INSTANT_DURATION_MS: u16 = 0;
        const FAST_DURATION_MS: u16 = 120;
        const DEFAULT_DURATION_MS: u16 = 200;
        const SLOW_DURATION_MS: u16 = 320;
        const COMPACT_DISTANCE_PX: u16 = 4;
        const DEFAULT_DISTANCE_PX: u16 = 8;
        const SPACIOUS_DISTANCE_PX: u16 = 16;

        Self {
            instant_ms: INSTANT_DURATION_MS,
            fast_ms: FAST_DURATION_MS,
            default_ms: DEFAULT_DURATION_MS,
            slow_ms: SLOW_DURATION_MS,
            compact_px: COMPACT_DISTANCE_PX,
            default_px: DEFAULT_DISTANCE_PX,
            spacious_px: SPACIOUS_DISTANCE_PX,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeDiff {
    changed_sections: Vec<String>,
}

impl ThemeDiff {
    pub(crate) fn new(changed_sections: Vec<String>) -> Self {
        Self { changed_sections }
    }

    #[must_use]
    pub fn changed_sections(&self) -> &[String] {
        &self.changed_sections
    }
}
