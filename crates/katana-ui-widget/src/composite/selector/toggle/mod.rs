mod types;
mod view;

pub use types::{ToggleProps, ToggleSize};

use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, container, dyn_container, h_stack, label};
use std::rc::Rc;
use view::{thumb_color, thumb_offset_off, thumb_offset_on, thumb_size, track_color, track_dims};

const TOGGLE_GAP: f32 = crate::floem_view::GAP_SM;
const TOGGLE_PADDING: f32 = crate::floem_view::GAP_XS;

fn noop_change(_: bool) {}

/// Resolved visual properties for `Toggle`.
#[derive(Clone)]
pub struct ResolvedToggle {
    pub track_width: f32,
    pub track_height: f32,
    pub track_color: Color,
    pub thumb_size: f32,
    pub thumb_offset: f32,
    pub thumb_color: Color,
    pub disabled: bool,
    pub value: bool,
    pub a11y_label: String,
    pub on_change: Rc<dyn Fn(bool)>,
}

impl ResolvedToggle {
    /// Toggle the value and notify the caller when enabled.
    pub fn toggle(&self) -> Option<bool> {
        if self.disabled {
            return None;
        }

        let next = !self.value;
        (self.on_change)(next);
        Some(next)
    }
}

/// Builder for the Toggle composite widget.
#[derive(Clone)]
pub struct Toggle {
    props: ToggleProps,
}

impl Toggle {
    #[must_use]
    pub fn new(a11y_label: impl Into<String>) -> Self {
        Self {
            props: ToggleProps {
                value: false,
                size: ToggleSize::default(),
                disabled: false,
                a11y_label: a11y_label.into(),
                on_change: Rc::new(noop_change),
            },
        }
    }

    #[must_use]
    pub fn value(mut self, value: bool) -> Self {
        self.props.value = value;
        self
    }

    #[must_use]
    pub fn size(mut self, size: ToggleSize) -> Self {
        self.props.size = size;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(bool) + 'static) -> Self {
        self.props.on_change = Rc::new(on_change);
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedToggle {
        let dims = track_dims(self.props.size);
        let thumb_sz = thumb_size(&dims);
        let thumb_off = if self.props.value {
            thumb_offset_on(&dims)
        } else {
            thumb_offset_off()
        };

        ResolvedToggle {
            track_width: dims.width,
            track_height: dims.height,
            track_color: track_color(self.props.value, self.props.disabled, theme),
            thumb_size: thumb_sz,
            thumb_offset: thumb_off,
            thumb_color: thumb_color(self.props.disabled, theme),
            disabled: self.props.disabled,
            value: self.props.value,
            a11y_label: self.props.a11y_label.clone(),
            on_change: Rc::clone(&self.props.on_change),
        }
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let value = create_rw_signal(self.props.value);
        let label_text = self.props.a11y_label.clone();
        let props = self.props.clone();
        let props_for_signal = props.clone();

        h_stack((
            label(move || label_text.clone()),
            dyn_container(
                move || value.try_get().unwrap_or(props_for_signal.value),
                move |current| {
                    let mut resolved_props = props.clone();
                    resolved_props.value = current;
                    let resolved = Toggle {
                        props: resolved_props,
                    }
                    .resolve(&theme);
                    let track_color =
                        crate::floem_view::FloemColor::from_token(resolved.track_color);
                    let thumb_color =
                        crate::floem_view::FloemColor::from_token(resolved.thumb_color);
                    let track_height = resolved.track_height;
                    let thumb_size = resolved.thumb_size;
                    let thumb_offset = resolved.thumb_offset;
                    let disabled = resolved.disabled;
                    let on_change = Rc::clone(&resolved.on_change);

                    container(container(label(|| "")).style(move |style| {
                        style
                            .width(thumb_size)
                            .height(thumb_size)
                            .margin_left(thumb_offset)
                            .background(thumb_color)
                            .border_radius(thumb_size / 2.0)
                    }))
                    .on_click_stop(move |_| {
                        if disabled {
                            return;
                        }

                        let next = !current;
                        value.set(next);
                        on_change(next);
                    })
                    .style(move |style| {
                        style
                            .width(resolved.track_width)
                            .height(resolved.track_height)
                            .padding(2.0)
                            .background(track_color)
                            .border(1.0)
                            .border_radius(track_height / 2.0)
                    })
                },
            ),
        ))
        .style(move |style| style.gap(TOGGLE_GAP).items_center().padding(TOGGLE_PADDING))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn off_thumb_at_leading_edge() {
        let theme = Theme::default_light();
        let r = Toggle::new("Test").value(false).resolve(&theme);
        assert!(r.thumb_offset < r.track_width / 2.0);
    }

    #[test]
    fn on_thumb_at_trailing_edge() {
        let theme = Theme::default_light();
        let r = Toggle::new("Test").value(true).resolve(&theme);
        assert!(r.thumb_offset >= r.track_width / 2.0);
    }

    #[test]
    fn on_track_color_is_accent() {
        let theme = Theme::default_light();
        let r = Toggle::new("Test").value(true).resolve(&theme);
        assert_eq!(r.track_color, theme.color.accent);
    }

    #[test]
    fn disabled_track_color_is_border() {
        let theme = Theme::default_light();
        let r = Toggle::new("Test").disabled(true).resolve(&theme);
        assert_eq!(r.track_color, theme.color.border);
    }

    #[test]
    fn a11y_label_preserved() {
        let theme = Theme::default_light();
        let r = Toggle::new("Dark mode").resolve(&theme);
        assert_eq!(r.a11y_label, "Dark mode");
    }

    #[test]
    fn toggle_calls_on_change_with_next_value() {
        let called = std::rc::Rc::new(std::cell::RefCell::new(None));
        let called_ref = std::rc::Rc::clone(&called);
        let theme = Theme::default_light();
        let r = Toggle::new("Test")
            .value(false)
            .on_change(move |value| {
                *called_ref.borrow_mut() = Some(value);
            })
            .resolve(&theme);

        assert_eq!(r.toggle(), Some(true));
        assert_eq!(*called.borrow(), Some(true));
    }

    #[test]
    fn disabled_toggle_does_not_call_on_change() {
        let called = std::rc::Rc::new(std::cell::RefCell::new(false));
        let called_ref = std::rc::Rc::clone(&called);
        let theme = Theme::default_light();
        let r = Toggle::new("Test")
            .disabled(true)
            .on_change(move |_| {
                *called_ref.borrow_mut() = true;
            })
            .resolve(&theme);

        assert_eq!(r.toggle(), None);
        assert!(!*called.borrow());
    }
}
