use super::{ProgressBar, view::percent_label};
use crate::theme::Theme;

#[test]
fn normalize_clamps_out_of_range_values_to_zero_to_one() {
    let theme = Theme::default_light();
    let resolved = ProgressBar::new()
        .min(20.0)
        .max(120.0)
        .value(-80.0)
        .resolve(&theme);
    assert_eq!(resolved.progress, 0.0);

    let resolved = ProgressBar::new()
        .min(20.0)
        .max(120.0)
        .value(200.0)
        .resolve(&theme);
    assert_eq!(resolved.progress, 1.0);
}

#[test]
fn reverse_min_max_is_supported_safely() {
    let theme = Theme::default_light();
    let resolved = ProgressBar::new()
        .min(100.0)
        .max(0.0)
        .value(50.0)
        .resolve(&theme);
    assert_eq!(resolved.progress, 0.5);
}

#[test]
fn custom_label_is_kept() {
    let theme = Theme::default_light();
    let resolved = ProgressBar::new()
        .label("task")
        .show_label(true)
        .resolve(&theme);
    assert_eq!(resolved.label_text, "task");
    assert_eq!(resolved.progress, 0.0);
}

#[test]
fn percent_label_is_100_when_full() {
    let theme = Theme::default_light();
    let resolved = ProgressBar::new()
        .min(0.0)
        .max(100.0)
        .value(100.0)
        .resolve(&theme);
    assert_eq!(percent_label(resolved.progress), "100%");
}
