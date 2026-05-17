use std::rc::Rc;

/// Formatting strategy for SlideControl value text.
#[derive(Clone)]
pub enum SlideValueFormat {
    Integer,
    Decimal(u8),
    Custom(Rc<dyn Fn(f64) -> String>),
}

fn noop_change(_: f64) {}

/// Properties for `SlideControl`.
#[derive(Clone)]
pub struct SlideControlProps {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub unit: String,
    pub format: SlideValueFormat,
    pub disabled: bool,
    pub readonly: bool,
    pub a11y_label: String,
    pub on_change: Rc<dyn Fn(f64)>,
}

impl Default for SlideControlProps {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            unit: String::new(),
            format: SlideValueFormat::Integer,
            disabled: false,
            readonly: false,
            a11y_label: String::new(),
            on_change: Rc::new(noop_change),
        }
    }
}

/// Builder for the SlideControl.
#[derive(Clone)]
pub struct SlideControl {
    pub(super) props: SlideControlProps,
}

#[derive(Clone)]
pub struct ResolvedSlideControl {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub unit: String,
    pub format: SlideValueFormat,
    pub disabled: bool,
    pub readonly: bool,
    pub a11y_label: String,
    pub on_change: Rc<dyn Fn(f64)>,
}
