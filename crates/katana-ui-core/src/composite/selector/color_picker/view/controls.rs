mod blending;
mod channel;
mod hue_alpha;
mod slider;

use crate::composite::selector::color_picker::types::ColorPickerValue;
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::reactive::RwSignal;
use std::rc::Rc;

use self::blending::BlendingControls;
use self::channel::ChannelControls;
use self::hue_alpha::HueAlphaControls;

pub(crate) struct ColorPickerControls;

impl ColorPickerControls {
    pub(crate) fn blending_row(
        state: RwSignal<ColorPickerValue>,
        on_change: Rc<dyn Fn(Color)>,
        locked: bool,
        theme: Theme,
    ) -> impl IntoView {
        BlendingControls::row(state, on_change, locked, theme)
    }

    pub(crate) fn channel_row(
        state: RwSignal<ColorPickerValue>,
        on_change: Rc<dyn Fn(Color)>,
        locked: bool,
        allows_alpha: bool,
        theme: Theme,
    ) -> impl IntoView {
        ChannelControls::row(state, on_change, locked, allows_alpha, theme)
    }

    pub(crate) fn hue_slider(
        state: RwSignal<ColorPickerValue>,
        on_change: Rc<dyn Fn(Color)>,
        locked: bool,
        theme: Theme,
        allows_alpha: bool,
        panel_scale: f32,
    ) -> impl IntoView {
        HueAlphaControls::hue_slider(state, on_change, locked, theme, allows_alpha, panel_scale)
    }

    pub(crate) fn alpha_slider(
        state: RwSignal<ColorPickerValue>,
        on_change: Rc<dyn Fn(Color)>,
        locked: bool,
        theme: Theme,
        allows_alpha: bool,
        panel_scale: f32,
    ) -> impl IntoView {
        HueAlphaControls::alpha_slider(state, on_change, locked, theme, allows_alpha, panel_scale)
    }
}
