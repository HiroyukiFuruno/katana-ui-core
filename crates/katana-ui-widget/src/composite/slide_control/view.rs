use super::{SlideControl, SlideValueFormat};
use crate::floem_view::FloemColor;
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::peniko::Brush;
use floem::reactive::{SignalGet, SignalUpdate, create_effect, create_rw_signal};
use floem::unit::Pct;
use floem::views::slider::slider as floem_slider;
use floem::views::{Decorators, h_stack, label, text_input as floem_text_input, v_stack};
use std::cell::Cell;
use std::rc::Rc;

const INPUT_WIDTH: f32 = 94.0;
const UNIT_GAP: f32 = 6.0;
const SLIDER_WIDTH: f32 = 240.0;
const EPSILON: f64 = 1e-9;
const A11Y_FONT_SIZE: f32 = 11.0;
const LABEL_SCALE: f64 = 100.0;
const MIN_NORMALIZED_PERCENT: f64 = 0.0;
const MAX_NORMALIZED_PERCENT: f64 = 100.0;
const STACK_GAP: f64 = 6.0;
const SLIDER_CONTROL_HEIGHT: f32 = 22.0;
const SLIDER_BAR_HEIGHT: f32 = 6.0;
const SLIDER_HANDLE_RADIUS: f32 = 7.0;
const SLIDER_TRACK_RADIUS_PERCENT: f64 = 100.0;

#[derive(Debug, Clone, Copy, PartialEq)]
struct SliderVisualTokens {
    bar: Color,
    accent_bar: Color,
    handle: Color,
}

fn normalized(value: f64, min: f64, max: f64, step: f64) -> f64 {
    let min = if min.is_finite() {
        min
    } else {
        MIN_NORMALIZED_PERCENT
    };
    let max = if max.is_finite() {
        max
    } else {
        MIN_NORMALIZED_PERCENT
    };
    if !value.is_finite() {
        return min;
    }

    let clamped = value.clamp(min.min(max), max.max(min));
    if !step.is_finite() || step <= 0.0 {
        return clamped;
    }

    (((clamped - min) / step).round() * step + min).clamp(min.min(max), max.max(min))
}

fn percent_for_value(value: f64, min: f64, max: f64) -> f64 {
    let span = (max - min).abs();
    if span <= EPSILON {
        MIN_NORMALIZED_PERCENT
    } else {
        ((value - min) / (max - min) * LABEL_SCALE)
            .clamp(MIN_NORMALIZED_PERCENT, MAX_NORMALIZED_PERCENT)
    }
}

fn value_for_percent(percent: f64, min: f64, max: f64, step: f64) -> f64 {
    if (max - min).abs() <= EPSILON {
        return min;
    }
    let span = max - min;
    normalized(
        min + (percent / MAX_NORMALIZED_PERCENT) * span,
        min,
        max,
        step,
    )
}

fn parse_input(value: &str) -> Option<f64> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.parse::<f64>().ok()
}

fn format_value(format: &SlideValueFormat, value: f64) -> String {
    match format {
        SlideValueFormat::Integer => format!("{:.0}", value),
        SlideValueFormat::Decimal(precision) => {
            let digits = usize::from(*precision);
            format!("{:.*}", digits, value)
        }
        SlideValueFormat::Custom(formatter) => formatter(value),
    }
}

fn slider_visual_tokens(disabled: bool, theme: &Theme) -> SliderVisualTokens {
    if disabled {
        SliderVisualTokens {
            bar: theme.color.border,
            accent_bar: theme.color.text_disabled,
            handle: theme.color.text_disabled,
        }
    } else {
        SliderVisualTokens {
            bar: theme.color.accent_muted,
            accent_bar: theme.color.accent,
            handle: theme.color.text_muted,
        }
    }
}

