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
