use crate::composite::selector::color_picker::types::ColorPickerValue;
use crate::floem_view::FloemColor;
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::reactive::RwSignal;
use floem::views::{Decorators, h_stack, label};
use std::rc::Rc;

use super::slider::{EguiColorSlider, EguiColorSliderKind};

const SLIDER_GAP: f32 = 4.0;
const LABEL_SLIDER_WIDTH: f32 = 36.0;
const SLIDER_HEIGHT: f32 = 30.0;
const LABEL_TEXT_SIZE: f32 = 12.0;
const SLIDER_BASE_WIDTH: f32 = 480.0;

pub(crate) struct HueAlphaControls;

impl HueAlphaControls {
    pub(crate) fn hue_slider(
        state: RwSignal<ColorPickerValue>,
        on_change: Rc<dyn Fn(Color)>,
        locked: bool,
        theme: Theme,
        _allows_alpha: bool,
        panel_scale: f32,
    ) -> impl IntoView {
        let label_color = FloemColor::from_token(theme.color.text);

        h_stack((
            label(|| "Hue").style(move |style| {
                style
                    .width(LABEL_SLIDER_WIDTH)
                    .color(label_color)
                    .font_size(LABEL_TEXT_SIZE)
            }),
            EguiColorSlider::new(
                state,
                Rc::clone(&on_change),
                EguiColorSliderKind::Hue,
                locked,
                theme.color.border,
            )
            .style(move |style| {
                style
                    .width(SLIDER_BASE_WIDTH * panel_scale)
                    .height(SLIDER_HEIGHT * panel_scale)
            }),
        ))
        .style(|style| style.items_center().gap(SLIDER_GAP))
    }

    pub(crate) fn alpha_slider(
        state: RwSignal<ColorPickerValue>,
        on_change: Rc<dyn Fn(Color)>,
        locked: bool,
        theme: Theme,
        allows_alpha: bool,
        panel_scale: f32,
    ) -> impl IntoView {
        let label_color = FloemColor::from_token(theme.color.text);

        h_stack((
            label(|| "Alpha").style(move |style| {
                style
                    .width(LABEL_SLIDER_WIDTH)
                    .font_size(LABEL_TEXT_SIZE)
                    .color(label_color)
            }),
            EguiColorSlider::new(
                state,
                Rc::clone(&on_change),
                EguiColorSliderKind::Alpha,
                locked || !allows_alpha,
                theme.color.border,
            )
            .style(move |style| {
                style
                    .width(SLIDER_BASE_WIDTH * panel_scale)
                    .height(SLIDER_HEIGHT * panel_scale)
            }),
        ))
        .style(|style| style.items_center().gap(SLIDER_GAP))
    }
}
