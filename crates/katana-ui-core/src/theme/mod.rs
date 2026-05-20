mod preset;
mod tokens;

pub use tokens::{
    BorderToken, ColorToken, FontFamily, FontToken, MotionToken, MotionTokenSet, RadiusToken, Rgba,
    ShadowToken, SpacingToken, ThemeDiff, ThemeId, ZIndexToken,
};

use preset::ThemePreset;
use serde::{Deserialize, Serialize};

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
    pub motion: Vec<MotionToken>,
}

impl ThemeSnapshot {
    #[must_use]
    pub fn light() -> Self {
        ThemePreset::light().into_snapshot()
    }

    #[must_use]
    pub fn dark() -> Self {
        ThemePreset::dark().into_snapshot()
    }

    #[must_use]
    pub fn color(&self, name: &str) -> Option<Rgba> {
        self.colors
            .iter()
            .find(|it| it.name == name)
            .map(|it| it.rgba)
    }

    #[must_use]
    pub fn font(&self, name: &str) -> Option<&FontToken> {
        self.fonts.iter().find(|it| it.name == name)
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
        if self.motion != other.motion {
            changed_sections.push("motion".to_string());
        }
        ThemeDiff::new(changed_sections)
    }

    #[must_use]
    pub fn motion_tokens(&self) -> MotionTokenSet {
        let mut tokens = MotionTokenSet::default();
        for token in &self.motion {
            match token.name.as_str() {
                "instant" => tokens.instant_ms = token.duration_ms,
                "fast" => tokens.fast_ms = token.duration_ms,
                "default" | "standard" => tokens.default_ms = token.duration_ms,
                "slow" => tokens.slow_ms = token.duration_ms,
                "compact" => tokens.compact_px = token.distance_px,
                "spacious" => tokens.spacious_px = token.distance_px,
                _ => {}
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::{FontFamily, ThemeSnapshot};

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

    #[test]
    fn theme_uses_katana_colors_and_font_roles() {
        let dark = ThemeSnapshot::dark();

        assert_eq!(Some([86, 156, 214, 255]), dark.color("accent"));
        assert_eq!(Some([37, 37, 38, 255]), dark.color("panel"));
        assert_eq!(
            Some(FontFamily::Proportional),
            dark.font("body").map(|it| it.family)
        );
        assert_eq!(
            Some(FontFamily::Monospace),
            dark.font("code").map(|it| it.family)
        );
        assert!(dark.motion.iter().any(|it| it.name == "fast"));
    }
}
