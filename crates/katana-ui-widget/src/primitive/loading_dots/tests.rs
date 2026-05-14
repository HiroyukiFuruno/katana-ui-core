use super::LoadingDots;
use super::types::{DEFAULT_COUNT, DEFAULT_GAP, DEFAULT_SIZE, DEFAULT_SPEED_MS, MIN_SIZE};

use crate::theme::Theme;

#[test]
fn resolve_uses_theme_accent_without_override() {
    let theme = Theme::default_light();
    let resolved = LoadingDots::new().resolve(&theme);
    assert_eq!(resolved.dot_count, DEFAULT_COUNT);
    assert_eq!(resolved.dot_size, DEFAULT_SIZE);
    assert_eq!(resolved.dot_gap, DEFAULT_GAP);
    assert_eq!(resolved.animation_speed_ms, DEFAULT_SPEED_MS);
    assert_eq!(resolved.color, theme.color.accent);
}

#[test]
fn clamp_size_and_gap_values_on_resolve() {
    let resolved = LoadingDots::new()
        .dot_size(0.0)
        .dot_gap(-4.0)
        .resolve(&Theme::default_light());
    assert_eq!(resolved.dot_size, MIN_SIZE);
    assert_eq!(resolved.dot_gap, 0.0);
}

#[test]
fn inactive_stops_animation_clock() {
    let resolved = LoadingDots::new()
        .active(false)
        .animation_speed_ms(10)
        .resolve(&Theme::default_light());
    assert!(!resolved.active);
    assert_eq!(resolved.animation_speed_ms, 0);
}
