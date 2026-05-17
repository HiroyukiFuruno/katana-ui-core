use crate::composite::selector::color_picker::types::ColorPickerValue;
use crate::floem_view::FloemColor;
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::reactive::{RwSignal, SignalGet};
use floem::views::{Decorators, button, h_stack, label};
use std::rc::Rc;

use crate::composite::selector::color_picker::ops;
use crate::composite::selector::color_picker::types::ColorPickerBlendMode;
use crate::composite::selector::color_picker::view::apply_state;

const SLIDER_GAP: f32 = 4.0;
const LABEL_WIDTH: f32 = 78.0;
const BUTTON_BORDER_RADIUS: f32 = 0.0;

pub(crate) struct BlendingControls;

impl BlendingControls {
    pub(crate) fn row(
        state: RwSignal<ColorPickerValue>,
        on_change: Rc<dyn Fn(Color)>,
        locked: bool,
        theme: Theme,
    ) -> impl IntoView {
        h_stack((
            button(label(|| "Normal"))
                .disabled(move || locked)
                .action({
                    let on_change = Rc::clone(&on_change);
                    move || {
                        if locked {
                            return;
                        }
                        let next = ops::ColorPickerOps::set_blend_mode(
                            state.get(),
                            ColorPickerBlendMode::Normal,
                        );
                        apply_state(&state, Rc::clone(&on_change), next);
                    }
                })
                .style(move |style| {
                    let selected = state.get().blending_mode == ColorPickerBlendMode::Normal;
                    let bg = if selected {
                        FloemColor::from_token(theme.color.accent_muted)
                    } else {
                        FloemColor::from_token(theme.color.surface)
                    };
                    style
                        .width(LABEL_WIDTH)
                        .border_radius(BUTTON_BORDER_RADIUS)
                        .background(bg)
                        .color(FloemColor::from_token(theme.color.text))
                }),
            button(label(|| "Additive"))
                .disabled(move || locked)
                .action({
                    let on_change = Rc::clone(&on_change);
                    move || {
                        if locked {
                            return;
                        }
                        let next = ops::ColorPickerOps::set_blend_mode(
                            state.get(),
                            ColorPickerBlendMode::Additive,
                        );
                        apply_state(&state, Rc::clone(&on_change), next);
                    }
                })
                .style(move |style| {
                    let selected = state.get().blending_mode == ColorPickerBlendMode::Additive;
                    let bg = if selected {
                        FloemColor::from_token(theme.color.accent_muted)
                    } else {
                        FloemColor::from_token(theme.color.surface)
                    };
                    style
                        .width(LABEL_WIDTH)
                        .border_radius(BUTTON_BORDER_RADIUS)
                        .background(bg)
                        .color(FloemColor::from_token(theme.color.text))
                }),
        ))
        .style(move |style| style.items_center().gap(SLIDER_GAP))
    }
}
