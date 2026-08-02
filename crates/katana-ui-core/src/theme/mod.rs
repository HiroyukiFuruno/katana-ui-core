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
    use super::{FontFamily, MotionToken, ThemeSnapshot};
    use crate::interaction::{MotionDurationToken, MotionEasingToken};

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
        assert_eq!(Some([248, 250, 252, 255]), dark.color("accent-foreground"));
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

    #[test]
    fn diff_reports_font_spacing_and_motion_changes() {
        let base = ThemeSnapshot::light();
        let mut changed = base.clone();
        changed.fonts[0].size += 1.0;
        changed.spacing[0].px += 1.0;
        changed.motion[0].duration_ms += 1;

        assert_eq!(
            &[
                "fonts".to_string(),
                "spacing".to_string(),
                "motion".to_string(),
            ],
            base.diff(&changed).changed_sections()
        );
    }

    #[test]
    fn motion_tokens_map_duration_distance_alias_and_unknown_names() {
        let mut theme = ThemeSnapshot::light();
        theme.motion = vec![
            motion("instant", 1, 0),
            motion("fast", 2, 0),
            motion("standard", 3, 0),
            motion("slow", 4, 0),
            motion("compact", 0, 5),
            motion("spacious", 0, 6),
            motion("unknown", 99, 99),
        ];

        let tokens = theme.motion_tokens();
        assert_eq!(1, tokens.instant_ms);
        assert_eq!(2, tokens.fast_ms);
        assert_eq!(3, tokens.default_ms);
        assert_eq!(4, tokens.slow_ms);
        assert_eq!(5, tokens.compact_px);
        assert_eq!(6, tokens.spacious_px);
        assert_eq!(1, tokens.duration(MotionDurationToken::Instant));
        assert_eq!("accelerate", tokens.easing(MotionEasingToken::Accelerate));
    }

    fn motion(name: &str, duration_ms: u16, distance_px: u16) -> MotionToken {
        MotionToken {
            name: name.to_string(),
            duration_ms,
            easing: "linear".to_string(),
            distance_px,
        }
    }
}