pub(super) fn view(control: SlideControl, theme: Theme) -> impl IntoView {
    let resolved = control.resolve(&theme);
    let disabled = resolved.disabled || resolved.readonly;
    let min = resolved.min;
    let max = resolved.max;
    let step = resolved.step;
    let format = resolved.format.clone();

    let value = create_rw_signal(normalized(resolved.value, min, max, step));
    let value_text = create_rw_signal(format_value(&resolved.format, value.get()));
    let on_change = Rc::clone(&resolved.on_change);

    let syncing = Rc::new(Cell::new(false));

    create_effect({
        let on_change = Rc::clone(&on_change);
        let mounted = Rc::new(Cell::new(false));
        move |_| {
            let current = value.get();
            if mounted.replace(true) {
                on_change(current);
            }
        }
    });

    create_effect({
        let format = format.clone();
        let syncing = Rc::clone(&syncing);
        move |_| {
            syncing.set(true);
            let next = format_value(&format, value.get());
            if value_text.try_get_untracked().unwrap_or_default() != next {
                value_text.set(next);
            }
            syncing.set(false);
        }
    });

    create_effect({
        let syncing = Rc::clone(&syncing);
        move |_| {
            if syncing.get() {
                return;
            }
            let raw = value_text.try_get().unwrap_or_default();
            if let Some(parsed) = parse_input(&raw) {
                let next = normalized(parsed, min, max, step);
                if (next - value.try_get_untracked().unwrap_or_default()).abs() > EPSILON {
                    value.set(next);
                }
            }
        }
    });

    let unit = resolved.unit;
    let slider_tokens = slider_visual_tokens(disabled, &theme);
    let slider_bar = FloemColor::from_token(slider_tokens.bar);
    let slider_accent_bar = FloemColor::from_token(slider_tokens.accent_bar);
    let slider_handle = FloemColor::from_token(slider_tokens.handle);

    v_stack((
        label(move || resolved.a11y_label.clone()).style(|style| style.font_size(A11Y_FONT_SIZE)),
        h_stack((
            floem_slider(move || {
                Pct(percent_for_value(
                    value.try_get_untracked().unwrap_or(0.0),
                    min,
                    max,
                ))
            })
            .on_change_pct({
                move |percent| value.set(value_for_percent(percent.0, min, max, step))
            })
            .disabled(move || disabled)
            .slider_style(move |style| {
                style
                    .bar_color(slider_bar)
                    .bar_radius(Pct(SLIDER_TRACK_RADIUS_PERCENT))
                    .bar_height(SLIDER_BAR_HEIGHT)
                    .accent_bar_color(slider_accent_bar)
                    .accent_bar_radius(Pct(SLIDER_TRACK_RADIUS_PERCENT))
                    .accent_bar_height(SLIDER_BAR_HEIGHT)
                    .handle_color(Brush::Solid(slider_handle))
                    .handle_radius(SLIDER_HANDLE_RADIUS)
            })
            .style(move |style| style.width(SLIDER_WIDTH).height(SLIDER_CONTROL_HEIGHT)),
            floem_text_input(value_text)
                .disabled(move || disabled)
                .style(move |style| style.width(INPUT_WIDTH)),
            label(move || unit.clone()).style(move |style| style.margin_left(UNIT_GAP)),
        ))
        .style(move |style| style.gap(UNIT_GAP).items_center()),
    ))
    .style(|style| style.gap(STACK_GAP))
}

#[cfg(test)]
mod tests {
    use super::slider_visual_tokens;
    use crate::theme::Theme;

    #[test]
    fn slider_visual_tokens_use_theme_colors() {
        let theme = Theme::default_light();
        let visual_tokens = slider_visual_tokens(false, &theme);

        assert_eq!(visual_tokens.bar, theme.color.accent_muted);
        assert_eq!(visual_tokens.accent_bar, theme.color.accent);
        assert_eq!(visual_tokens.handle, theme.color.text_muted);
    }

    #[test]
    fn disabled_slider_visual_tokens_are_deemphasized() {
        let theme = Theme::default_light();
        let visual_tokens = slider_visual_tokens(true, &theme);

        assert_eq!(visual_tokens.bar, theme.color.border);
        assert_eq!(visual_tokens.accent_bar, theme.color.text_disabled);
        assert_eq!(visual_tokens.handle, theme.color.text_disabled);
    }
}
