mod types;
mod view;

pub use types::{ResolvedSlideControl, SlideControl, SlideControlProps, SlideValueFormat};

use crate::theme::Theme;
use floem::IntoView;

const DEFAULT_STEP: f64 = 1.0;

fn normalize_range(min: f64, max: f64) -> (f64, f64) {
    let min = if min.is_finite() { min } else { 0.0 };
    let max = if max.is_finite() { max } else { 0.0 };
    if min <= max {
        (min, max)
    } else {
        (max.min(min), max.max(min))
    }
}

fn normalize_step(step: f64) -> f64 {
    if step.is_finite() && step > 0.0 {
        step
    } else {
        DEFAULT_STEP
    }
}

fn normalize_value(value: f64, min: f64, max: f64, step: f64) -> f64 {
    let min = if min.is_finite() { min } else { 0.0 };
    let max = if max.is_finite() { max } else { 0.0 };
    let normalized = value.clamp(min.min(max), max.max(min));
    let step = normalize_step(step);
    let snapped = ((normalized - min) / step).round() * step + min;
    snapped.clamp(min, max)
}

impl SlideControl {
    #[must_use]
    pub fn new(a11y_label: impl Into<String>) -> Self {
        Self {
            props: SlideControlProps {
                a11y_label: a11y_label.into(),
                ..SlideControlProps::default()
            },
        }
    }

    #[must_use]
    pub fn value(mut self, value: f64) -> Self {
        self.props.value = value;
        self
    }

    #[must_use]
    pub fn min(mut self, min: f64) -> Self {
        self.props.min = min;
        self
    }

    #[must_use]
    pub fn max(mut self, max: f64) -> Self {
        self.props.max = max;
        self
    }

    #[must_use]
    pub fn step(mut self, step: f64) -> Self {
        self.props.step = step;
        self
    }

    #[must_use]
    pub fn unit(mut self, unit: impl Into<String>) -> Self {
        self.props.unit = unit.into();
        self
    }

    #[must_use]
    pub fn format(mut self, format: SlideValueFormat) -> Self {
        self.props.format = format;
        self
    }

    #[must_use]
    pub fn integer(mut self) -> Self {
        self.props.format = SlideValueFormat::Integer;
        self
    }

    #[must_use]
    pub fn decimal(mut self, precision: u8) -> Self {
        self.props.format = SlideValueFormat::Decimal(precision);
        self
    }

    #[must_use]
    pub fn custom_format(mut self, format: impl Fn(f64) -> String + 'static) -> Self {
        self.props.format = SlideValueFormat::Custom(std::rc::Rc::new(format));
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    #[must_use]
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.props.readonly = readonly;
        self
    }

    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(f64) + 'static) -> Self {
        self.props.on_change = std::rc::Rc::new(on_change);
        self
    }

    #[must_use]
    pub fn resolve(&self, _theme: &Theme) -> ResolvedSlideControl {
        let (min, max) = normalize_range(self.props.min, self.props.max);
        let step = normalize_step(self.props.step);
        let value = normalize_value(self.props.value, min, max, step);
        ResolvedSlideControl {
            value,
            min,
            max,
            step,
            unit: self.props.unit.clone(),
            format: self.props.format.clone(),
            disabled: self.props.disabled,
            readonly: self.props.readonly,
            a11y_label: self.props.a11y_label.clone(),
            on_change: std::rc::Rc::clone(&self.props.on_change),
        }
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        view::view(self, theme)
    }
}
