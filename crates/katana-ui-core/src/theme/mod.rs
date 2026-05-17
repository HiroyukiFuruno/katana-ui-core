use serde::{Deserialize, Serialize};

const RGBA_CHANNEL_COUNT: usize = 4;
type Rgba = [u8; RGBA_CHANNEL_COUNT];
const LIGHT_SURFACE: Rgba = [250, 250, 250, 255];
const LIGHT_TEXT: Rgba = [24, 24, 27, 255];
const DARK_SURFACE: Rgba = [24, 24, 27, 255];
const DARK_TEXT: Rgba = [250, 250, 250, 255];
const BODY_FONT_SIZE: f32 = 14.0;
const BODY_FONT_WEIGHT: u16 = 400;
const SPACING_SM: f32 = 8.0;
const RADIUS_SM: f32 = 4.0;
const SHADOW_NONE: f32 = 0.0;
const BORDER_THIN: f32 = 1.0;
const Z_INDEX_OVERLAY: i32 = 100;

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FontToken {
    pub name: String,
    pub family: String,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeSnapshot {
    pub id: ThemeId,
    pub colors: Vec<ColorToken>,
    pub fonts: Vec<FontToken>,
    pub spacing: Vec<SpacingToken>,
    pub radii: Vec<RadiusToken>,
    pub shadows: Vec<ShadowToken>,
    pub borders: Vec<BorderToken>,
    pub z_indexes: Vec<ZIndexToken>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeDiff {
    changed_sections: Vec<String>,
}

impl ThemeSnapshot {
    #[must_use]
    pub fn light() -> Self {
        Self::fixture("light", LIGHT_SURFACE, LIGHT_TEXT)
    }

    #[must_use]
    pub fn dark() -> Self {
        Self::fixture("dark", DARK_SURFACE, DARK_TEXT)
    }

    #[must_use]
    pub fn diff(&self, other: &Self) -> ThemeDiff {
        let mut changed_sections = Vec::new();
        if self.colors != other.colors {
            changed_sections.push("colors".to_string());
        }
        if self.fonts != other.fonts {
            changed_sections.push("fonts".to_string());
        }
        if self.spacing != other.spacing {
            changed_sections.push("spacing".to_string());
        }
        ThemeDiff { changed_sections }
    }

    fn fixture(id: &str, surface: Rgba, text: Rgba) -> Self {
        Self {
            id: ThemeId::new(id),
            colors: vec![
                ColorToken {
                    name: "surface".to_string(),
                    rgba: surface,
                },
                ColorToken {
                    name: "text".to_string(),
                    rgba: text,
                },
            ],
            fonts: vec![FontToken {
                name: "body".to_string(),
                family: "system".to_string(),
                size: BODY_FONT_SIZE,
                weight: BODY_FONT_WEIGHT,
            }],
            spacing: vec![SpacingToken {
                name: "sm".to_string(),
                px: SPACING_SM,
            }],
            radii: vec![RadiusToken {
                name: "sm".to_string(),
                px: RADIUS_SM,
            }],
            shadows: vec![ShadowToken {
                name: "none".to_string(),
                blur: SHADOW_NONE,
                spread: SHADOW_NONE,
            }],
            borders: vec![BorderToken {
                name: "thin".to_string(),
                width: BORDER_THIN,
            }],
            z_indexes: vec![ZIndexToken {
                name: "overlay".to_string(),
                value: Z_INDEX_OVERLAY,
            }],
        }
    }
}

impl ThemeDiff {
    #[must_use]
    pub fn changed_sections(&self) -> &[String] {
        &self.changed_sections
    }
}

#[cfg(test)]
mod tests {
    use super::ThemeSnapshot;

    #[test]
    fn light_and_dark_have_stable_ids() {
        assert_eq!("light", ThemeSnapshot::light().id.as_str());
        assert_eq!("dark", ThemeSnapshot::dark().id.as_str());
    }

    #[test]
    fn diff_reports_changed_sections() {
        let diff = ThemeSnapshot::light().diff(&ThemeSnapshot::dark());
        assert_eq!(&["colors".to_string()], diff.changed_sections());
    }
}
